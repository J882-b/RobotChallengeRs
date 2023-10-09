use iced::{alignment, Application, Color, Command, Element, executor, Length, Settings, Theme};
use iced::widget::{column, container, scrollable, text};

fn main() -> iced::Result {
    RobotChallenge::run(Settings::default())
}

struct RobotChallenge {
    strategies: Vec<Strategy>,
    round: usize,
}

struct Strategy {
    name: String,
    author: String,
}

#[derive(Debug, Clone)]
enum Message {
    NewGame,
    NewRound,
    Move,
    Shoot,
    Hit,
}

impl Application for RobotChallenge {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            RobotChallenge {
                strategies: vec!(),
                round: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Robot Challenge".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::NewGame => {}
            Message::NewRound => {}
            Message::Move => {}
            Message::Shoot => {}
            Message::Hit => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {

        let game_title = text("Game area")
            .width(Length::Fill)
            .size(30)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);

        // TODO: Game board

        let score_title = text("Score area")
            .width(Length::Fill)
            .size(30)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);

        // TODO: Score listing

        let content = column![game_title, score_title]
            .spacing(20);

        scrollable(
            container(content)
                .width(Length::Fill)
                .padding(10)
                .center_x()
        )
            .into()
    }
}
