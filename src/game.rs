use crate::{board::Board, player::Player};

pub(crate) struct Game {
    board: Board,
    players: Vec<Player>,
}