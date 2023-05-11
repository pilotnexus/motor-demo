use pilot_sys::var::{Var, VarProps};
use smlang::statemachine;

use crate::pilot::variables::PlcVars;

// This section defines the state machine using a DSL based on Boost-SML (https://boost-ext.github.io/sml/)
// The DSL is defined as follows:
// ```
// statemachine!{
//    transitions: {
//        *SrcState1 + Event1 [ guard1 ] / action1 = DstState2, // * denotes starting state
//        SrcState2 + Event2 [ guard2 ] / action2 = DstState1,
//    }
//    // ...
//}
// ```
statemachine! {
    transitions: {
        *InitialState + InitializedEvent  = IdleState,
        IdleState + StartMoveEvent = MovingState,
        MovingState + MoveCompleteEvent = IdleState,
        MovingState + MoveErrorEvent = ErrorState,
        //ErrorState + ResetEvent = IdleState, //-> TODO: this should be implemented to recover from the ErrorState
    }
}

pub struct Context {}

impl<'a> StateMachineContext for Context {}

///Wraps the StateMachine and adds a PLC variable that is updated with the current state
/// States are mapped to integers in state_to_u32. This needs to be updated when more States are
/// added to the state machine (it will throw a compilation error when the match is non-exhaustive).
pub struct PilotStateMachine<'a> {
    sm: StateMachine<Context>,
    state: &'a Var<u32>,
}

impl<'a> PilotStateMachine<'a> {
    pub fn new(state: &'a Var<u32>) -> Self {
        Self {
            sm: StateMachine::new(Context {}),
            state,
        }
    }

    // update to match all states and the values
    // that should be mapped to the states PLC variable
    fn u32_state(&self) -> u32 {
        match self.sm.state() {
            States::InitialState => 0,
            States::IdleState => 1,
            States::MovingState => 2,
            States::ErrorState => 3,
        }
    }

    pub fn state(&self) -> &States {
        self.sm.state()
    }

    pub fn process_event(&mut self, event: Events) -> Result<(), Error> {
        match self.sm.process_event(event) {
            Ok(_) => {
                self.state.set(self.u32_state());
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
}
