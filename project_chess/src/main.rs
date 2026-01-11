pub mod engine;
pub mod gui;

use gui::{ChessState, Message, update, view};
use iced::Task;

fn main() -> iced::Result {
    iced::application(init, update, view)
        .title("Chess Engine")
        .subscription(gui::subscription)
        .theme(style)
        .run()
}

fn init() -> (ChessState, Task<Message>) {
    (ChessState::default(), Task::none())
}

fn style(_state: &ChessState) -> iced::Theme {
    iced::Theme::Dark
}
