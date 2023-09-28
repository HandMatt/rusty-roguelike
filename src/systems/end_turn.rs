use crate::prelude::*;

#[system]
/// End the current turn and cycle on to the next TurnState.
/// * `turn_state` - writeable access to the TurnState resource
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        // if the game is awaiting input exit the function
        TurnState::AwaitingInput => return,
        // if its the players turn the next phase will be the monsters' turn
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        // if the monsters' turn is ending go back to waiting for input
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };
    // set the turn resource to the chosen value
    *turn_state = new_state;
}
