use crate::board_representation::game_state::GameMove;
use std::cmp::Ordering;

pub mod alphabeta;
pub mod cache;
pub mod history;
pub mod quiescence;
pub mod reserved_memory;
pub mod searcher;
pub mod statistics;
pub mod timecontrol;

pub const MAX_SEARCH_DEPTH: usize = 100;
pub const MATE_SCORE: i16 = 15000;
pub const MATED_IN_MAX: i16 = -14000;
pub const STANDARD_SCORE: i16 = -32767;

pub fn init_constants() {
    quiescence::PIECE_VALUES.len();
}

#[derive(Clone)]
pub struct GradedMove {
    pub mv_index: usize,
    pub score: f64,
}

impl GradedMove {
    pub fn new(mv_index: usize, score: f64) -> GradedMove {
        GradedMove { mv_index, score }
    }
}

impl Eq for GradedMove {}

impl PartialEq for GradedMove {
    fn eq(&self, other: &GradedMove) -> bool {
        self.score == other.score
    }
}

impl Ord for GradedMove {
    fn cmp(&self, other: &GradedMove) -> Ordering {
        if self.score > other.score {
            return Ordering::Less;
        } else if self.score < other.score {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl PartialOrd for GradedMove {
    fn partial_cmp(&self, other: &GradedMove) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
