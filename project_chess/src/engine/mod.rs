pub use pieces::{Color, Piece, PieceType};

// use crate::engine::logic::Logic;
#[derive(Clone, Debug)]
pub struct GameState {
    pub board: [Option<Piece>; 64],
    pub turn: Color,

    pub en_passant_target: Option<usize>,
    pub castling_rights: u8,

    pub white_timer: f32,
    pub black_timer: f32,
    pub game_active: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut game = GameState {
            board: [None; 64],
            turn: Color::White,
            en_passant_target: None,
            castling_rights: 0b1111,
            white_timer: 300.0,
            black_timer: 300.0,
            game_active: true,
        };

        game.setup_initial_position();
        game
    }

    fn setup_initial_position(&mut self) {
        self.board[0] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        });
        self.board[1] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        });
        self.board[2] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        });
        self.board[3] = Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        });
        self.board[4] = Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        });
        self.board[5] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        });
        self.board[6] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        });
        self.board[7] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        });

        for i in 8..16 {
            self.board[i] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            });
        }

        self.board[56] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        });
        self.board[57] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        });
        self.board[58] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        });
        self.board[59] = Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        });
        self.board[60] = Some(Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        });
        self.board[61] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        });
        self.board[62] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        });
        self.board[63] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        });

        for i in 48..56 {
            self.board[i] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            });
        }
    }

    pub fn print_board(&self) {
        for rank in (0..8).rev() {
            print!("{}", rank + 1);
            print!(" ");
            for file in 0..8 {
                let idx = (rank * 8 + file) as usize;
                match self.board[idx] {
                    Some(piece) => {
                        print!("{}", Piece::symbol(&piece));
                        print!(" ");
                    }
                    None => {
                        if rank % 2 == file % 2 {
                            print!("■ ") //asta e black
                        } else {
                            print!("□ ") //asta e white
                        }
                    }
                }
            }
            println!();
        }
        println!("  a b c d e f g h");
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

pub mod logic;
pub mod pieces;
