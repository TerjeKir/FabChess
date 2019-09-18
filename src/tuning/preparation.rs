extern crate core;

use core::board_representation::game_state::{
    GameMove, GameMoveType, GameResult, GameState, WHITE,
};
use core::board_representation::game_state_attack_container::GameStateAttackContainer;
use core::evaluation::eval_game_state;
use core::move_generation::makemove::make_move;
use core::move_generation::movegen::{self, AdditionalGameStateInformation, MoveList};
use core::search::alphabeta::{
    check_end_condition, check_for_draw, get_next_gm, in_check_slow, leaf_score,
};
use core::search::history::History;
use core::search::quiescence::{
    best_move_value, is_capture, passes_delta_pruning, see, DELTA_PRUNING,
};
use core::search::reserved_memory::{ReservedAttackContainer, ReservedMoveList};
use core::search::GradedMove;
use core::search::{MAX_SEARCH_DEPTH, STANDARD_SCORE};
use core::tuning::loading::load_positions;
use core::tuning::loading::{save_positions, FileFormatSupported, LabelledGameState, Statistics};
use std::fs;

//const FEN_DIR: &str = "D:/FenCollection/Real";
const FEN_DIR: &str = "D:/FenCollection/Lichess";
fn main() {
    //2. Transform all FEN-Positions in Quiet positions
    //3. Save all positions just like loaded, all positions after q-search, all positions after q-search without stripped(no positions with >10 or <-10 eval)
    let mut positions: Vec<LabelledGameState> = Vec::with_capacity(8_000_000);
    let mut stats = Statistics::default();
    let paths = fs::read_dir(FEN_DIR).unwrap();
    for path in paths {
        load_positions(
            &format!("{}", path.unwrap().path().display()),
            FileFormatSupported::EPD,
            &mut positions,
            &mut stats,
        );
    }
    println!("{}", stats);
    println!("Positions: {}", positions.len());
    /*save_positions(
        &format!("{}/all_positions_noqsearch.txt", FEN_DIR),
        &positions,
    );*/

    let mut quiet_nonstripped: Vec<LabelledGameState> = Vec::with_capacity(positions.len());
    let mut quiet_stripped: Vec<LabelledGameState> = Vec::with_capacity(positions.len());

    let mut history = History::default();
    let mut move_list = ReservedMoveList::default();
    let mut attack_container = ReservedAttackContainer::default();
    let mut see_buffer = vec![0i16; MAX_SEARCH_DEPTH];

    for position in positions {
        let mut other = position.game_state.clone();
        other.color_to_move = 1 - other.color_to_move;
        if in_check_slow(&other) {
            continue;
        }
        let (score, state) = stripped_q_search(
            -16000,
            16000,
            position.game_state.clone(),
            if position.game_state.color_to_move == WHITE {
                1
            } else {
                -1
            },
            0,
            0,
            &mut history,
            &mut move_list,
            &mut attack_container,
            &mut see_buffer,
        );
        quiet_nonstripped.push(LabelledGameState {
            game_state: state.clone(),
            label: position.label,
        });
        if score.abs() < 1000 {
            quiet_stripped.push(LabelledGameState {
                game_state: state,
                label: position.label,
            });
        }
    }
    println!("Quiet positions: {}", quiet_nonstripped.len());
    println!("Quiet and stripped positions: {}", quiet_stripped.len());
    save_positions(
        &format!("{}/all_positions_qsearch.txt", FEN_DIR),
        &quiet_nonstripped,
    );
    /*save_positions(
        &format!("{}/all_positions_qsearchstripped.txt", FEN_DIR),
        &quiet_stripped,
    );*/
}

pub fn stripped_q_search(
    mut alpha: i16,
    beta: i16,
    game_state: GameState,
    color: i16,
    current_depth: usize,
    depth_left: i16,
    history: &mut History,
    move_list: &mut ReservedMoveList,
    attack_container: &mut ReservedAttackContainer,
    see_buffer: &mut Vec<i16>,
) -> (i16, GameState) {
    //Check for draw
    if check_for_draw(&game_state, history) {
        return (leaf_score(GameResult::Draw, color, depth_left), game_state);
    }
    let incheck = in_check_slow(&game_state);
    attack_container.attack_containers[current_depth].write_state(&game_state);
    let static_evaluation = eval_game_state(
        &game_state,
        &attack_container.attack_containers[current_depth],
    );
    //Standing pat pruning
    let stand_pat = static_evaluation.final_eval * color;
    if !incheck && stand_pat >= beta {
        return (stand_pat, game_state);
    }
    if !incheck && stand_pat > alpha {
        alpha = stand_pat;
    }
    //Big Delta Pruning
    let diff = alpha - stand_pat - DELTA_PRUNING;
    if !incheck && diff > 0 && best_move_value(&game_state) < diff {
        return (stand_pat, game_state);
    }
    history.push(game_state.hash, game_state.half_moves == 0);
    let (agsi, max_index) = make_moves(
        &game_state,
        &mut move_list.move_lists[current_depth],
        &attack_container.attack_containers[current_depth],
        static_evaluation.phase,
        stand_pat,
        alpha,
        see_buffer,
        incheck,
    );
    let incheck = agsi.stm_incheck;
    let has_legal_move = agsi.stm_haslegalmove;

    let mut current_max_score = if incheck { STANDARD_SCORE } else { stand_pat };
    let mut current_best_state: Option<GameState> = None;
    let mut index = 0;
    while index < max_index {
        let r = get_next_gm(&mut move_list.move_lists[current_depth], index, max_index).0;
        let capture_move: GameMove =
            move_list.move_lists[current_depth].move_list[r].expect("Could not get next gm");
        let next_g = make_move(&game_state, &capture_move);
        let (score, other_state) = stripped_q_search(
            -beta,
            -alpha,
            next_g,
            -color,
            current_depth + 1,
            depth_left - 1,
            history,
            move_list,
            attack_container,
            see_buffer,
        );

        if -score > current_max_score {
            current_max_score = -score;
            current_best_state = Some(other_state);
        }
        if -score >= beta {
            break;
        }
        index += 1;
    }
    history.pop();
    let game_status = check_end_condition(&game_state, has_legal_move, incheck);
    if game_status != GameResult::Ingame {
        return (leaf_score(game_status, color, depth_left), game_state);
    }
    if current_best_state.is_none() {
        return (stand_pat, game_state);
    }
    (
        current_max_score,
        current_best_state.expect("Couldn't unwrap this"),
    )
}

pub fn make_moves(
    game_state: &GameState,
    move_list: &mut MoveList,
    attack_container: &GameStateAttackContainer,
    phase: f64,
    stand_pat: i16,
    alpha: i16,
    see_buffer: &mut Vec<i16>,
    incheck: bool,
) -> (AdditionalGameStateInformation, usize) {
    let agsi = movegen::generate_moves(&game_state, !incheck, move_list, attack_container);
    let (mut mv_index, mut capture_index) = (0, 0);
    while mv_index < move_list.counter {
        let mv: &GameMove = move_list.move_list[mv_index].as_ref().unwrap();
        if let GameMoveType::EnPassant = mv.move_type {
            move_list.graded_moves[capture_index] = Some(GradedMove::new(mv_index, 100.0));
        } else {
            if !incheck && !passes_delta_pruning(mv, phase, stand_pat, alpha) {
                mv_index += 1;
                continue;
            }
            if capture_index > 0 && is_capture(mv) || !incheck {
                let score = see(&game_state, mv, true, see_buffer);
                if score < 0 {
                    mv_index += 1;
                    continue;
                }
                move_list.graded_moves[capture_index] =
                    Some(GradedMove::new(mv_index, f64::from(score)));
            } else {
                move_list.graded_moves[capture_index] = Some(GradedMove::new(mv_index, 0.));
            }
        }
        mv_index += 1;
        capture_index += 1;
    }
    (agsi, capture_index)
}
