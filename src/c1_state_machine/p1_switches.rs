use super::StateMachine;

// This state machine is used to simulate a switch that is either on or off.
pub struct LightSwitch;

impl StateMachine for LightSwitch {
    type State = bool;
    type Transition = ();

    fn next_state(starting_state: &bool, transition: &()) -> bool {
        !*starting_state
    }
}