#![allow(unused_imports)]
#![cfg_attr(not(test), no_std)]
extern crate pilot_sys;
use axis::motoraxis::MotorAxisVariables;
use pilot_sys::async_util::wait_or;
use pilot_sys::loop_async;
use pilot_sys::time::{wait, wait_next_cycle};
use pilot_sys::var::{NumVar, SubscribeMode, Var, VarChange, VarProps};
mod pilot;
use bitflags::bitflags;
use core::time::Duration;
use futures::future::{self, Either};
use pilot::variables::PlcVars;
use pilot_sys::futures::{join, pin_mut, select_biased, FutureExt};
use pilot_sys::*;
use statemachine::{Context, StateMachine};

use crate::axis::motoraxis;
use crate::helper::host_set;
use crate::statemachine::{Events, PilotStateMachine, States};

mod axis;
mod helper;
mod manual_task;
mod moving_task;
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

    //this is the main loop that starts all the other tasks as required
    loop_async! {{


        let start_demo = host_set(&v.start_demo); //we wait for the button press
        let manual_mode = async {
            v.manual_mode.pos().await;   //or a change of manual_mode to 1
        };
                                                  //
        // 'select' requires Future + Unpin bounds
        pin_mut!(start_demo);
        pin_mut!(manual_mode);

        match future::select(start_demo, manual_mode).await {
            Either::Left(((), _)) => {
                // don't continue polling toggle_outputs
                if let Err(_) = moving_task::run(&mut sm, &lateral_axis, &longitudinal_axis).await {
                    //we are in the error state, wait for reset
                    host_set(&v.reset_error).await;
                    if let Err(_) = sm.process_event(Events::ResetEvent) {
                        //TODO handle move to reset event error
                    }
                }
            }
            Either::Right(((), _manual_mode)) => {
                if let Ok(_) = sm.process_event(Events::ManualMoveEvent) {

                println!("entering manual state");
                // task that waits until manual_mode changes from 1 to 0
                let manual_mode_wait_neg = async {
                    v.manual_mode.neg().await;
                }.fuse();
                let lateral_task = manual_task::run(&lateral_axis).fuse();
                let longitudinal_task = manual_task::run(&longitudinal_axis).fuse();

                pin_mut!(manual_mode_wait_neg);
                pin_mut!(lateral_task);
                pin_mut!(longitudinal_task);

                select_biased! {
                   () = manual_mode_wait_neg => {
                     },
                     () = lateral_task => {
                     },
                     () = longitudinal_task => {
                     },
                  }
                }

                if let Ok(_) = sm.process_event(Events::ExitManualMoveEvent) {
                  println!("leaving manual state");
                } else {
                  println!("error leaving manual state");
                }


            },
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
