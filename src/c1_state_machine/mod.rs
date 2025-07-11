mod p1_switches;

pub trait StateMachine {
    type State;
    type Transition;
    fn next_state(starting_state: &Self::State, transition: &Self::Transition) -> Self::State;
}