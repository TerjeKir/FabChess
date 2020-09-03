use super::params::*;
use super::EvaluationScore;
use crate::board_representation::game_state::PIECE_TYPES;
use crate::evaluation::psqt_evaluation::BLACK_INDEX;
use std::fmt::{Display, Formatter, Result};
use std::fs;

pub const IDX_TEMPO_BONUS: usize = 0;
pub const SIZE_TEMPO_BONUS: usize = 2;

pub const IDX_SHIELDING_PAWN_MISSING: usize = IDX_TEMPO_BONUS + SIZE_TEMPO_BONUS;
pub const SIZE_SHIELDING_PAWN_MISSING: usize = 8;

pub const IDX_SHIELDING_PAWN_ONOPEN_MISSING: usize =
    IDX_SHIELDING_PAWN_MISSING + SIZE_SHIELDING_PAWN_MISSING;
pub const SIZE_SHIELDING_PAWN_ONOPEN_MISSING: usize = 8;

pub const IDX_PAWN_DOUBLED: usize =
    IDX_SHIELDING_PAWN_ONOPEN_MISSING + SIZE_SHIELDING_PAWN_ONOPEN_MISSING;
pub const SIZE_PAWN_DOUBLED: usize = 2;

pub const IDX_PAWN_ISOLATED: usize = IDX_PAWN_DOUBLED + SIZE_PAWN_DOUBLED;
pub const SIZE_PAWN_ISOLATED: usize = 2;

pub const IDX_PAWN_BACKWARD: usize = IDX_PAWN_ISOLATED + SIZE_PAWN_ISOLATED;
pub const SIZE_PAWN_BACKWARD: usize = 2;

pub const IDX_PAWN_SUPPORTED: usize = IDX_PAWN_BACKWARD + SIZE_PAWN_BACKWARD;
pub const SIZE_PAWN_SUPPORTED: usize = 128;

pub const IDX_PAWN_ATTACK_CENTER: usize = IDX_PAWN_SUPPORTED + SIZE_PAWN_SUPPORTED;
pub const SIZE_PAWN_ATTACK_CENTER: usize = 2;

pub const IDX_PAWN_MOBILITY: usize = IDX_PAWN_ATTACK_CENTER + SIZE_PAWN_ATTACK_CENTER;
pub const SIZE_PAWN_MOBILITY: usize = 2;

pub const IDX_PAWN_PASSED: usize = IDX_PAWN_MOBILITY + SIZE_PAWN_MOBILITY;
pub const SIZE_PAWN_PASSED: usize = 14;

pub const IDX_PAWN_PASSED_NOTBLOCKED: usize = IDX_PAWN_PASSED + SIZE_PAWN_PASSED;
pub const SIZE_PAWN_PASSED_NOTBLOCKED: usize = 14;

pub const IDX_PAWN_PASSED_KINGDISTANCE: usize =
    IDX_PAWN_PASSED_NOTBLOCKED + SIZE_PAWN_PASSED_NOTBLOCKED;
pub const SIZE_PAWN_PASSED_KINGDISTANCE: usize = 14;

pub const IDX_PAWN_PASSED_ENEMYKINGDISTANCE: usize =
    IDX_PAWN_PASSED_KINGDISTANCE + SIZE_PAWN_PASSED_KINGDISTANCE;
pub const SIZE_PAWN_PASSED_ENEMYKINGDISTANCE: usize = 14;

pub const IDX_PAWN_PASSED_SUBDISTANCE: usize =
    IDX_PAWN_PASSED_ENEMYKINGDISTANCE + SIZE_PAWN_PASSED_ENEMYKINGDISTANCE;
pub const SIZE_PAWN_PASSED_SUBDISTANCE: usize = 26;

pub const IDX_ROOK_BEHIND_SUPPORT_PASSER: usize =
    IDX_PAWN_PASSED_SUBDISTANCE + SIZE_PAWN_PASSED_SUBDISTANCE;
pub const SIZE_ROOK_BEHIND_SUPPORT_PASSER: usize = 2;

pub const IDX_ROOK_BEHIND_ENEMY_PASSER: usize =
    IDX_ROOK_BEHIND_SUPPORT_PASSER + SIZE_ROOK_BEHIND_SUPPORT_PASSER;
pub const SIZE_ROOK_BEHIND_ENEMY_PASSER: usize = 2;

pub const IDX_PAWN_PASSED_WEAK: usize =
    IDX_ROOK_BEHIND_ENEMY_PASSER + SIZE_ROOK_BEHIND_ENEMY_PASSER;
pub const SIZE_PAWN_PASSED_WEAK: usize = 2;

pub const IDX_KNIGHT_SUPPORTED: usize = IDX_PAWN_PASSED_WEAK + SIZE_PAWN_PASSED_WEAK;
pub const SIZE_KNIGHT_SUPPORTED: usize = 2;

pub const IDX_KNIGHT_OUTPOST_TABLE: usize = IDX_KNIGHT_SUPPORTED + SIZE_KNIGHT_SUPPORTED;
pub const SIZE_KNIGHT_OUTPOST_TABLE: usize = 128;

pub const IDX_ROOK_ON_OPEN: usize = IDX_KNIGHT_OUTPOST_TABLE + SIZE_KNIGHT_OUTPOST_TABLE;
pub const SIZE_ROOK_ON_OPEN: usize = 2;

pub const IDX_ROOK_ON_SEMI_OPEN: usize = IDX_ROOK_ON_OPEN + SIZE_ROOK_ON_OPEN;
pub const SIZE_ROOK_ON_SEMI_OPEN: usize = 2;

pub const IDX_QUEEN_ON_OPEN: usize = IDX_ROOK_ON_SEMI_OPEN + SIZE_ROOK_ON_SEMI_OPEN;
pub const SIZE_QUEEN_ON_OPEN: usize = 2;

pub const IDX_QUEEN_ON_SEMI_OPEN: usize = IDX_QUEEN_ON_OPEN + SIZE_QUEEN_ON_OPEN;
pub const SIZE_QUEEN_ON_SEMI_OPEN: usize = 2;

pub const IDX_ROOK_ON_SEVENTH: usize = IDX_QUEEN_ON_SEMI_OPEN + SIZE_QUEEN_ON_SEMI_OPEN;
pub const SIZE_ROOK_ON_SEVENTH: usize = 2;

pub const IDX_PAWN_PIECE_VALUE: usize = IDX_ROOK_ON_SEVENTH + SIZE_ROOK_ON_SEVENTH;
pub const SIZE_PAWN_PIECE_VALUE: usize = 2;

pub const IDX_KNIGHT_PIECE_VALUE: usize = IDX_PAWN_PIECE_VALUE + SIZE_PAWN_PIECE_VALUE;
pub const SIZE_KNIGHT_PIECE_VALUE: usize = 2;

pub const IDX_KNIGHT_VALUE_WITH_PAWN: usize = IDX_KNIGHT_PIECE_VALUE + SIZE_KNIGHT_PIECE_VALUE;
pub const SIZE_KNIGHT_VALUE_WITH_PAWN: usize = 17;

pub const IDX_BISHOP_PIECE_VALUE: usize = IDX_KNIGHT_VALUE_WITH_PAWN + SIZE_KNIGHT_VALUE_WITH_PAWN;
pub const SIZE_BISHOP_PIECE_VALUE: usize = 2;

pub const IDX_BISHOP_PAIR: usize = IDX_BISHOP_PIECE_VALUE + SIZE_BISHOP_PIECE_VALUE;
pub const SIZE_BISHOP_PAIR: usize = 2;

pub const IDX_ROOK_PIECE_VALUE: usize = IDX_BISHOP_PAIR + SIZE_BISHOP_PAIR;
pub const SIZE_ROOK_PIECE_VALUE: usize = 2;

pub const IDX_QUEEN_PIECE_VALUE: usize = IDX_ROOK_PIECE_VALUE + SIZE_ROOK_PIECE_VALUE;
pub const SIZE_QUEEN_PIECE_VALUE: usize = 2;

pub const IDX_DIAGONALLY_ADJ_SQ_WPAWNS: usize = IDX_QUEEN_PIECE_VALUE + SIZE_QUEEN_PIECE_VALUE;
pub const SIZE_DIAGONALLY_ADJ_SQ_WPAWNS: usize = 10;

pub const IDX_KNIGHT_MOBILITY: usize = IDX_DIAGONALLY_ADJ_SQ_WPAWNS + SIZE_DIAGONALLY_ADJ_SQ_WPAWNS;
pub const SIZE_KNIGHT_MOBILITY: usize = 18;

pub const IDX_BISHOP_MOBILITY: usize = IDX_KNIGHT_MOBILITY + SIZE_KNIGHT_MOBILITY;
pub const SIZE_BISHOP_MOBILITY: usize = 28;

pub const IDX_ROOK_MOBILITY: usize = IDX_BISHOP_MOBILITY + SIZE_BISHOP_MOBILITY;
pub const SIZE_ROOK_MOBILITY: usize = 30;

pub const IDX_QUEEN_MOBILITY: usize = IDX_ROOK_MOBILITY + SIZE_ROOK_MOBILITY;
pub const SIZE_QUEEN_MOBILITY: usize = 56;

pub const IDX_ATTACK_WEIGHT: usize = IDX_QUEEN_MOBILITY + SIZE_QUEEN_MOBILITY;
pub const SIZE_ATTACK_WEIGHT: usize = 16;

pub const IDX_SAFETY_TABLE: usize = IDX_ATTACK_WEIGHT + SIZE_ATTACK_WEIGHT;
pub const SIZE_SAFETY_TABLE: usize = 200;

pub const IDX_KNIGHT_ATTACK_VALUE: usize = IDX_SAFETY_TABLE + SIZE_SAFETY_TABLE;
pub const SIZE_KNIGHT_ATTACK_VALUE: usize = 2;

pub const IDX_BISHOP_ATTACK_VALUE: usize = IDX_KNIGHT_ATTACK_VALUE + SIZE_KNIGHT_ATTACK_VALUE;
pub const SIZE_BISHOP_ATTACK_VALUE: usize = 2;

pub const IDX_ROOK_ATTACK_VALUE: usize = IDX_BISHOP_ATTACK_VALUE + SIZE_BISHOP_ATTACK_VALUE;
pub const SIZE_ROOK_ATTACK_VALUE: usize = 2;

pub const IDX_QUEEN_ATTACK_VALUE: usize = IDX_ROOK_ATTACK_VALUE + SIZE_ROOK_ATTACK_VALUE;
pub const SIZE_QUEEN_ATTACK_VALUE: usize = 2;

pub const IDX_KNIGHT_CHECK_VALUE: usize = IDX_QUEEN_ATTACK_VALUE + SIZE_QUEEN_ATTACK_VALUE;
pub const SIZE_KNIGHT_CHECK_VALUE: usize = 2;

pub const IDX_BISHOP_CHECK_VALUE: usize = IDX_KNIGHT_CHECK_VALUE + SIZE_KNIGHT_CHECK_VALUE;
pub const SIZE_BISHOP_CHECK_VALUE: usize = 2;

pub const IDX_ROOK_CHECK_VALUE: usize = IDX_BISHOP_CHECK_VALUE + SIZE_BISHOP_CHECK_VALUE;
pub const SIZE_ROOK_CHECK_VALUE: usize = 2;

pub const IDX_QUEEN_CHECK_VALUE: usize = IDX_ROOK_CHECK_VALUE + SIZE_ROOK_CHECK_VALUE;
pub const SIZE_QUEEN_CHECK_VALUE: usize = 2;

pub const IDX_PSQT: usize = IDX_QUEEN_CHECK_VALUE + SIZE_QUEEN_CHECK_VALUE;
pub const SIZE_PSQT: usize = 768;

pub const IDX_SLIGHTLY_WINNING_NO_PAWN: usize = IDX_PSQT + SIZE_PSQT;
pub const SIZE_SLIGHTLY_WINNING_NO_PAWN: usize = 1;

pub const IDX_SLIGHTLY_WINNING_ENEMY_CAN_SAC: usize =
    IDX_SLIGHTLY_WINNING_NO_PAWN + SIZE_SLIGHTLY_WINNING_NO_PAWN;
pub const SIZE_SLIGHTLY_WINNING_ENEMY_CAN_SAC: usize = 1;

pub const TOTAL_PARAMS: usize =
    IDX_SLIGHTLY_WINNING_ENEMY_CAN_SAC + SIZE_SLIGHTLY_WINNING_ENEMY_CAN_SAC;

#[derive(Clone)]
pub struct Parameters {
    pub params: [f32; TOTAL_PARAMS],
}
impl Parameters {
    pub fn write_to_file(&self, file: &str) {
        fs::write(file, &format!("{}", self)).expect("Unable to write file");
    }
    fn init_psqt(params: &mut [f32; TOTAL_PARAMS], s: &[[EvaluationScore; 8]; 8], idx: usize) {
        for i in 0..8 {
            Parameters::init_constants(params, &s[i], idx + 16 * i)
        }
    }
    fn init_constants(params: &mut [f32; TOTAL_PARAMS], s: &[EvaluationScore], idx: usize) {
        for i in 0..s.len() {
            Parameters::init_constant(params, s[i], idx + 2 * i)
        }
    }
    fn init_constant(params: &mut [f32; TOTAL_PARAMS], s: EvaluationScore, idx: usize) {
        params[idx] = f32::from(s.0);
        params[idx + 1] = f32::from(s.1);
    }
    pub fn default() -> Self {
        let mut params = [0.; TOTAL_PARAMS];
        Parameters::init_constant(&mut params, TEMPO_BONUS, IDX_TEMPO_BONUS);
        Parameters::init_constants(
            &mut params,
            &SHIELDING_PAWN_MISSING,
            IDX_SHIELDING_PAWN_MISSING,
        );
        Parameters::init_constants(
            &mut params,
            &SHIELDING_PAWN_MISSING_ON_OPEN_FILE,
            IDX_SHIELDING_PAWN_ONOPEN_MISSING,
        );
        Parameters::init_constant(&mut params, PAWN_DOUBLED_VALUE, IDX_PAWN_DOUBLED);
        Parameters::init_constant(&mut params, PAWN_ISOLATED_VALUE, IDX_PAWN_ISOLATED);
        Parameters::init_constant(&mut params, PAWN_BACKWARD_VALUE, IDX_PAWN_BACKWARD);
        Parameters::init_psqt(&mut params, &PAWN_SUPPORTED_VALUE, IDX_PAWN_SUPPORTED);
        Parameters::init_constant(&mut params, PAWN_ATTACK_CENTER, IDX_PAWN_ATTACK_CENTER);
        Parameters::init_constant(&mut params, PAWN_MOBILITY, IDX_PAWN_MOBILITY);
        Parameters::init_constants(&mut params, &PAWN_PASSED_VALUES, IDX_PAWN_PASSED);
        Parameters::init_constants(
            &mut params,
            &PAWN_PASSED_NOT_BLOCKED_VALUES,
            IDX_PAWN_PASSED_NOTBLOCKED,
        );
        Parameters::init_constants(
            &mut params,
            &PASSED_KING_DISTANCE,
            IDX_PAWN_PASSED_KINGDISTANCE,
        );
        Parameters::init_constants(
            &mut params,
            &PASSED_ENEMY_KING_DISTANCE,
            IDX_PAWN_PASSED_ENEMYKINGDISTANCE,
        );
        Parameters::init_constants(
            &mut params,
            &PASSED_SUBTRACT_DISTANCE,
            IDX_PAWN_PASSED_SUBDISTANCE,
        );
        Parameters::init_constant(
            &mut params,
            ROOK_BEHIND_SUPPORT_PASSER,
            IDX_ROOK_BEHIND_SUPPORT_PASSER,
        );
        Parameters::init_constant(
            &mut params,
            ROOK_BEHIND_ENEMY_PASSER,
            IDX_ROOK_BEHIND_ENEMY_PASSER,
        );
        Parameters::init_constant(&mut params, PAWN_PASSED_WEAK, IDX_PAWN_PASSED_WEAK);
        Parameters::init_constant(&mut params, KNIGHT_SUPPORTED_BY_PAWN, IDX_KNIGHT_SUPPORTED);
        Parameters::init_psqt(&mut params, &KNIGHT_OUTPOST_TABLE, IDX_KNIGHT_OUTPOST_TABLE);
        Parameters::init_constant(&mut params, ROOK_ON_OPEN_FILE_BONUS, IDX_ROOK_ON_OPEN);
        Parameters::init_constant(
            &mut params,
            ROOK_ON_SEMI_OPEN_FILE_BONUS,
            IDX_ROOK_ON_SEMI_OPEN,
        );
        Parameters::init_constant(&mut params, QUEEN_ON_OPEN_FILE_BONUS, IDX_QUEEN_ON_OPEN);
        Parameters::init_constant(
            &mut params,
            QUEEN_ON_SEMI_OPEN_FILE_BONUS,
            IDX_QUEEN_ON_SEMI_OPEN,
        );
        Parameters::init_constant(&mut params, ROOK_ON_SEVENTH, IDX_ROOK_ON_SEVENTH);
        Parameters::init_constant(&mut params, PAWN_PIECE_VALUE, IDX_PAWN_PIECE_VALUE);
        Parameters::init_constant(&mut params, KNIGHT_PIECE_VALUE, IDX_KNIGHT_PIECE_VALUE);
        for i in 0..17 {
            params[IDX_KNIGHT_VALUE_WITH_PAWN + i] = f32::from(KNIGHT_VALUE_WITH_PAWNS[i]);
        }
        Parameters::init_constant(&mut params, BISHOP_PIECE_VALUE, IDX_BISHOP_PIECE_VALUE);
        Parameters::init_constant(&mut params, BISHOP_PAIR_BONUS, IDX_BISHOP_PAIR);
        Parameters::init_constant(&mut params, ROOK_PIECE_VALUE, IDX_ROOK_PIECE_VALUE);
        Parameters::init_constant(&mut params, QUEEN_PIECE_VALUE, IDX_QUEEN_PIECE_VALUE);
        Parameters::init_constants(
            &mut params,
            &DIAGONALLY_ADJACENT_SQUARES_WITH_OWN_PAWNS,
            IDX_DIAGONALLY_ADJ_SQ_WPAWNS,
        );
        Parameters::init_constants(&mut params, &KNIGHT_MOBILITY_BONUS, IDX_KNIGHT_MOBILITY);
        Parameters::init_constants(&mut params, &BISHOP_MOBILITY_BONUS, IDX_BISHOP_MOBILITY);
        Parameters::init_constants(&mut params, &ROOK_MOBILITY_BONUS, IDX_ROOK_MOBILITY);
        Parameters::init_constants(&mut params, &QUEEN_MOBILITY_BONUS, IDX_QUEEN_MOBILITY);
        Parameters::init_constants(&mut params, &ATTACK_WEIGHT, IDX_ATTACK_WEIGHT);
        Parameters::init_constants(&mut params, &SAFETY_TABLE, IDX_SAFETY_TABLE);
        Parameters::init_constant(&mut params, KNIGHT_ATTACK_WORTH, IDX_KNIGHT_ATTACK_VALUE);
        Parameters::init_constant(&mut params, BISHOP_ATTACK_WORTH, IDX_BISHOP_ATTACK_VALUE);
        Parameters::init_constant(&mut params, ROOK_ATTACK_WORTH, IDX_ROOK_ATTACK_VALUE);
        Parameters::init_constant(&mut params, QUEEN_ATTACK_WORTH, IDX_QUEEN_ATTACK_VALUE);
        Parameters::init_constant(&mut params, KNIGHT_SAFE_CHECK, IDX_KNIGHT_CHECK_VALUE);
        Parameters::init_constant(&mut params, BISHOP_SAFE_CHECK, IDX_BISHOP_CHECK_VALUE);
        Parameters::init_constant(&mut params, ROOK_SAFE_CHECK, IDX_ROOK_CHECK_VALUE);
        Parameters::init_constant(&mut params, QUEEN_SAFE_CHECK, IDX_QUEEN_CHECK_VALUE);
        for pt in PIECE_TYPES.iter() {
            Parameters::init_psqt(
                &mut params,
                &PSQT[*pt as usize][0],
                IDX_PSQT + *pt as usize * 128,
            )
        }
        params[IDX_SLIGHTLY_WINNING_NO_PAWN] = SLIGHTLY_WINNING_NO_PAWN;
        params[IDX_SLIGHTLY_WINNING_ENEMY_CAN_SAC] = SLIGHTLY_WINNING_ENEMY_CAN_SAC;
        Parameters { params }
    }

    pub fn zero() -> Self {
        Parameters {
            params: [0.; TOTAL_PARAMS],
        }
    }
    fn format_evaluation_score(&self, idx: usize) -> String {
        format!(
            "EvaluationScore({}, {})",
            self.params[idx].round() as isize,
            self.params[idx + 1].round() as isize
        )
    }
    fn format_constant(&self, idx: usize) -> String {
        format!(
            ": EvaluationScore = {};\n",
            self.format_evaluation_score(idx)
        )
    }
    fn format_constants(&self, idx: usize, size: usize) -> String {
        let mut res_str = String::new();
        res_str.push_str(&format!(": [EvaluationScore; {}] = [", size / 2));
        for i in 0..size / 2 {
            res_str.push_str(&self.format_evaluation_score(idx + 2 * i));
            res_str.push_str(",");
        }
        res_str.push_str("];\n");
        res_str
    }
    fn format_psqt(&self, idx: usize) -> String {
        let mut res_str = String::new();
        res_str.push_str("[");
        for i in 0..8 {
            res_str.push_str("[");
            for j in 0..8 {
                res_str.push_str(&format!(
                    "{},",
                    self.format_evaluation_score(idx + 16 * i + 2 * j)
                ))
            }
            res_str.push_str("],");
        }
        res_str.push_str("]");
        res_str
    }
    fn format_psqt_for_black(&self, idx: usize) -> String {
        let mut res_str = String::new();
        res_str.push_str("[");
        for i in 0..8 {
            res_str.push_str("[");
            for j in 0..8 {
                let actual_i = BLACK_INDEX[8 * i + j] / 8;
                let actual_j = BLACK_INDEX[8 * i + j] % 8;
                res_str.push_str(&format!(
                    "EvaluationScore({},{}),",
                    -self.params[idx + 16 * actual_i + 2 * actual_j].round() as isize,
                    -self.params[idx + 16 * actual_i + 2 * actual_j + 1].round() as isize
                ))
            }
            res_str.push_str("],");
        }
        res_str.push_str("]");
        res_str
    }

    pub fn get_norm(&self) -> f32 {
        let mut norm = 0.;
        for i in 0..self.params.len() {
            norm += self.params[i].powf(2.);
        }
        norm.sqrt()
    }

    pub fn pointwise_operation<F: Fn(f32) -> f32>(&mut self, f: F) {
        for i in 0..self.params.len() {
            self.params[i] = f(self.params[i]);
        }
    }
    pub fn add(&mut self, other: &Parameters, scale: f32) {
        for i in 0..self.params.len() {
            self.params[i] += other.params[i] * scale;
        }
    }
    pub fn scale(&mut self, scale: f32) {
        self.pointwise_operation(|x| scale * x);
    }
    pub fn square(&mut self) {
        self.pointwise_operation(|x| x * x);
    }
    pub fn sqrt(&mut self) {
        self.pointwise_operation(|x| x.sqrt())
    }
    pub fn add_scalar(&mut self, scalar: f32) {
        self.pointwise_operation(|x| x + scalar)
    }
    pub fn mul(&mut self, other: &Parameters) {
        for i in 0..self.params.len() {
            self.params[i] *= other.params[i];
        }
    }
    pub fn mul_inverse_other(&mut self, other: &Parameters) {
        for i in 0..self.params.len() {
            self.params[i] *= 1. / other.params[i];
        }
    }
}
impl Display for Parameters {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let mut res_str = String::new();
        res_str.push_str("use super::EvaluationScore;\n");
        res_str.push_str(&format!(
            "pub const SLIGHTLY_WINNING_NO_PAWN: f32 = {};\n",
            self.params[IDX_SLIGHTLY_WINNING_NO_PAWN],
        ));
        res_str.push_str(&format!(
            "pub const SLIGHTLY_WINNING_ENEMY_CAN_SAC: f32 = {};\n",
            self.params[IDX_SLIGHTLY_WINNING_ENEMY_CAN_SAC],
        ));
        res_str.push_str(&format!(
            "pub const TEMPO_BONUS{}",
            self.format_constant(IDX_TEMPO_BONUS),
        ));
        res_str.push_str(&format!(
            "pub const SHIELDING_PAWN_MISSING{}",
            self.format_constants(IDX_SHIELDING_PAWN_MISSING, SIZE_SHIELDING_PAWN_MISSING),
        ));
        res_str.push_str(&format!(
            "pub const SHIELDING_PAWN_MISSING_ON_OPEN_FILE{}",
            self.format_constants(
                IDX_SHIELDING_PAWN_ONOPEN_MISSING,
                SIZE_SHIELDING_PAWN_ONOPEN_MISSING
            ),
        ));
        res_str.push_str(&format!(
            "pub const PAWN_DOUBLED_VALUE{}",
            self.format_constant(IDX_PAWN_DOUBLED),
        ));
        res_str.push_str(&format!(
            "pub const PAWN_ISOLATED_VALUE{}",
            self.format_constant(IDX_PAWN_ISOLATED),
        ));
        res_str.push_str(&format!(
            "pub const PAWN_BACKWARD_VALUE{}",
            self.format_constant(IDX_PAWN_BACKWARD),
        ));
        res_str.push_str(&format!(
            "pub const PAWN_SUPPORTED_VALUE: [[EvaluationScore; 8];8] = {};\n",
            self.format_psqt(IDX_PAWN_SUPPORTED),
        ));
        res_str.push_str(&format!(
            "pub const PAWN_ATTACK_CENTER{}",
            self.format_constant(IDX_PAWN_ATTACK_CENTER),
        ));
        res_str.push_str(&format!(
            "pub const PAWN_MOBILITY{}",
            self.format_constant(IDX_PAWN_MOBILITY),
        ));
        res_str.push_str(&format!(
            "pub const PAWN_PASSED_VALUES{}",
            self.format_constants(IDX_PAWN_PASSED, SIZE_PAWN_PASSED),
        ));
        res_str.push_str(&format!(
            "pub const PAWN_PASSED_NOT_BLOCKED_VALUES{}",
            self.format_constants(IDX_PAWN_PASSED_NOTBLOCKED, SIZE_PAWN_PASSED_NOTBLOCKED),
        ));
        res_str.push_str(&format!(
            "pub const PASSED_KING_DISTANCE{}",
            self.format_constants(IDX_PAWN_PASSED_KINGDISTANCE, SIZE_PAWN_PASSED_KINGDISTANCE),
        ));
        res_str.push_str(&format!(
            "pub const PASSED_ENEMY_KING_DISTANCE{}",
            self.format_constants(
                IDX_PAWN_PASSED_ENEMYKINGDISTANCE,
                SIZE_PAWN_PASSED_ENEMYKINGDISTANCE
            ),
        ));

        res_str.push_str(&format!(
            "pub const PASSED_SUBTRACT_DISTANCE{}",
            self.format_constants(IDX_PAWN_PASSED_SUBDISTANCE, SIZE_PAWN_PASSED_SUBDISTANCE),
        ));
        res_str.push_str(&format!(
            "pub const ROOK_BEHIND_SUPPORT_PASSER{}",
            self.format_constant(IDX_ROOK_BEHIND_SUPPORT_PASSER),
        ));
        res_str.push_str(&format!(
            "pub const ROOK_BEHIND_ENEMY_PASSER{}",
            self.format_constant(IDX_ROOK_BEHIND_ENEMY_PASSER),
        ));
        res_str.push_str(&format!(
            "pub const PAWN_PASSED_WEAK{}",
            self.format_constant(IDX_PAWN_PASSED_WEAK),
        ));
        res_str.push_str(&format!(
            "pub const KNIGHT_SUPPORTED_BY_PAWN{}",
            self.format_constant(IDX_KNIGHT_SUPPORTED),
        ));
        res_str.push_str(&format!(
            "pub const KNIGHT_OUTPOST_TABLE: [[EvaluationScore; 8];8] = {};\n",
            self.format_psqt(IDX_KNIGHT_OUTPOST_TABLE),
        ));
        res_str.push_str(&format!(
            "pub const ROOK_ON_OPEN_FILE_BONUS{}",
            self.format_constant(IDX_ROOK_ON_OPEN),
        ));
        res_str.push_str(&format!(
            "pub const ROOK_ON_SEMI_OPEN_FILE_BONUS{}",
            self.format_constant(IDX_ROOK_ON_SEMI_OPEN),
        ));
        res_str.push_str(&format!(
            "pub const QUEEN_ON_OPEN_FILE_BONUS{}",
            self.format_constant(IDX_QUEEN_ON_OPEN),
        ));
        res_str.push_str(&format!(
            "pub const QUEEN_ON_SEMI_OPEN_FILE_BONUS{}",
            self.format_constant(IDX_QUEEN_ON_SEMI_OPEN),
        ));
        res_str.push_str(&format!(
            "pub const ROOK_ON_SEVENTH{}",
            self.format_constant(IDX_ROOK_ON_SEVENTH),
        ));
        res_str.push_str(&format!(
            "pub const PAWN_PIECE_VALUE{}",
            self.format_constant(IDX_PAWN_PIECE_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const KNIGHT_PIECE_VALUE{}",
            self.format_constant(IDX_KNIGHT_PIECE_VALUE),
        ));
        res_str.push_str("pub const KNIGHT_VALUE_WITH_PAWNS: [i16; 17] = [");
        for i in 0..17 {
            res_str.push_str(&format!(
                "{}, ",
                self.params[IDX_KNIGHT_VALUE_WITH_PAWN + i].round() as isize
            ));
        }
        res_str.push_str("];\n");
        res_str.push_str(&format!(
            "pub const BISHOP_PIECE_VALUE{}",
            self.format_constant(IDX_BISHOP_PIECE_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const BISHOP_PAIR_BONUS{}",
            self.format_constant(IDX_BISHOP_PAIR),
        ));
        res_str.push_str(&format!(
            "pub const ROOK_PIECE_VALUE{}",
            self.format_constant(IDX_ROOK_PIECE_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const QUEEN_PIECE_VALUE{}",
            self.format_constant(IDX_QUEEN_PIECE_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const DIAGONALLY_ADJACENT_SQUARES_WITH_OWN_PAWNS{}",
            self.format_constants(IDX_DIAGONALLY_ADJ_SQ_WPAWNS, SIZE_DIAGONALLY_ADJ_SQ_WPAWNS),
        ));
        res_str.push_str(&format!(
            "pub const KNIGHT_MOBILITY_BONUS{}",
            self.format_constants(IDX_KNIGHT_MOBILITY, SIZE_KNIGHT_MOBILITY),
        ));
        res_str.push_str(&format!(
            "pub const BISHOP_MOBILITY_BONUS{}",
            self.format_constants(IDX_BISHOP_MOBILITY, SIZE_BISHOP_MOBILITY),
        ));
        res_str.push_str(&format!(
            "pub const ROOK_MOBILITY_BONUS{}",
            self.format_constants(IDX_ROOK_MOBILITY, SIZE_ROOK_MOBILITY),
        ));
        res_str.push_str(&format!(
            "pub const QUEEN_MOBILITY_BONUS{}",
            self.format_constants(IDX_QUEEN_MOBILITY, SIZE_QUEEN_MOBILITY),
        ));
        res_str.push_str(&format!(
            "pub const ATTACK_WEIGHT{}",
            self.format_constants(IDX_ATTACK_WEIGHT, SIZE_ATTACK_WEIGHT),
        ));
        res_str.push_str(&format!(
            "pub const SAFETY_TABLE{}",
            self.format_constants(IDX_SAFETY_TABLE, SIZE_SAFETY_TABLE),
        ));
        res_str.push_str(&format!(
            "pub const KNIGHT_ATTACK_WORTH{}",
            self.format_constant(IDX_KNIGHT_ATTACK_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const BISHOP_ATTACK_WORTH{}",
            self.format_constant(IDX_BISHOP_ATTACK_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const ROOK_ATTACK_WORTH{}",
            self.format_constant(IDX_ROOK_ATTACK_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const QUEEN_ATTACK_WORTH{}",
            self.format_constant(IDX_QUEEN_ATTACK_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const KNIGHT_SAFE_CHECK{}",
            self.format_constant(IDX_KNIGHT_CHECK_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const BISHOP_SAFE_CHECK{}",
            self.format_constant(IDX_BISHOP_CHECK_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const ROOK_SAFE_CHECK{}",
            self.format_constant(IDX_ROOK_CHECK_VALUE),
        ));
        res_str.push_str(&format!(
            "pub const QUEEN_SAFE_CHECK{}",
            self.format_constant(IDX_QUEEN_CHECK_VALUE),
        ));
        res_str.push_str("pub const PSQT: [[[[EvaluationScore; 8]; 8]; 2]; 6] = [");
        for pt in PIECE_TYPES.iter() {
            res_str.push_str("[");
            res_str.push_str(&(self.format_psqt(IDX_PSQT + 128 * *pt as usize)));
            res_str.push_str(",");
            res_str.push_str(&(self.format_psqt_for_black(IDX_PSQT + 128 * *pt as usize)));
            res_str.push_str("],");
        }
        res_str.push_str("];\n");
        write!(formatter, "{}", res_str)
    }
}
