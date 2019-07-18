use super::super::board_representation::game_state::{
    GameMove, GameMoveType, GameResult, GameState, PieceType,
};
use super::super::evaluation::{self, eval_game_state};
use super::super::move_generation::movegen;
use super::super::move_generation::movegen::{AdditionalGameStateInformation, MoveList};
use super::alphabeta::STANDARD_SCORE;
use super::alphabeta::{check_end_condition, clear_pv, leaf_score};
use super::cache::{Cache, CacheEntry};
use super::history::History;
use super::search::Search;
use super::GradedMove;
use crate::bitboards;
use crate::move_generation::makemove::make_move;

pub const DELTA_PRUNING: i16 = 100;
lazy_static! {
    pub static ref PIECE_VALUES: [i16; 6] = [100, 300, 310, 500, 900, 30000];
}

pub fn q_search(
    mut alpha: i16,
    mut beta: i16,
    game_state: &GameState,
    color: i16,
    depth_left: i16,
    current_depth: usize,
    search: &mut Search,
    history: &mut History,
    cache: &mut Cache,
    root_plies_played: usize,
    move_list: &mut MoveList,
    agsi: AdditionalGameStateInformation,
    is_q_root: bool,
) -> i16 {
    search.search_statistics.add_q_node(current_depth);
    clear_pv(current_depth, search);
    if search.stop {
        return STANDARD_SCORE;
    }
    let game_status = check_end_condition(
        &game_state,
        agsi.stm_haslegalmove,
        agsi.stm_incheck,
        history,
    );
    if game_status != GameResult::Ingame {
        return leaf_score(game_status, color, depth_left);
    }

    let static_evaluation = eval_game_state(&game_state, false);
    let stand_pat = static_evaluation.final_eval * color;
    if stand_pat >= beta {
        return stand_pat;
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    //Apply Big Delta Pruning
    let diff = alpha - stand_pat - DELTA_PRUNING;
    //Missing stats
    if diff > 0 && best_move_value(game_state) < diff {
        return stand_pat;
    }

    let (mut mv_index, mut capture_index) = (0, 0);
    while mv_index < move_list.counter[current_depth] {
        let mv: &GameMove = move_list.move_list[current_depth][mv_index]
            .as_ref()
            .unwrap();
        if is_q_root && !is_capture(mv) {
            mv_index += 1;
            continue;
        }
        if let GameMoveType::EnPassant = mv.move_type {
            move_list.graded_moves[current_depth][capture_index] =
                Some(GradedMove::new(mv_index, 100.0));
        } else {
            if !passes_delta_pruning(mv, static_evaluation.phase, stand_pat, alpha) {
                search.search_statistics.add_q_delta_cutoff();
                mv_index += 1;
                continue;
            }
            let score = see(&game_state, mv, true, &mut search.see_buffer);
            if score < 0 {
                search.search_statistics.add_q_see_cutoff();
                mv_index += 1;
                continue;
            }
            move_list.graded_moves[current_depth][capture_index] =
                Some(GradedMove::new(mv_index, score as f64));
        }
        mv_index += 1;
        capture_index += 1;
    }
    //Probe TT
    {
        let ce = &cache.cache[game_state.hash as usize & super::cache::CACHE_MASK];
        if let Some(s) = ce {
            let ce: &CacheEntry = s;
            if ce.hash == game_state.hash {
                search.search_statistics.add_cache_hit_qs();
                if ce.depth >= depth_left as i8 {
                    if !ce.alpha && !ce.beta {
                        search.search_statistics.add_cache_hit_replace_qs();
                        search.pv_table[current_depth].pv[0] =
                            Some(CacheEntry::u16_to_mv(ce.mv, &game_state));
                        return ce.score;
                    } else {
                        if ce.beta {
                            if ce.score > alpha {
                                alpha = ce.score;
                            }
                        } else {
                            if ce.score < beta {
                                beta = ce.score;
                            }
                        }
                        if alpha >= beta {
                            search.search_statistics.add_cache_hit_aj_replace_qs();
                            search.pv_table[current_depth].pv[0] =
                                Some(CacheEntry::u16_to_mv(ce.mv, &game_state));
                            return ce.score;
                        }
                    }
                }
                let mv = CacheEntry::u16_to_mv(ce.mv, &game_state);
                let mut c_i = 0;
                while c_i < capture_index {
                    let other_mv: &GameMove = &move_list.move_list[current_depth][move_list
                        .graded_moves[current_depth][c_i]
                        .as_ref()
                        .unwrap()
                        .mv_index]
                        .as_ref()
                        .unwrap();
                    if other_mv.from == mv.from
                        && other_mv.to == mv.to
                        && other_mv.move_type == mv.move_type
                    {
                        move_list.graded_moves[current_depth][c_i]
                            .as_mut()
                            .unwrap()
                            .score += 10000.0;
                        break;
                    }
                    c_i += 1;
                }
            }
        }
    }
    history.push(game_state.hash, game_state.half_moves == 0);
    let mut index = 0;
    let mut current_max_score = stand_pat;
    let mut has_move = false;
    while index < capture_index {
        let gmvindex =
            super::alphabeta::get_next_gm(move_list, current_depth, index, capture_index);
        let capture_move = move_list.move_list[current_depth][gmvindex].unwrap();
        let next_g = make_move(&game_state, &capture_move);
        let next_g_agsi = movegen::generate_moves2(&next_g, true, move_list, current_depth + 1);
        let score = -q_search(
            -beta,
            -alpha,
            &next_g,
            -color,
            depth_left - 1,
            current_depth + 1,
            search,
            history,
            cache,
            root_plies_played,
            move_list,
            next_g_agsi,
            false,
        );
        if score > current_max_score {
            current_max_score = score;
            search.pv_table[current_depth].pv[0] = Some(capture_move);
            has_move = true;
            //Hang on following pv in theory
        }
        if score >= beta {
            search.search_statistics.add_q_beta_cutoff(index);
            break;
        }
        if score > alpha {
            alpha = score;
        }
        index += 1;
    }
    history.pop();
    if current_max_score < beta {
        if index > 0 {
            search.search_statistics.add_q_beta_noncutoff();
        }
    }
    if has_move {
        super::alphabeta::make_cache(
            cache,
            &search.pv_table[current_depth],
            current_max_score,
            &game_state,
            alpha,
            beta,
            0,
            root_plies_played,
            Some(static_evaluation.final_eval),
        );
    }
    current_max_score
}

#[inline(always)]
pub fn is_capture(mv: &GameMove) -> bool {
    match &mv.move_type {
        GameMoveType::Capture(_) => true,
        GameMoveType::Promotion(_, s) => match s {
            Some(_) => true,
            _ => false,
        },
        GameMoveType::EnPassant => true,
        _ => false,
    }
}

#[inline(always)]
pub fn best_move_value(state: &GameState) -> i16 {
    let mut res = 0;
    let mut i = 4;
    while i > 0 {
        if state.pieces[i][1 - state.color_to_move] != 0u64 {
            res = PIECE_VALUES[i];
            break;
        }
        i -= 1;
    }

    if (state.pieces[0][state.color_to_move]
        & bitboards::RANKS[if state.color_to_move == 0 { 6 } else { 1 }])
        != 0u64
    {
        res += PIECE_VALUES[4] - PIECE_VALUES[0];
    }
    res
}

#[inline(always)]
pub fn passes_delta_pruning(capture_move: &GameMove, phase: f64, eval: i16, alpha: i16) -> bool {
    if phase == 0.0 || eval >= alpha {
        return true;
    }
    if let GameMoveType::Promotion(_, _) = capture_move.move_type {
        return true;
    }
    let captured_piece = match &capture_move.move_type {
        GameMoveType::Capture(c) => c,
        GameMoveType::EnPassant => &PieceType::Pawn,
        _ => panic!("No capture!"),
    };
    eval + evaluation::piece_value(&captured_piece, phase) + DELTA_PRUNING >= alpha
}

#[inline(always)]
pub fn see(game_state: &GameState, mv: &GameMove, exact: bool, gain: &mut Vec<i16>) -> i16 {
    let may_xray = game_state.pieces[0][0]
        | game_state.pieces[0][1]
        | game_state.pieces[2][0]
        | game_state.pieces[2][1]
        | game_state.pieces[3][0]
        | game_state.pieces[3][1]
        | game_state.pieces[4][0]
        | game_state.pieces[4][1];
    let mut from_set = 1u64 << mv.from;
    let mut occ = get_occupied_board(&game_state);
    let mut attadef = attacks_to(&game_state, mv.to, occ);
    gain[0] = capture_value(&mv);
    let mut color_to_move = game_state.color_to_move;
    let mut attacked_piece = match mv.piece_type {
        PieceType::Pawn => 0,
        PieceType::Knight => 1,
        PieceType::Bishop => 2,
        PieceType::Rook => 3,
        PieceType::Queen => 4,
        PieceType::King => 5,
    };
    let mut index = 0;
    let mut deleted_pieces = 0u64;
    while from_set != 0u64 {
        deleted_pieces |= from_set;
        index += 1;
        gain[index] = PIECE_VALUES[attacked_piece] - gain[index - 1];
        if !exact && (-gain[index - 1]).max(gain[index]) < 0 {
            break;
        }
        attadef ^= from_set;
        occ ^= from_set;
        if from_set & may_xray != 0u64 {
            //Recalculate rays
            attadef |=
                recalculate_sliders(&game_state, color_to_move, mv.to, occ) & (!deleted_pieces);
        }
        color_to_move = 1 - color_to_move;
        let res = least_valuable_piece(attadef, color_to_move, &game_state);
        from_set = res.0;
        attacked_piece = res.1;
        if attacked_piece == 5
            && least_valuable_piece(attadef, 1 - color_to_move, &game_state).1 != 1000
        {
            break;
        }
    }
    while index > 1 {
        index -= 1;
        gain[index - 1] = -((-gain[index - 1]).max(gain[index]));
    }
    gain[0]
}

#[inline(always)]
pub fn recalculate_sliders(
    game_state: &GameState,
    color_to_move: usize,
    square: usize,
    occ: u64,
) -> u64 {
    //Bishops
    movegen::bishop_attack(square, occ)
        & (game_state.pieces[2][color_to_move] | game_state.pieces[4][color_to_move])
        | movegen::rook_attack(square, occ)
            & (game_state.pieces[3][color_to_move] | game_state.pieces[4][color_to_move])
}

#[inline(always)]
pub fn attacks_to(game_state: &GameState, square: usize, occ: u64) -> u64 {
    let square_board = 1u64 << square;
    movegen::attackers_from_white(
        square_board,
        square,
        game_state.pieces[0][0],
        game_state.pieces[1][0],
        game_state.pieces[2][0] | game_state.pieces[4][0],
        game_state.pieces[3][0] | game_state.pieces[4][0],
        occ,
    )
    .0 | movegen::attackers_from_black(
        square_board,
        square,
        game_state.pieces[0][1],
        game_state.pieces[1][1],
        game_state.pieces[2][1] | game_state.pieces[4][1],
        game_state.pieces[3][1] | game_state.pieces[4][1],
        occ,
    )
    .0 | bitboards::KING_ATTACKS[square] & (game_state.pieces[5][0] | game_state.pieces[5][1])
}

#[inline(always)]
pub fn capture_value(mv: &GameMove) -> i16 {
    match &mv.move_type {
        GameMoveType::Capture(c) => piece_value(&c),
        GameMoveType::Promotion(_, b) => match b {
            Some(c) => piece_value(&c),
            _ => panic!("Promotion but no capture"),
        },
        _ => panic!("No capture"),
    }
}

#[inline(always)]
pub fn piece_value(piece_type: &PieceType) -> i16 {
    match piece_type {
        PieceType::Pawn => PIECE_VALUES[0],
        PieceType::Knight => PIECE_VALUES[1],
        PieceType::Bishop => PIECE_VALUES[2],
        PieceType::Rook => PIECE_VALUES[3],
        PieceType::Queen => PIECE_VALUES[4],
        PieceType::King => PIECE_VALUES[5],
    }
}

#[inline(always)]
pub fn get_occupied_board(game_state: &GameState) -> u64 {
    game_state.pieces[0][0]
        | game_state.pieces[1][0]
        | game_state.pieces[2][0]
        | game_state.pieces[3][0]
        | game_state.pieces[4][0]
        | game_state.pieces[5][0]
        | game_state.pieces[0][1]
        | game_state.pieces[1][1]
        | game_state.pieces[2][1]
        | game_state.pieces[3][1]
        | game_state.pieces[4][1]
        | game_state.pieces[5][1]
}

#[inline(always)]
pub fn least_valuable_piece(
    from_board: u64,
    color_to_move: usize,
    game_state: &GameState,
) -> (u64, usize) {
    for i in 0..6 {
        let subset = game_state.pieces[i][color_to_move] & from_board;
        if subset != 0u64 {
            return (1u64 << subset.trailing_zeros(), i);
        }
    }
    (0u64, 1000)
}

#[cfg(test)]
mod tests {
    use super::see;
    use super::GameMove;
    use super::GameMoveType;
    use super::GameState;
    use super::PieceType;

    #[test]
    fn see_test() {
        let mut see_buffer = vec![0i16; 128];
        assert_eq!(
            see(
                &GameState::from_fen("1k1r4/1pp4p/p7/4p3/8/P5P1/1PP4P/2K1R3 w - -"),
                &GameMove {
                    from: 4usize,
                    to: 36usize,
                    move_type: GameMoveType::Capture(PieceType::Pawn),
                    piece_type: PieceType::Rook,
                },
                true,
                &mut see_buffer
            ),
            100
        );
        assert_eq!(
            see(
                &GameState::from_fen("1k2r3/1pp4p/p7/4p3/8/P5P1/1PP4P/2K1R3 w - -"),
                &GameMove {
                    from: 4usize,
                    to: 36usize,
                    move_type: GameMoveType::Capture(PieceType::Pawn),
                    piece_type: PieceType::Rook,
                },
                true,
                &mut see_buffer
            ),
            -400
        );
        assert_eq!(
            see(
                &GameState::from_fen("1k1r3q/1ppn3p/p4b2/4p3/8/P2N2P1/1PP1R1BP/2K1Q3 w - -"),
                &GameMove {
                    from: 19,
                    to: 36,
                    move_type: GameMoveType::Capture(PieceType::Pawn),
                    piece_type: PieceType::Knight,
                },
                true,
                &mut see_buffer
            ),
            -200
        );
        assert_eq!(
            see(
                &GameState::from_fen("1k1r3q/1ppn3p/p4b2/4n3/8/P2N2P1/1PP1R1BP/2K1Q3 w - -"),
                &GameMove {
                    from: 19,
                    to: 36,
                    move_type: GameMoveType::Capture(PieceType::Knight),
                    piece_type: PieceType::Knight,
                },
                true,
                &mut see_buffer
            ),
            0
        );
        assert_eq!(
            see(
                &GameState::from_fen("1k1r2q1/1ppn3p/p4b2/4p3/8/P2N2P1/1PP1R1BP/2K1Q3 w - -"),
                &GameMove {
                    from: 19,
                    to: 36,
                    move_type: GameMoveType::Capture(PieceType::Pawn),
                    piece_type: PieceType::Knight,
                },
                true,
                &mut see_buffer
            ),
            -90
        );
        assert_eq!(
            see(
                &GameState::from_fen("8/8/3p4/4r3/2RKP3/5k2/8/8 b - -"),
                &GameMove {
                    from: 36,
                    to: 28,
                    move_type: GameMoveType::Capture(PieceType::Pawn),
                    piece_type: PieceType::Rook,
                },
                true,
                &mut see_buffer
            ),
            100
        );
        assert_eq!(
            see(
                &GameState::from_fen("k7/8/5q2/8/3r4/2KQ4/8/8 w - -"),
                &GameMove {
                    from: 19,
                    to: 27,
                    move_type: GameMoveType::Capture(PieceType::Rook),
                    piece_type: PieceType::Queen,
                },
                true,
                &mut see_buffer
            ),
            500
        );
        assert_eq!(
            see(
                &GameState::from_fen("8/8/5q2/2k5/3r4/2KQ4/8/8 w - -"),
                &GameMove {
                    from: 19,
                    to: 27,
                    move_type: GameMoveType::Capture(PieceType::Rook),
                    piece_type: PieceType::Queen,
                },
                true,
                &mut see_buffer
            ),
            -400
        );
        assert_eq!(
            see(
                &GameState::from_fen("4pq2/3P4/8/8/8/8/8/k1K5 w - -"),
                &GameMove {
                    from: 51,
                    to: 60,
                    move_type: GameMoveType::Promotion(PieceType::Queen, Some(PieceType::Pawn)),
                    piece_type: PieceType::Pawn,
                },
                true,
                &mut see_buffer
            ),
            0
        );
        assert_eq!(
            see(
                &GameState::from_fen("4pq2/3P4/2B5/8/8/8/8/k1K5 w - -"),
                &GameMove {
                    from: 51,
                    to: 60,
                    move_type: GameMoveType::Promotion(PieceType::Queen, Some(PieceType::Pawn)),
                    piece_type: PieceType::Pawn,
                },
                true,
                &mut see_buffer
            ),
            100
        );
    }
}