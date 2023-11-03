

use iced::{
    Application,
    Settings
};
use crate::game::RobotChallenge;


mod strategies;
mod game;


fn main() -> iced::Result {
    RobotChallenge::run(Settings::default())
}

