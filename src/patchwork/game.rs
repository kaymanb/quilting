use geo::Point;

use crate::patchwork::{Board, Patch, PatchCircle, TimeTrack};

#[derive(Clone, Debug, Default)]
struct Player {
    board: Board,
    buttons: u8,
}

/// A game of Patchwork.
///
/// Rules:
/// * Player 1 goes first.
#[derive(Debug)]
pub struct Game {
    current_player: usize,
    patch_circle: PatchCircle,
    players: Vec<Player>,
    time_track: TimeTrack,
}

impl Game {
    pub fn for_players(num_players: usize) -> Self {
        Self {
            current_player: 0,
            patch_circle: PatchCircle::default(),
            players: vec![Player::default(); num_players],
            time_track: TimeTrack::for_players(num_players),
        }
    }

    /// Progress the game by a single turn.
    pub fn turn(&self) {}

    /// Picks and returns a patch from the circle that the player can place on
    /// their board.
    fn pick_patch(&mut self, player: usize) -> Option<(Point, Patch)> {
        let choices = self.patch_circle.next(3);

        // Just grab the first valid choice for now.
        let choice: Option<(Point, Patch)> = None;
        for (index, patch) in choices.iter() {
            if let Some((point, rotation)) = self.players[player].board.fit(patch) {
                break;
            }
        }
        choice
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::for_players(2)
    }
}
