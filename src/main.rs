use iced::{alignment, Application, Color, Command, Element, executor, Length, mouse, Point, Rectangle, Renderer, Settings, Size, Theme};
use iced::widget::canvas::{Geometry, Cache, Path, Stroke, stroke, LineCap};
use iced::widget::{canvas, Canvas, column, container, scrollable, text};

fn main() -> iced::Result {
    RobotChallenge::run(Settings::default())
}

struct RobotChallenge {
    round: usize,
    max_energy: usize,
    tanks : Vec<Tank>,
    board_cache : Cache,
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
                round: 0,
                max_energy: 5,
                tanks: vec!(), // TODO: Create tanks with strategy.
                board_cache : Default::default(),
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

        let game_board : Canvas<&RobotChallenge, Message> = canvas(self as &Self)
            .width(400)
            .height(400);

        let score_title = text("Score area")
            .width(Length::Fill)
            .size(30)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);

        // TODO: Score listing

        let content = column![game_title, game_board, score_title]
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

impl<Message> canvas::Program<Message, Renderer> for RobotChallenge {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let board = self.board_cache.draw(renderer, bounds.size(), |frame| {
            let radius = frame.width().min(frame.height()) / 2.0;

            // TODO: Draw game board background.
            let background = Path::rectangle(Point::new(0.0, 0.0), frame.size());
            frame.fill(&background, Color::from_rgb8(0xC2, 0x25, 0xA2 ));

            // TODO: Draw tanks.


        });

        vec![board]
    }
}

// TODO: Implement Debug
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
struct BoardPoint {
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
    location: BoardPoint,
    is_alive: bool,
}

#[derive(Debug, Clone)]
struct Input {
    game_board: Dimension,
    own_status: Status,
    opponent_status: Vec<Status>,
    fire_range: usize,
}

trait Strategy {
    fn name(&self) -> String;
    fn author(&self) -> String;
    fn next_move(&mut self, input: Input) -> &Move;
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

    fn next_move(&mut self, input: Input) -> &Move {
        let next_move = self.moves.get(self.move_index).unwrap();
        self.move_index = (self.move_index + 1) % self.moves.len();
        next_move
    }
}

