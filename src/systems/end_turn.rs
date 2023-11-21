use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
/// End the current turn and cycle on to the next TurnState.
/// * `turn_state` - writeable access to the TurnState resource
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        // if the game is awaiting input exit the function
        TurnState::AwaitingInput => return,
        // if its the players turn the next phase will be the monsters' turn
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        // if the monsters' turn is ending go back to waiting for input
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }
    });

    // set the turn resource to the chosen value
    *turn_state = new_state;
}
