/// A token on the time track.
#[derive(Copy, Clone, Debug)]
enum Token {
    Time,
    Button,
    Patch,
}

type Position = usize;
type Pointer = (Position, usize);

/// A track for managing time remaining in the game.
///
/// Rules:
/// * Player 1 goes first.
/// * There are 54 tiles on the time track.
#[derive(Debug)]
pub struct TimeTrack {
    pointers: Vec<Pointer>,
    track: Vec<Option<Vec<Token>>>,
}

impl TimeTrack {
    pub fn for_players(num_players: usize) -> Self {
        let mut track = vec![None; 54];
        track[0] = Some(vec![Token::Time; num_players]);
        for position in [5, 11, 17, 23, 29, 35, 41, 47, 53] {
            track[position] = Some(vec![Token::Button])
        }
        for position in [20, 26, 32, 44, 50] {
            track[position] = Some(vec![Token::Patch])
        }
        Self {
            track,
            pointers: (0..num_players).map(|i| (0, i)).collect(),
        }
    }
    /// Places, and returns a pointer, to a token at a position on the track.
    fn place(&mut self, token: Token, position: Position) -> Pointer {
        match &mut self.track[position] {
            Some(tokens) => tokens.push(token),
            None => self.track[position] = Some(vec![token]),
        }
        (position, self.track.len() - 1)
    }
}
