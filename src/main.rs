

use iced::{alignment, Application, Color, Command, Element, executor, Length, mouse, Point, Rectangle, Renderer, Settings, Theme, Vector};
use iced::widget::canvas::{Geometry, Cache, Path, path};
use iced::widget::{canvas, Canvas, column, container, scrollable, text};
use std::default::Default;
use std::fmt::{Debug, Formatter};
use std::time::Duration;
use iced::widget::canvas::path::lyon_path::geom::Angle;
use iced::widget::canvas::path::lyon_path::geom::euclid::Transform2D;

fn main() -> iced::Result {
    RobotChallenge::run(Settings::default())
}

struct RobotChallenge {
    round: usize,
    next_tank_index: Vec<usize>,
    board: Board,
    board_cache : Cache,
}

impl RobotChallenge {
    const MAX_ROUNDS: usize = 20;
}

#[derive(Debug, Clone)]
enum Message {
    NewGame(Result<String, SleeperError>),
    NewRound(Result<String, SleeperError>),
    Move(Result<String, SleeperError>),
    Laser(Result<String, SleeperError>),
    Hit(Result<String, SleeperError>),
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
                self.round += 1;
                println!("NewRound {}", self.round);
                if self.round >= RobotChallenge::MAX_ROUNDS {
                    Command::none()
                } else {
                    // TODO: Randomize next tank index
                    for index in 0..self.board.tanks.len() {
                        self.next_tank_index.push(index);
                    }
                    Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Move)
                }
            }
            Message::Move(_) => {
                if self.next_tank_index.is_empty() {
                    Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::NewRound)
                } else {
                    let index = self.next_tank_index.pop().unwrap();
                    let tank = self.board.tanks.get_mut(index).unwrap();
                    println!("{:?}",tank);
                    // TODO Next move input.
                    let next_move = tank.strategy.next_move(Default::default());
                    if *next_move == Move::TurnLeft {
                        tank.direction = tank.direction.counter_clockwise();
                    } else if *next_move == Move::TurnRight {
                        tank.direction = tank.direction.clockwise();
                    } else if *next_move == Move::Forward {
                        let direction = tank.direction.clone();
                        tank.point = tank.point.with_offset(direction, 1);
                    }
                    println!("{:?}", next_move);
                    self.board_cache.clear();  // Trigger draw on canvas.
                    if Move::Fire == *next_move {
                        Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Laser)
                    } else {
                        Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Move)
                    }
                }
            }
            Message::Laser(_) => {
                println!("Laser");
                // TODO: if Hit go to Hit.
                Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Move)
            }
            Message::Hit(_) => {
                println!("Hit");
                Command::none()
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
            let center_transform = Transform2D::translation(-10.0, -10.0);
            let restore_transform = Transform2D::translation(10.0, 10.0);

            for tank in &self.board.tanks {
                frame.with_save(|frame| {
                    // Rotate tank.
                    let center_path = tank_path.transform(&center_transform);
                    let transform_rotation = Transform2D::rotation(Angle::degrees(tank.direction.degrees()));
                    let rotated_path = center_path.transform(&transform_rotation);
                    let restore_path = rotated_path.transform(&restore_transform);
                    let x = (tank.point.x * 20) as f32;
                    let y = (tank.point.y * 20) as f32;
                    frame.translate(Vector::new(x, y));
                    frame.fill( &restore_path, tank.color);
                });
            }

            // TODO: Laser. A line from the shooting tank.

            // TODO: Tank hit. An X over a tank if hit.


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
    builder.close();
    builder.build()
}

// TODO: #[derive(Debug)] Implement Debug in Strategy
struct Board {
    dimension: Dimension,
    tanks : Vec<Tank>,
    laser: Laser,
    hit: Hit,
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
            laser: Default::default(),
            hit: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
struct Hit {
    point: BoardPoint,
    is_visible: bool,
}


#[derive(Default, Debug)]
struct Laser {
    point: BoardPoint,
    direction: Direction,
    is_visible: bool,
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
            direction: Default::default(),
        }
    }
}

impl Tank {
    const MAX_ENERGY: usize = 5;
    const FIRE_RANGE: usize = 10;
}

#[derive(PartialEq, Debug, Clone)]
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
    x: isize,
    y: isize,
}

impl Default for BoardPoint {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

impl BoardPoint {
    fn with_offset(&self, direction: Direction, offset: isize) -> Self {
        Self {
            x: self.x + direction.x() * offset,
            y: self.y + direction.y() * offset}
        }
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn degrees(&self) -> f32 {
        match *self {
            Self::North => 0.0,
            Self::East => 90.0,
            Self::South => 180.0,
            Self::West => 270.0,
        }
    }

    fn x(&self) -> isize {
        match *self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 0,
            Self::West => -1,
        }
    }

    fn y(&self) -> isize {
        match *self {
            Self::North => -1,
            Self::East => 0,
            Self::South => 1,
            Self::West => 0,
        }
    }

    fn clockwise(&self) -> Direction {
        match *self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn counter_clockwise(&self) -> Direction {
        match *self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn opposite(&self) -> Direction {
        match *self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

#[derive(Default, Debug)]
struct TankStatus {
    direction: Direction,
    location: BoardPoint,
    is_alive: bool,
}

#[derive(Debug, Default)]
struct NextMoveInput {
    game_board: Dimension,
    own_status: TankStatus,
    opponent_status: Vec<TankStatus>,
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

