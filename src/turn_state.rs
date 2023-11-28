#[derive(Copy, Clone, Debug, PartialEq)]
/// Represents one of three turn states
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}
