#![macro_use]

use pilot_sys::var::{ Var, VarProps, PilotBindings };
use pilot_macro::*;
use crate::{pilot::bindings::PilotAccess, axis::motoraxis::MotorAxisVariables};

#[derive(ConstNew, PilotAccess, PilotBindings)]
pub struct PlcVars {

  // this binds motor 0 to the lateral_axis variable using the MotorAxisVariables struct 
  #[bind_nested(m3[0])]
  pub lateral_axis: MotorAxisVariables,

  // this binds motor 1 to the longitudinal_axis variable using the MotorAxisVariables struct 
  #[bind_nested(m3[1])]
  pub longitudinal_axis: MotorAxisVariables,

  //represents the current state of the state machine (see src/statemachine.rs)
  #[bind_ignore]
  pub state: Var<u32>,

  //starts the machine, for demo purposes only
  #[bind_ignore]
  pub start_machine: Var<bool>,

  //starts the motor movements, for demo purposes only
  #[bind_ignore]
  pub start_demo: Var<bool>,
}

