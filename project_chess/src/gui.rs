use crate::engine::GameState;
use crate::engine::logic::{GameStatus, Logic};
use crate::engine::pieces::Color;
use crate::engine::pieces::Piece;
use ::std::time::{Duration, Instant};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable;
use iced::widget::{button, column, container, image, row, text};
use iced::{Element, Font, Length, Task};

use rodio::OutputStreamBuilder;
use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::thread;

const SQUARE_SIZE: f32 = 75.0;

const TICK_RATE: Duration = Duration::from_millis(100);
#[derive(Default)]
pub struct ChessState {
    pub game: GameState,
    pub selected_square: Option<(u8, u8)>,
    pub valid_moves: Vec<(u8, u8)>,
    pub captured_by_white: Vec<Piece>,
    pub captured_by_black: Vec<Piece>,
    pub history_moves: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SquareClicked(u8, u8),
    Tick(Instant),
    RestartGame,
}
pub fn piece_path(piece: crate::engine::Piece) -> String {
    //daca vreau mai multe themes pentru piese, as putea sa adaug subfoldere in assets si modific ultimul string de formatat
    let color_piece = match piece.color {
        crate::engine::Color::White => "w",
        crate::engine::Color::Black => "b",
    };

    let type_piece = match piece.piece_type {
        crate::engine::PieceType::Pawn => "p",
        crate::engine::PieceType::Knight => "n",
        crate::engine::PieceType::Bishop => "b",
        crate::engine::PieceType::Rook => "r",
        crate::engine::PieceType::Queen => "q",
        crate::engine::PieceType::King => "k",
    };

    format!("assets/2/{}{}.png", color_piece, type_piece)
}

pub fn view(state: &ChessState) -> Element<'_, Message> {
    let status = state.game.get_game_status();
    let (status_text, status_color) = match status {
        crate::engine::logic::GameStatus::Ongoing => {
            (format!("{:?}'s turn", state.game.turn), iced::Color::BLACK)
        }
        crate::engine::logic::GameStatus::Check => (
            format!("{:?} is in check!", state.game.turn),
            iced::Color::from_rgb(0.9, 0.5, 0.0),
        ),
        crate::engine::logic::GameStatus::Checkmate => (
            format!("CHECKMATE, {:?} wins!", state.game.turn.opposite()),
            iced::Color::from_rgb(0.8, 0.0, 0.0),
        ),
        crate::engine::logic::GameStatus::Timeout(color_tmt) => (
            format!(
                "TIMEOUT, {:?} wins!, {:?} ran out of time",
                state.game.turn.opposite(),
                color_tmt
            ),
            iced::Color::from_rgb(0.8, 0.0, 0.0),
        ),
        crate::engine::logic::GameStatus::Stalemate => (
            format!("STALEMATE, {:?} wins!", state.game.turn.opposite()),
            iced::Color::from_rgb(0.8, 0.0, 0.0),
        ),
    };

    let status_widget: Element<Message> = container(
        text(status_text)
            .size(30)
            .color(iced::Color::WHITE)
            .font(Font::MONOSPACE)
            .align_x(Horizontal::Center),
    )
    .padding(10)
    .width(Length::Fill)
    .align_x(Horizontal::Center)
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(status_color)),
        border: iced::Border {
            radius: 6.0.into(),
            ..iced::Border::default()
        },
        ..container::Style::default()
    })
    .into();

    let restart_button = button(text("Restart Game").size(20).align_x(Horizontal::Center))
        .padding(10)
        .on_press(Message::RestartGame)
        .style(|_theme, status| {
            let bg = if status == button::Status::Hovered {
                iced::Color::from_rgb(0.9, 0.4, 0.4)
            } else {
                iced::Color::from_rgb(0.8, 0.3, 0.3)
            };

            button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    radius: 5.0.into(),
                    ..iced::Border::default()
                },
                ..button::Style::default()
            }
        });

    let mut board_col = column![].spacing(0);

    for rank in (0..8).rev() {
        let mut row_row = row![].spacing(0);

        for file in 0..8 {
            let idx = (rank * 8 + file) as usize;

            let content: Element<Message> = match state.game.board[idx] {
                Some(piece) => container(
                    image(piece_path(piece))
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .into(),
                None => text("").into(),
            };

            let is_selected = state.selected_square == Some((rank as u8, file as u8));
            let is_legal_move = state.valid_moves.contains(&(rank as u8, file as u8));

            let is_captured = if is_legal_move {
                match state.game.board[idx] {
                    Some(piece) => piece.color != state.game.turn,
                    None => false,
                }
            } else {
                false
            };

            let (bg_color, border_style) = if is_selected {
                (
                    iced::Color::from_rgb(0.2, 0.2, 0.9),
                    iced::Border {
                        color: iced::Color::from_rgb(0.1, 0.1, 0.6),
                        width: 4.0,
                        radius: 0.0.into(),
                    },
                )
            } else if is_legal_move {
                if is_captured {
                    (
                        iced::Color::from_rgb(0.9, 0.2, 0.2),
                        iced::Border {
                            color: iced::Color::from_rgb(0.6, 0.1, 0.1),
                            width: 4.0,
                            radius: 0.0.into(),
                        },
                    )
                } else {
                    (
                        iced::Color::from_rgb(0.5, 0.8, 0.5),
                        iced::Border {
                            color: iced::Color::from_rgb(0.3, 0.6, 0.3),
                            width: 4.0,
                            radius: 0.0.into(),
                        },
                    )
                }
            } else if (rank + file) % 2 != 0 {
                (
                    iced::Color::from_rgb(0.93, 0.93, 0.82),
                    iced::Border::default(),
                )
            } else {
                (
                    iced::Color::from_rgb(0.46, 0.58, 0.33),
                    iced::Border::default(),
                )
            };

            let square = button(
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
            )
            .width(SQUARE_SIZE)
            .height(SQUARE_SIZE)
            .on_press(Message::SquareClicked(rank as u8, file as u8))
            .style(move |_theme, _status| button::Style {
                background: Some(iced::Background::Color(bg_color)),
                border: border_style,
                ..button::Style::default()
            });

            row_row = row_row.push(square);
        }

        board_col = board_col.push(row_row);
    }
    let white_time_string = format_time(state.game.white_timer);
    let black_time_string = format_time(state.game.black_timer);

    let mut history_list = column![].spacing(5).width(Length::Fill);

    for move_string in state.history_moves.iter().rev() {
        history_list = history_list.push(
            text(move_string)
                .size(18)
                .color(iced::Color::from_rgb(0.8, 0.8, 0.8)),
        );
    }

    let history_widget = container(scrollable(history_list).height(Length::FillPortion(8)))
        .padding(10)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.15, 0.15, 0.15,
            ))),
            border: iced::Border {
                radius: 5.0.into(),
                width: 1.0,
                color: iced::Color::from_rgb(0.3, 0.3, 0.3),
            },
            ..Default::default()
        });

    let black_timer = container(
        row![
            container(captured_pieces_row(&state.captured_by_black))
                .width(Length::Fill)
                .align_x(Horizontal::Left),
            text(format!("Black: {}", black_time_string))
                .size(25)
                .color(iced::Color::WHITE),
        ]
        .align_y(Vertical::Center),
    )
    .padding(10)
    .width(SQUARE_SIZE * 8.0)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb(
            0.2, 0.2, 0.2,
        ))),
        border: iced::Border {
            radius: 6.0.into(),
            ..iced::Border::default()
        },
        ..container::Style::default()
    });

    let white_timer = container(
        row![
            container(captured_pieces_row(&state.captured_by_white))
                .width(Length::Fill)
                .align_x(Horizontal::Left),
            text(format!("White: {}", white_time_string))
                .size(25)
                .color(iced::Color::BLACK),
        ]
        .align_y(Vertical::Center),
    )
    .padding(10)
    .width(SQUARE_SIZE * 8.0)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb(
            0.9, 0.9, 0.9,
        ))),
        border: iced::Border {
            color: iced::Color::BLACK,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..container::Style::default()
    });

    //asta e tot ce e de randat
    let content = row![
        //partea stanga
        column![
            container(black_timer)
                .width(Length::Fill)
                .align_x(Horizontal::Right),
            board_col,
            container(white_timer)
                .width(Length::Fill)
                .align_x(Horizontal::Right)
        ]
        .spacing(10)
        .align_x(Horizontal::Right)
        .width(Length::FillPortion(2)),
        //partea dreapta
        column![status_widget, history_widget, restart_button]
            .height(Length::Fill)
            .spacing(20)
            .align_x(Horizontal::Right)
    ]
    .padding(20)
    .spacing(5)
    .align_y(Vertical::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}

pub fn update(state: &mut ChessState, message: Message) -> Task<Message> {
    match message {
        Message::SquareClicked(clicked_rank, clicked_file) => {
            if let Some(start) = state.selected_square {
                let end = (clicked_rank, clicked_file);
                let destination_idx = (end.0 * 8 + end.1) as usize;

                let target_piece = state.game.board[destination_idx];
                let moving_piece = state.game.board[(start.0 * 8 + start.1) as usize];
                let capture_target = state.game.board[destination_idx];

                match state.game.make_move(start, end, None) {
                    Ok(move_str) => {
                        println!("Move Successful: {}", move_str);

                        if let Some(p) = moving_piece {
                            if p.piece_type == crate::engine::PieceType::King
                                && (start.1 as i8 - end.1 as i8).abs() == 2
                            {
                                play_sound("castle.mp3");
                            } else if target_piece.is_some() {
                                play_sound("capture.mp3");
                            } else {
                                play_sound("move-self.mp3");
                            }
                        }

                        state.history_moves.push(move_str);

                        if let Some(captured_piece) = capture_target {
                            match state.game.turn.opposite() {
                                Color::White => state.captured_by_white.push(captured_piece),
                                Color::Black => state.captured_by_black.push(captured_piece),
                            }
                        }

                        match state.game.get_game_status() {
                            GameStatus::Check => play_sound("move-check.mp3"),
                            GameStatus::Checkmate => play_sound("game-win.mp3"),
                            GameStatus::Stalemate => play_sound("game-draw.mp3"),
                            _ => {}
                        }

                        state.selected_square = None;
                        state.valid_moves.clear();
                    }
                    Err(chess_error) => {
                        println!("Move Rejected: {:?}", chess_error);

                        let idx = (clicked_rank * 8 + clicked_file) as usize;
                        if let Some(piece) = state.game.board[idx]
                            && piece.color == state.game.turn
                        {
                            state.selected_square = Some((clicked_rank, clicked_file));
                            state.valid_moves =
                                state.game.get_legal_dest(clicked_rank, clicked_file);
                        } else {
                            play_sound("illegal.mp3");
                            state.selected_square = None;
                            state.valid_moves.clear();
                        }
                    }
                }
            } else {
                let idx = (clicked_rank * 8 + clicked_file) as usize;
                if let Some(piece) = state.game.board[idx] {
                    if piece.color == state.game.turn {
                        state.selected_square = Some((clicked_rank, clicked_file));
                        state.valid_moves = state.game.get_legal_dest(clicked_rank, clicked_file); //
                    } else {
                        println!(
                            "Cannot select {:?} - it is {:?}'s turn!",
                            piece.color, state.game.turn
                        );
                        play_sound("illegal.mp3");
                    }
                }
            }
        }
        Message::Tick(_) => {
            let status = state.game.get_game_status();

            if matches!(status, GameStatus::Ongoing | GameStatus::Check) {
                match state.game.turn {
                    Color::White => state.game.white_timer -= 0.1,
                    Color::Black => state.game.black_timer -= 0.1,
                }
            } else {
                state.game.game_active = false;
            }
        }
        Message::RestartGame => {
            state.game = GameState::new();

            state.selected_square = None;
            state.valid_moves.clear();
            state.captured_by_black.clear();
            state.captured_by_white.clear();
            state.history_moves.clear();

            println!("Game restarted");
        }
    }
    Task::none()
}

//functie care o data la 100 de milisecunde da update la tick
pub fn subscription(state: &ChessState) -> iced::Subscription<Message> {
    if state.game.game_active {
        iced::time::every(TICK_RATE).map(Message::Tick)
    } else {
        iced::Subscription::none()
    }
}

fn format_time(seconds: f32) -> String {
    let s = seconds.max(0.0) as u32;
    let min = s / 60;
    let sec = s % 60;
    format!("{:02}:{:02}", min, sec)
}

fn captured_pieces_row(pieces: &[Piece]) -> Element<'_, Message> {
    row(pieces
        .iter()
        .map(|&p| image(piece_path(p)).width(20).height(20).into()))
    .spacing(2)
    .into()
}

pub fn play_sound(file_name: &str) {
    let path = format!("sound_assets/standard/{}", file_name);

    thread::spawn(move || {
        let stream_result = OutputStreamBuilder::open_default_stream().ok();

        if let Some(stream_handle) = stream_result {
            let mixer = stream_handle.mixer();

            if let Ok(file) = File::open(&path) {
                let sink = Sink::connect_new(mixer);

                if let Ok(source) = Decoder::new(BufReader::new(file)) {
                    sink.append(source);
                    sink.sleep_until_end();
                }
            } else {
                eprintln!("Could not find file: {}", path);
            }
        } else {
            eprintln!("Warning: No audio output device available.");
        }
    });
}
