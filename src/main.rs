use iced::{alignment, Application, Color, Command, Element, executor, Length, Settings, Theme};
use iced::widget::{column, container, scrollable, text};

fn main() -> iced::Result {
    RobotChallenge::run(Settings::default())
}

struct RobotChallenge {
    tanks: Vec<Tank>,
    round: usize,
    max_energy: usize,
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
                tanks: vec!(),
                round: 0,
                max_energy: 5
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

struct Tank {
    strategy: Box<dyn Strategy>,
    energy: usize,
    hits: usize,
    frags: usize,
    point: Point, // Init to random available Point.
    direction: Direction, // Init to random direction.
}

#[derive(Debug, Clone)]
enum Move {
    Fire,
    TurnLeft,
    Forward,
    TurnRight,
    Wait,
}

#[derive(Debug, Clone)]
struct Dimension {
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Status {
    direction: Direction,
    location: Point,
    is_alive: bool,
}

#[derive(Debug, Clone)]
struct Input {
    playfield: Dimension,
    own_status: Status,
    opponent_status: Vec<Status>,
    fire_range: usize,
}

trait Strategy {
    fn name(&self) -> String;
    fn author(&self) -> String;
    fn next_move(&mut self, input: Input) -> Move;
}

#[derive(Debug, Clone)]
struct Dummy {
    name: String,
    author: String,
    moves: Vec<Move>,
    move_index: usize
}

impl Dummy {
    fn new() -> Self {
        Self {
            name: "Dummy".to_string(),
            author: "JMH".to_string(),
            moves: vec![Move::Fire, Move::TurnLeft, Move::Forward],
            move_index: 0
        }
    }
}
impl Strategy for Dummy {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn author(&self) -> String {
        self.author.clone()
    }

    fn next_move(&mut self, input: Input) -> Move {
        let next_move = self.moves.get(self.move_index).unwrap().clone();
        self.move_index = (self.move_index + 1) % self.moves.len();
        next_move
    }
}

