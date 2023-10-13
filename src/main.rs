

use iced::{alignment, Application, Color, Command, Element, executor, Length, mouse, Point, Rectangle, Renderer, Settings, Theme, Vector};
use iced::widget::canvas::{Geometry, Cache, Path, path};
use iced::widget::{canvas, Canvas, column, container, scrollable, text};
use std::default::Default;
use std::fmt::{Debug, Formatter};
use std::time::Duration;

fn main() -> iced::Result {
    RobotChallenge::run(Settings::default())
}

struct RobotChallenge {
    round: u16,
    next_tank_index: Vec<usize>,
    board: Board,
    board_cache : Cache,
}

#[derive(Debug, Clone)]
enum Message {
    NewGame(Result<String, SleeperError>),
    NewRound(Result<String, SleeperError>),
    Move(Result<String, SleeperError>),
}

impl Application for RobotChallenge {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                round: 0,
                board: Default::default(),
                next_tank_index: vec![],
                board_cache : Default::default(),
            },
            Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::NewGame)
        )
    }

    fn title(&self) -> String {
        "Robot Challenge".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::NewGame(_) => {
                println!("NewGame");
                // TODO: Setup a new game.
                Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::NewRound)
            }
            Message::NewRound(_) => {
                println!("NewRound");
                self.round += 1;
                // TODO: Randomize next tank index
                for index in 0..self.board.tanks.len() {
                    self.next_tank_index.push(index);
                                 }
                Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Move)
            }
            Message::Move(_) => {
                println!("Move");
                if self.next_tank_index.is_empty() {
                    // TODO: Trigger new round
                    Command::none()
                } else {
                    let index = self.next_tank_index.pop().unwrap();
                    println!("{:?}",self.board.tanks.get(index).unwrap());
                    Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Move)
                }

            }
        }
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

struct Sleeper;

impl Sleeper {
    async fn sleep(duration: Duration) -> Result<String, SleeperError>{
        std::thread::sleep(duration);
        Ok("Booing".to_string())
    }
}

#[derive(Debug, Clone)]
enum SleeperError {
    String,
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

            // Draw game board background.
            let background = Path::rectangle(Point::new(0.0, 0.0), frame.size());
            frame.fill(&background, GameColors::LIGHT_GRAY);

            // Draw tanks.
            let tank_path = tank_path();

            for tank in &self.board.tanks {
                frame.with_save(|frame| {
                    let x = (tank.point.x * 20) as f32;
                    let y = (tank.point.y * 20) as f32;
                    frame.translate(Vector::new(x, y));
                    frame.fill( &tank_path, tank.color);
                });
            }

            // TODO: Tank hit. An X over a tank if hit.
            // TODO: Laser. A line from the shooting tank.

        });

        vec![board]
    }
}

struct GameColors;

impl GameColors {
    const LIGHT_GRAY: Color = Color {
        r: 0.824, // 0xD3
        g: 0.824, // 0xD3
        b: 0.824, // 0xD3
        a: 1.0,
    };

    const GREEN: Color = Color {
        r: 0.0,
        g: 1.0, // 0xFF
        b: 0.0,
        a: 1.0,
    };

    const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

// Tank Path in the shape of an arrow.
fn tank_path() -> Path {
    let mut builder = path::Builder::new();
    builder.move_to(Point::new(9.0, 4.0));
    builder.line_to(Point::new(11.0, 4.0));
    builder.line_to(Point::new(16.0, 10.0));
    builder.line_to(Point::new(12.0, 10.0));
    builder.line_to(Point::new(12.0, 16.0));
    builder.line_to(Point::new(8.0, 16.0));
    builder.line_to(Point::new(8.0, 10.0));
    builder.line_to(Point::new(4.0, 10.0));
    builder.line_to(Point::new(9.0, 4.0));
    builder.build()
}

// TODO: #[derive(Debug)] Implement Debug in Strategy
struct Board {
    dimension: Dimension,
    tanks : Vec<Tank>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            dimension: Default::default(),
            tanks: vec![
                Tank{
                    color: GameColors::RED,
                    point: BoardPoint { x: 5, y: 15 },
                    ..Default::default()
                },
                Tank{
                    color: GameColors::BLUE,
                    point: BoardPoint { x: 15, y: 5 },
                    ..Default::default()
                }],
            
        }
    }
}

struct Tank {
    strategy: Box<dyn Strategy>,
    color: Color,
    energy: usize,
    hits: usize,
    frags: usize,
    point: BoardPoint, // Set to random available Point when adding to Board.
    direction: Direction, // Set to random direction when adding to Board.
}

impl Debug for Tank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tank(strategy: {}, energy: {}, hits: {}, frags: {:?}, {:?}, direction: {:?}", self.strategy.name(), self.energy, self.hits, self.frags, self.point, self.direction)
    }
}

impl Default for Tank {
    fn default() -> Self {
        Self {
            strategy: Box::new(Dummy::default()),
            color: GameColors::GREEN,
            energy: Self::MAX_ENERGY,
            hits: 0,
            frags: 0,
            point: Default::default(),
            direction: Direction::North,
        }
    }
}

impl Tank {
    const MAX_ENERGY: usize = 5;
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

impl Default for Dimension {
    fn default() -> Self {
        Self { width: 20, height: 20 }
    }
}

#[derive(Debug, Clone)]
struct BoardPoint {
    x: usize,
    y: usize,
}

impl Default for BoardPoint {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Status {
    direction: Direction,
    location: BoardPoint,
    is_alive: bool,
}

#[derive(Debug)]
struct NextMoveInput {
    game_board: Dimension,
    own_status: Status,
    opponent_status: Vec<Status>,
    fire_range: usize,
}

trait Strategy {
    fn name(&self) -> String;
    fn author(&self) -> String;
    fn next_move(&mut self, input: NextMoveInput) -> &Move;
}

#[derive(Debug, Clone)]
struct Dummy {
    moves: Vec<Move>,
    move_index: usize
}

impl Default for Dummy {
    fn default() -> Self {
        Self {
            moves: vec![Move::Fire, Move::TurnLeft, Move::Forward],
            move_index: 0
        }
    }
}

impl Strategy for Dummy {
    fn name(&self) -> String {
        "Dummy".to_string()
    }

    fn author(&self) -> String {
        "JMH".to_string()
    }

    fn next_move(&mut self, input: NextMoveInput) -> &Move {
        let next_move = self.moves.get(self.move_index).unwrap();
        self.move_index = (self.move_index + 1) % self.moves.len();
        next_move
    }
}

