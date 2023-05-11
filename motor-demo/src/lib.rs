#![allow(unused_imports)]
#![cfg_attr(not(test), no_std)]
extern crate pilot_sys;
use axis::motoraxis::MotorAxisVariables;
use pilot_sys::async_util::wait_or;
use pilot_sys::loop_async;
use pilot_sys::time::{wait, wait_next_cycle};
use pilot_sys::var::{NumVar, SubscribeMode, Var, VarChange, VarProps};
mod pilot;
use core::time::Duration;
use pilot::variables::PlcVars;
use pilot_sys::futures::join;
use pilot_sys::*;
use statemachine::{Context, StateMachine};

use bitflags::bitflags;

use crate::axis::motoraxis;
use crate::helper::host_set;
use crate::statemachine::{Events, PilotStateMachine, States};

mod axis;
mod helper;
mod plugins;
mod statemachine;

pub async fn main_task(v: &PlcVars) {
    //see src/statemachine.rs for the definition of the state machine
    let mut sm = PilotStateMachine::new(&v.state);

    subscribe_variables(&v);

    //create the state machine according to the SM defined in src/statemachine.rs
    //we pass the PlcVars to the statemachine context so we can access the variables in the statemachine

    //initialize motoraxis struct for motor 0 (lateral axis)
    //make sure that the motor_number is the same as in the v.lateral_axis variable definition in variables.rs
    //motor_number: 0 AND  #[bind_nested(m3[0])]
    //              ^                       ^
    //               \      identical      /
    let lateral_axis = motoraxis::MotorAxis::new(0, &v.lateral_axis);

    //initialize motoraxis struct for motor 1 (longitudinal axis)
    let longitudinal_axis = motoraxis::MotorAxis::new(1, &v.longitudinal_axis);

    //in this demo, the machine will remain in the InitalState until the start_machine variable is set to true
    while *sm.state() != States::IdleState {
        host_set(&v.start_machine).await; //we wait for the button press

        if let Err(_) = sm.process_event(Events::InitializedEvent) {
            //this only happens if there is a guard condition that is not met
            //or the event transition is not possible
            //(see https://github.com/korken89/smlang-rs#using-guards-and-actions)
            //in this demo there is no guard and the event InitializedEvent is possible
            //from the initial state.
        }
    }

    //we will only get to this point if we are in IdleState
    let lateral_target_positions: [u32; 4] = [100, 50, 200, 0];
    let longitudinal_target_positions: [u32; 4] = [50, 10, 200, 0];
    let timeout = Duration::from_secs(10);

    //do this forever
    loop_async! {{

        host_set(&v.start_demo).await; //we wait for the button press

        //here are two ways of using if let, the first one matches on Ok, the second one on Err.
        if let Ok(_) = sm.process_event(Events::StartMoveEvent) {

            for idx in 0..core::cmp::min(lateral_target_positions.len(),
                                         longitudinal_target_positions.len()) {

                //we start both tasks and wait for both to finish the movement before we start the next using join!
                let (lateral_result, longitudinal_result) =
                    join!(
                        lateral_axis.move_axis_to_position(lateral_target_positions[idx], timeout),
                        longitudinal_axis.move_axis_to_position(longitudinal_target_positions[idx], timeout));

                //look if there was an error
                match (lateral_result, longitudinal_result) {
                    (Ok(()), Ok(())) => { //no error occured in both motors
                    },
                    _ => { // some error occured
                        if let Err(_) = sm.process_event(Events::MoveErrorEvent) {
                            //TODO: handle process_event error
                        }
                        break;
                    }
                }
            }

            if let Err(_) = sm.process_event(Events::MoveCompleteEvent) {
                //TODO: handle process_event error
            }
        } else {
            //TODO: handle process_event error
        }
    }}
}

fn subscribe_variables(v: &PlcVars) {
    v.start_machine.subscribe(SubscribeMode::Sticky);
    v.start_demo.subscribe(SubscribeMode::Sticky);
    v.state.subscribe(SubscribeMode::Sticky);
    subscribe_axis(&v.lateral_axis);
    subscribe_axis(&v.longitudinal_axis);
}

fn subscribe_axis(axis: &MotorAxisVariables) {
    axis.position.subscribe(SubscribeMode::Sticky);
}
