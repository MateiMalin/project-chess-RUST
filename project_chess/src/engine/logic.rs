//here i ll have the moves, check/checkmate logic etc
use super::{Color, GameState, PieceType};

#[derive(Debug)]
pub enum ChessError {
    IllegalMove,
    NotYourTurn,
    OutOfBounds,
    EmptySquare,
}
pub enum GameStatus {
    Ongoing,
    Check,
    Checkmate,
    Stalemate,
    Timeout(crate::engine::Color),
}

#[derive(Debug, Clone, Copy)]
pub enum Sound {
    Move,
    Capture,
    Castle,
    Win,
    Check,
}

pub trait Logic {
    fn is_on_board(rank: u8, file: u8) -> bool;
    fn get_legal_dest(&self, rank: u8, file: u8) -> Vec<(u8, u8)>;
    fn make_move(
        &mut self,
        start: (u8, u8),
        dest: (u8, u8),
        promotion: Option<PieceType>,
    ) -> Result<String, ChessError>;
    fn get_game_status(&self) -> GameStatus;
}

impl Logic for GameState {
    fn get_game_status(&self) -> GameStatus {
        if self.white_timer <= 0.0 {
            return GameStatus::Timeout(Color::White);
        }
        if self.black_timer <= 0.0 {
            return GameStatus::Timeout(Color::Black);
        }
        let mut has_legal_moves = false;

        for idx in 0..64 {
            if let Some(p) = self.board[idx]
                && p.color == self.turn
                && !self.generate_valid_moves(idx).is_empty()
            {
                has_legal_moves = true;
                break;
            }
        }

        if has_legal_moves {
            if self.is_in_check(self.turn) {
                return GameStatus::Check;
            }
            return GameStatus::Ongoing;
        }

        if self.is_in_check(self.turn) {
            GameStatus::Checkmate
        } else {
            GameStatus::Stalemate
        }
    }

    fn is_on_board(rank: u8, file: u8) -> bool {
        rank < 8 && file < 8
    }

    //asta imi da destinatiile in format rank,file
    fn get_legal_dest(&self, rank: u8, file: u8) -> Vec<(u8, u8)> {
        let idx = (rank * 8 + file) as usize;

        let pseudo_moves = self.generate_valid_moves(idx);

        pseudo_moves
            .iter()
            .map(|&i| ((i / 8) as u8, (i % 8) as u8))
            .collect()
    }

    fn make_move(
        &mut self,
        start: (u8, u8),
        end: (u8, u8),
        promotion: Option<PieceType>,
    ) -> Result<String, ChessError> {
        let start_idx = (start.0 * 8 + start.1) as usize;
        let end_idx = (end.0 * 8 + end.1) as usize;

        if !<Self as Logic>::is_on_board(start.0, start.1)
            || !<Self as Logic>::is_on_board(end.0, end.1)
        {
            return Err(ChessError::OutOfBounds);
        }

        let piece_ref = self.board[start_idx]
            .as_ref()
            .ok_or(ChessError::EmptySquare)?;

        if piece_ref.color != self.turn {
            return Err(ChessError::NotYourTurn);
        }

        let moves = self.generate_valid_moves(start_idx);
        if !moves.contains(&end_idx) {
            return Err(ChessError::IllegalMove);
        }

        let piece = self.board[start_idx]
            .take()
            .ok_or(ChessError::EmptySquare)?;

        if piece.piece_type == PieceType::Pawn
            && let Some(target) = self.en_passant_target
            && end_idx == target
        {
            let poz = if piece.color == Color::White {
                8
            } else {
                -8i32
            };
            let target_idx = (end_idx as i32 - poz) as usize;
            self.board[target_idx] = None;
        }

        self.board[end_idx] = Some(piece);

        if piece.piece_type == PieceType::Pawn
            && ((start.0 == 1 && end.0 == 3) || (start.0 == 6 && end.0 == 4))
        {
            self.en_passant_target = Some((start_idx + end_idx) / 2);
        } else {
            self.en_passant_target = None;
        }

        if let Some(mut p) = self.board[end_idx]
            && p.piece_type == PieceType::Pawn
            && (end.0 == 0 || end.0 == 7)
        {
            let new_type = promotion.unwrap_or(PieceType::Queen);
            p.piece_type = match new_type {
                PieceType::Rook | PieceType::Bishop | PieceType::Knight => new_type,
                _ => PieceType::Queen,
            };
            self.board[end_idx] = Some(p);
        }

        if piece.piece_type == PieceType::King {
            let diff = (start.1 as i8 - end.1 as i8).abs();
            let king_idx = (start.0 * 8 + start.1) as usize;
            if diff == 2 {
                if end.1 == 6 {
                    //king side castling
                    let rook = self.board[king_idx + 3].take();
                    self.board[king_idx + 1] = rook;
                } else if end.1 == 2 {
                    //queen side castling
                    let rook = self.board[king_idx - 4].take();
                    self.board[king_idx - 1] = rook;
                }
            }
        }

        if piece.piece_type == PieceType::King {
            if piece.color == Color::White {
                self.castling_rights &= 0b0011;
            } else {
                self.castling_rights &= 0b1100;
            }
        }

        if piece.piece_type == PieceType::Rook {
            match start_idx {
                0 => self.castling_rights &= 0b1011,
                7 => self.castling_rights &= 0b0111,
                56 => self.castling_rights &= 0b1110,
                63 => self.castling_rights &= 0b1101,
                _ => {}
            }
        }

        //if rook captured
        match end_idx {
            0 => self.castling_rights &= 0b1011,
            7 => self.castling_rights &= 0b0111,
            56 => self.castling_rights &= 0b1110,
            63 => self.castling_rights &= 0b1101,
            _ => {}
        }

        self.turn = self.turn.opposite();

        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        Ok(format!(
            "{}{}-{}{}",
            files[start.1 as usize],
            start.0 + 1,
            files[end.1 as usize],
            end.0 + 1
        ))
    }
}

impl GameState {
    //here ill generate all the pseudo moves that each piece type can have
    pub fn generate_all_moves(&self, idx: usize, include_castling: bool) -> Vec<usize> {
        let mut moves = Vec::new();

        let piece = match self.board[idx] {
            Some(p) => p,
            None => return moves,
        };

        let rank = idx / 8;
        let file = idx % 8;

        let rook_dirs = [(-1, 0), (1, 0), (0, 1), (0, -1)];
        let knight_dirs = [
            (-1, -2),
            (-2, -1),
            (2, 1),
            (1, 2),
            (1, -2),
            (2, -1),
            (-1, 2),
            (-2, 1),
        ];
        let king_dirs = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (1, 0),
            (1, 1),
            (0, 1),
            (1, -1),
            (0, -1),
        ];
        let bishop_dirs = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

        match piece.piece_type {
            PieceType::Rook => {
                self.add_slide_mechanic(&mut moves, rank as i8, file as i8, &rook_dirs, piece.color)
            }
            PieceType::Bishop => self.add_slide_mechanic(
                &mut moves,
                rank as i8,
                file as i8,
                &bishop_dirs,
                piece.color,
            ),
            PieceType::Queen => {
                self.add_slide_mechanic(
                    &mut moves,
                    rank as i8,
                    file as i8,
                    &bishop_dirs,
                    piece.color,
                );
                self.add_slide_mechanic(
                    &mut moves,
                    rank as i8,
                    file as i8,
                    &rook_dirs,
                    piece.color,
                );
            }
            PieceType::Knight => {
                for (dr, df) in knight_dirs {
                    self.can_i_land(
                        &mut moves,
                        (rank as i8) + dr,
                        (file as i8) + df,
                        piece.color,
                    );
                }
            }
            PieceType::King => {
                for (dr, df) in king_dirs {
                    self.can_i_land(
                        &mut moves,
                        (rank as i8) + dr,
                        (file as i8) + df,
                        piece.color,
                    );
                }

                if include_castling {
                    //punem logica de castling
                    let king_idx = idx;
                    if !self.is_square_attacked(king_idx, piece.color.opposite()) {
                        let (k_mask, q_mask) = if piece.color == Color::White {
                            (0b1000, 0b0100)
                        } else {
                            (0b0010, 0b0001)
                        };

                        if (self.castling_rights & k_mask) != 0 {
                            //inseamna ca putem sa facem castling pe partea cu regele
                            let f_idx = king_idx + 1;
                            let g_idx = king_idx + 2;
                            if self.board[f_idx].is_none()
                                && self.board[g_idx].is_none()
                                && !self.is_square_attacked(f_idx, piece.color.opposite())
                                && !self.is_square_attacked(g_idx, piece.color.opposite())
                            {
                                moves.push(g_idx);
                            }
                        }
                        if (self.castling_rights & q_mask) != 0 {
                            //putem sa facem castling pe partea cu regina, yay
                            let b_idx = king_idx - 3;
                            let c_idx = king_idx - 2;
                            let d_idx = king_idx - 1;

                            if self.board[b_idx].is_none()
                                && self.board[c_idx].is_none()
                                && self.board[d_idx].is_none()
                                && !self.is_square_attacked(b_idx, piece.color.opposite())
                                && !self.is_square_attacked(c_idx, piece.color.opposite())
                                && !self.is_square_attacked(d_idx, piece.color.opposite())
                            {
                                moves.push(c_idx);
                            }
                        }
                    }
                }
            }

            PieceType::Pawn => {
                let start_rank = if piece.color == Color::White { 1 } else { 6 };
                let forward: i8 = if piece.color == Color::White { 1 } else { -1 };

                let r1 = (rank as i8) + forward;
                if (0..8).contains(&r1) {
                    let idx1 = (r1 as usize) * 8 + file;
                    if self.board[idx1].is_none() {
                        moves.push(idx1);

                        if rank == start_rank {
                            let r2 = (rank as i8) + 2 * forward;
                            let idx2 = (r2 as usize) * 8 + file;
                            if self.board[idx2].is_none() {
                                moves.push(idx2);
                            }
                        }
                    }
                }

                for df in [-1, 1] {
                    let r_cap = (rank as i8) + forward;
                    let f_cap = (file as i8) + df;
                    if (0..8).contains(&r_cap) && (0..8).contains(&f_cap) {
                        let idx_cap = (r_cap as usize) * 8 + (f_cap as usize);
                        if let Some(target) = self.board[idx_cap] {
                            if target.color != piece.color {
                                moves.push(idx_cap);
                            }
                        } else if let Some(ep_target) = self.en_passant_target
                            && idx_cap == ep_target
                        {
                            moves.push(idx_cap);
                        }
                    }
                }
            }
        }
        moves
    }

    //aici is toate miscarile, also filtrate de check pentru rege
    pub fn generate_valid_moves(&self, idx: usize) -> Vec<usize> {
        let pseudo_moves = self.generate_all_moves(idx, true);
        let mut valid_moves = Vec::new();

        for target in pseudo_moves {
            let mut simulation = self.clone();

            let p = simulation.board[idx].take();
            simulation.board[target] = p;

            if !simulation.is_in_check(self.turn) {
                valid_moves.push(target);
            }
        }
        valid_moves
    }
    //logic for the queen, bishop and the rook
    fn add_slide_mechanic(
        &self,
        moves: &mut Vec<usize>,
        r: i8,
        f: i8,
        dirs: &[(i8, i8)],
        color: Color,
    ) {
        for &(dr, df) in dirs {
            for i in 1..8 {
                let new_r = r + dr * i;
                let new_f = f + df * i;

                if !(0..8).contains(&new_r) || !(0..8).contains(&new_f) {
                    break;
                }

                let idx = (new_r as usize) * 8 + (new_f as usize);
                match self.board[idx] {
                    None => {
                        moves.push(idx);
                    }
                    Some(piece) => {
                        if piece.color != color {
                            moves.push(idx);
                        }
                        break;
                    }
                }
            }
        }
    }

    //logic for the knight and the king
    fn can_i_land(&self, moves: &mut Vec<usize>, r: i8, f: i8, color: Color) {
        if (0..8).contains(&r) && (0..8).contains(&f) {
            let idx = (r as usize) * 8 + (f as usize);
            match self.board[idx] {
                None => moves.push(idx),
                Some(piece) => {
                    if piece.color != color {
                        moves.push(idx);
                    }
                }
            }
        }
    }

    //bool function so i can see if the king is in check
    fn is_in_check(&self, color: Color) -> bool {
        for idx in 0..64 {
            if let Some(piece) = self.board[idx]
                && piece.color == color
                && piece.piece_type == PieceType::King
                && self.is_square_attacked(idx, color.opposite())
            {
                return true;
            }
        }
        false
    }

    fn is_square_attacked(&self, square_idx: usize, attacker_color: Color) -> bool {
        for idx in 0..64 {
            if let Some(piece) = self.board[idx]
                && piece.color == attacker_color
            {
                let moves = self.generate_all_moves(idx, false);

                if moves.contains(&square_idx) {
                    return true;
                }
            }
        }
        false
    }
}
