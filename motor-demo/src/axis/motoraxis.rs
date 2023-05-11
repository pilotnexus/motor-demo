use core::time::Duration;

use bitflags::bitflags;
use pilot_macro::{ConstNew, PilotAccess, PilotBindings};
use pilot_sys::{
    loop_async,
    time::{self, wait_next_cycle},
    var::{Var, VarProps},
};

use crate::{plugins::motor_control::{dc_get_flag, dc_move, dc_reset, dc_set_speed, dc_set_target_position}, helper};

const MIN_MOTOR_SPEED: u8 = 30;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct MotorErrorFlags: u8 {
        const CONTROLLER_ERROR = 0b00000001;
        const SPEED_TO_LOW =     0b00000010;
        const ENCODER_TIMEOUT =  0b00000100;
        const DURATION_TIMEOUT = 0b00001000;
        const ERROR = Self::CONTROLLER_ERROR.bits() | Self::SPEED_TO_LOW.bits() | Self::ENCODER_TIMEOUT.bits() | Self::DURATION_TIMEOUT.bits();
    }
}

#[derive(ConstNew, PilotAccess, PilotBindings)]
#[bind_type(crate::pilot::bindings::Motor)]
pub struct MotorAxisVariables {
    ///see https://github.com/DAmesberger/dcmctrl
    ///Bit 0 ...... Got OTW condition
    ///Bit 1 ...... Got FAULT condition
    ///Bit 2 ...... Got timeout
    ///Bit 3 ...... Reserved
    ///Bit 4 ...... Reserved
    ///Bit 5 ...... Reserved
    ///Bit 6 ...... Reserved
    ///Bit 7 ...... Reserved
    #[bind_read(flags)]
    pub controller_fault_flags: Var<u8>,

    /// Our fault flags defined in MotorErrorFlags
    #[bind_ignore]
    pub motor_error_flags: Var<u8>,

    ///read position flag from the FPGA motor controller
    #[bind_read(position)]
    pub position: Var<u32>,
}

pub struct MotorAxis<'a> {
    motor_number: u8,
    plc_vars: &'a MotorAxisVariables,
}

///demo implementation of a motor axis
///currently it only moves to positions
///using move_axis_to_position()
impl<'a> MotorAxis<'a> {
    pub fn new(motor_number: u8, plc_vars: &'a MotorAxisVariables) -> Self {
        Self {
            motor_number,
            plc_vars,
        }
    }

    pub async fn move_axis_to_position(
        &self,
        target_pos: u32,
        timeout: Duration,
    ) -> Result<(), MotorErrorFlags> {
        //reading the speed set from the PLC variable
        let speed = 150; //we use a fixed speed of 150 in this example

        if speed < MIN_MOTOR_SPEED {
            self.plc_vars.motor_error_flags.set(MotorErrorFlags::SPEED_TO_LOW.bits());
            Err(MotorErrorFlags::SPEED_TO_LOW)
        } else {
            //reset errors
            self.plc_vars.motor_error_flags.set(0);
            //start motor
            dc_move(self.motor_number, speed, target_pos).await;

            //loop as long as the controller does not have a fault state
            //we use a helper function loop_until that takes a maximum duration it will call
            //the run closure. It then returns with Err(()) if the timeout is exceeded, or Ok(T) if
            //the closure returned Some(T) (to indicate that the loop is done)
            let result = helper::loop_until(timeout, &|| {
                if self.plc_vars.controller_fault_flags.get() != 0 {
                    Some(Some(MotorErrorFlags::CONTROLLER_ERROR)) //we have an error, end the loop_until early
                } else if self.plc_vars.position.get() == target_pos {
                    Some(None) //we have reached the target position, no errors occured
                } else {
                    None //we are still moving and have no timeout
                }
            }).await;

            //for better clarity we transform the result in a second step, unifying the MotorErrorFlags
            let result = match result {
                Ok(motor_error) => match motor_error {
                    Some(error_flags) => Err(error_flags),
                    None => Ok(()), //no duration timeout and no errors from the controller
                },
                Err(_) => {
                    dc_set_speed(self.motor_number, 0).await; //we stop the motor right now
                    wait_next_cycle().await; //wait for the next PLC cycle
                    Err(MotorErrorFlags::DURATION_TIMEOUT)
                }, //the motor took too long to reach the target position
            };

            //lets set the error flags if errors occured
            if let Err(motor_error) = result {
                self.plc_vars.motor_error_flags.set(motor_error.bits());
            }

            result
        }
    }
}
