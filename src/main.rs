

use iced::{alignment, Application, Color, Command, Element, executor, Length, mouse, Point, Rectangle, Renderer, Settings, Theme, Vector};
use iced::widget::canvas::{Geometry, Cache, Path, path};
use iced::widget::{canvas, Canvas, column, container, scrollable, text};
use std::default::Default;

fn main() -> iced::Result {
    RobotChallenge::run(Settings::default())
}

struct RobotChallenge {
    round: u16,
    board: Board,
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
                board: Default::default(),
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

            // Draw game board background.
            let background = Path::rectangle(Point::new(0.0, 0.0), frame.size());
            frame.fill(&background, GameColors::LIGHT_GRAY);

            // Draw tanks.
            let tank_path = tank_path();

            for tank in &self.board.tanks {
                frame.with_save(|frame| {
                    let x = (tank.point.x as u16 * 20) as f32;
                    let y = (tank.point.y as u16 * 20) as f32;
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

// TODO: Implement Debug
struct Tank {
    strategy: Box<dyn Strategy>,
    color: Color,
    energy: u8,
    hits: u8,
    frags: u8,
    point: BoardPoint, // Set to random available Point when adding to Board.
    direction: Direction, // Set to random direction when adding to Board.
}

impl Default for Tank {
    fn default() -> Self {
        Self {
            strategy: Box::new(Dummy::new()),
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
    const MAX_ENERGY: u8 = 5;
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
    width: u8,
    height: u8,
}

impl Default for Dimension {
    fn default() -> Self {
        Self { width: 20, height: 20 }
    }
}

#[derive(Debug, Clone)]
struct BoardPoint {
    x: u8,
    y: u8,
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

#[derive(Debug, Clone)]
struct Status {
    direction: Direction,
    location: BoardPoint,
    is_alive: bool,
}

#[derive(Debug, Clone)]
struct StrategyInput {
    game_board: Dimension,
    own_status: Status,
    opponent_status: Vec<Status>,
    fire_range: usize,
}

trait Strategy {
    fn name(&self) -> String;
    fn author(&self) -> String;
    fn next_move(&mut self, input: StrategyInput) -> &Move;
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

    fn next_move(&mut self, input: StrategyInput) -> &Move {
        let next_move = self.moves.get(self.move_index).unwrap();
        self.move_index = (self.move_index + 1) % self.moves.len();
        next_move
    }
}

