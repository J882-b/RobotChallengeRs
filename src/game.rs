

use iced::{
    alignment,
    Application,
    Color,
    Command,
    Element,
    executor,
    Length,
    mouse,
    Point,
    Rectangle,
    Renderer,
    Theme,
    Vector
};
use iced::widget::canvas::{
    Cache,
    Geometry,
    LineCap,
    Path,
    path,
    Stroke,
    stroke
};
use iced::widget::{
    canvas,
    Canvas,
    Column,
    container,
    row,
    Row,
    scrollable,
    text
};
use std::default::Default;
use std::fmt::{
    Debug,
    Formatter
};
use std::time::Duration;
use iced::widget::canvas::path::lyon_path::geom::Angle;
use iced::widget::canvas::path::lyon_path::geom::euclid::Transform2D;
use rand::distributions::{
    Distribution,
    Standard
};
use rand::Rng;
use rand::seq::SliceRandom;
use crate::strategies::{Dummy, FireFire, Random, Slacker, Spinner};


pub(crate) struct RobotChallenge {
    round: usize,
    next_tank_index: Vec<usize>,
    board_cache: Cache,
    dimension: Dimension,
    tanks: Vec<Tank>,
    laser: Laser,
    hit: Hit,
}

impl RobotChallenge {
    fn is_valid_point(&self, point: &BoardPoint) -> bool {
        0 <= point.x && point.x < self.dimension.width as isize
            && 0 <= point.y && point.y < self.dimension.height as isize
    }

    fn is_tank(&self, point: &BoardPoint) -> bool {
        self.tanks.iter().any(|tank| tank.point == *point)
    }

    fn get_tank_mut(&mut self, point: &BoardPoint) -> &mut Tank {
        self.tanks.iter_mut().find(|tank| tank.point == *point).unwrap()
    }

    fn next_move_input(&self, current_index: usize) -> NextMoveInput {
        let mut next_move_input = NextMoveInput::default();
        next_move_input.game_board = self.dimension.clone();
        next_move_input.fire_range = Tank::FIRE_RANGE;
        for index in 0..self.tanks.len() {
            let tank = self.tanks.get(index).unwrap();
            let tank_status = TankStatus {
                direction: tank.direction.clone(),
                location: tank.point.clone(),
                is_alive: tank.energy > 0,
            };
            if index == current_index {
                next_move_input.own_status = tank_status
            } else {
                next_move_input.opponent_status.push(tank_status);
            }
        }
        next_move_input
    }

    fn score_row<'a, Message, Renderer>(name: String, name_color: Color, energy: String, hits: String, frags: String)
                                        -> Row<'a, Message, Renderer>
        where
            Renderer: iced_core::text::Renderer + 'a,
            Renderer::Theme: text::StyleSheet,
            <<Renderer as iced_core::Renderer>::Theme as text::StyleSheet>::Style: From<Color>
    {
        let score_name = text(name)
            .width(200)
            .size(15)
            .style(name_color)
            .horizontal_alignment(alignment::Horizontal::Center);

        let score_energy = text(energy)
            .width(50)
            .size(15)
            .style(Color::BLACK)
            .horizontal_alignment(alignment::Horizontal::Center);

        let score_hits = text(hits)
            .width(50)
            .size(15)
            .style(Color::BLACK)
            .horizontal_alignment(alignment::Horizontal::Center);

        let score_frags = text(frags)
            .width(50)
            .size(15)
            .style(Color::BLACK)
            .horizontal_alignment(alignment::Horizontal::Center);

        row![score_name, score_energy, score_hits, score_frags]
    }
}

impl RobotChallenge {
    const MAX_ROUNDS: usize = 100;
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    NewGame(Result<String, SleeperError>),
    NewRound(Result<String, SleeperError>),
    Move(Result<String, SleeperError>),
    Laser(Result<String, SleeperError>),
    Hit(Result<String, SleeperError>),
    EndGame(Result<String, SleeperError>)
}

impl Application for RobotChallenge {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            // TODO: Generate tanks from strategies, colors and board points.

            // TODO: Random for available board point.

            Self {
                round: 0,
                next_tank_index: vec![],
                board_cache: Default::default(),
                dimension: Default::default(),
                tanks: vec![
                    Tank {
                        color: GameColors::RED,
                        point: BoardPoint { x: 6, y: 10 },
                        strategy: Box::new(Dummy::default()),
                        ..Default::default()
                    },
                    Tank {
                        color: GameColors::BLUE,
                        point: BoardPoint { x: 2, y: 10 },
                        strategy: Box::new(Dummy::dummy2()),
                        ..Default::default()
                    },
                    Tank {
                        color: GameColors::GREEN,
                        point: BoardPoint { x: 12, y: 12 },
                        strategy: Box::new(Random::default()),
                        ..Default::default()
                    },
                    Tank {
                        color: GameColors::AQUA,
                        point: BoardPoint { x: 4, y: 14 },
                        strategy: Box::new(Random::random2()),
                        ..Default::default()
                    },
                    Tank {
                        color: GameColors::PERU,
                        point: BoardPoint { x: 14, y: 5 },
                        strategy: Box::new(Slacker::default()),
                        ..Default::default()
                    },
                    Tank {
                        color: GameColors::TOMATO,
                        point: BoardPoint { x: 10, y: 5 },
                        strategy: Box::new(Spinner::default()),
                        ..Default::default()
                    },
                    Tank {
                        color: GameColors::PINK,
                        point: BoardPoint { x: 19, y: 19 },
                        strategy: Box::new(FireFire::default()),
                        ..Default::default()
                    }],
                laser: Default::default(),
                hit: Default::default(),
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
                Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::NewRound)
            }
            Message::NewRound(_) => {
                self.round += 1;
                println!("NewRound {}", self.round);
                if self.round >= RobotChallenge::MAX_ROUNDS {
                    Command::none()
                } else {
                    // Randomize next tank index
                    let mut indexes: Vec<usize> = (0..self.tanks.len()).collect();
                    let mut rng = rand::thread_rng();
                    indexes.shuffle(&mut rng);

                    for index in indexes {
                        let tank = self.tanks.get(index).unwrap();
                        // Only tanks with energy can move.
                        if tank.energy > 0 {
                            self.next_tank_index.push(index as usize);
                        }
                    }
                    if self.next_tank_index.len() > 1 {
                        Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Move)
                    } else {
                        Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::EndGame)
                    }
                }
            }
            Message::Move(_) => {
                if self.next_tank_index.is_empty() {
                    Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::NewRound)
                } else {
                    let index = self.next_tank_index.pop().unwrap();

                    let mut next_move = Move::Wait;
                    let tank = self.tanks.get(index).unwrap();
                    // If energy == 0 just wait.
                    if tank.energy > 0 {
                        let next_move_input = self.next_move_input(index);
                        println!("{:?}", tank);
                        let tank = self.tanks.get_mut(index).unwrap();
                        next_move = tank.strategy.next_move(next_move_input);
                        println!("{:?}", next_move);
                    }
                    if next_move == Move::TurnLeft {
                        let tank = self.tanks.get_mut(index).unwrap();
                        tank.direction = tank.direction.counter_clockwise();
                    } else if next_move == Move::TurnRight {
                        let tank = self.tanks.get_mut(index).unwrap();
                        tank.direction = tank.direction.clockwise();
                    } else if next_move == Move::Forward {
                        let tank = self.tanks.get(index).unwrap();
                        let new_point = tank.point.with_offset(tank.direction, 1);
                        let is_valid_point = self.is_valid_point(&new_point);
                        let is_tank = self.is_tank(&new_point);
                        if is_valid_point && !is_tank {
                            let tank = self.tanks.get_mut(index).unwrap();
                            tank.point = new_point;
                        }
                    } else if next_move == Move::Fire {
                        let tank = self.tanks.get(index).unwrap();
                        self.laser.point = tank.point.clone();
                        self.laser.direction = tank.direction.clone();
                        self.laser.length = Tank::FIRE_RANGE;
                        self.laser.is_visible = true;
                        // Change laser length if there is a tank or board edge.
                        let fire_direction = self.laser.direction.clone();
                        for i in 1..=Tank::FIRE_RANGE {
                            let fire_point = self.laser.point.with_offset(fire_direction, i as isize);
                            if !self.is_valid_point(&fire_point) {
                                self.laser.length = i - 1;
                                break;
                            } else if self.is_tank(&fire_point) {
                                self.laser.hit = true;
                                self.laser.length = i - 1;
                                self.hit.point = fire_point.clone();
                                // Update tank energy, hits, frags.
                                let hit_tank = self.get_tank_mut(&fire_point);
                                let mut frag = false;
                                if hit_tank.energy > 0 {
                                    hit_tank.energy -= 1;
                                    if hit_tank.energy == 0 {
                                        frag = true;
                                    }
                                    let tank = self.tanks.get_mut(index).unwrap();
                                    tank.hits += 1;
                                    tank.frags += if frag { 1 } else { 0 };
                                }
                                break;
                            }
                        }
                    }
                    self.board_cache.clear();  // Trigger draw on canvas.
                    if Move::Fire == next_move {
                        Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Laser)
                    } else {
                        Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Move)
                    }
                }
            }
            Message::Laser(_) => {
                println!("Laser");
                let hit = self.laser.hit;
                // Reset laser
                self.laser = Default::default();
                self.board_cache.clear();  // Trigger draw on canvas.
                // Perform hit if needed.
                if hit {
                    self.hit.is_visible = true;
                    self.board_cache.clear();  // Trigger draw on canvas.
                    Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Hit)
                } else {
                    Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Move)
                }
            }
            Message::Hit(_) => {
                println!("Hit");
                // Reset hit
                self.hit = Default::default();
                self.board_cache.clear();  // Trigger draw on canvas.
                Command::perform(Sleeper::sleep(Duration::from_millis(100)), Message::Move)
            }
            Message::EndGame(_) => {
                println!("EndGame");
                let index = self.next_tank_index.pop().unwrap();
                let tank = self.tanks.get_mut(index).unwrap();
                println!("The winner is {} by {}", tank.strategy.name(), tank.strategy.author());
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let game_board: Canvas<&RobotChallenge, Message> = canvas(self as &Self)
            .width(400)
            .height(400);

        let score_row_headers = RobotChallenge::score_row(
            "Name".to_string(),
            Color::BLACK,
            "Energy".to_string(),
            "Hits".to_string(),
            "Frags".to_string());

        let score_rows = self.tanks.iter().map(|tank|
            RobotChallenge::score_row::<Message, Renderer>(
                tank.strategy.name().clone(),
                tank.color.clone(),
                tank.energy.to_string().clone(),
                tank.hits.to_string().clone(),
                tank.frags.to_string().clone()
            )
        ).collect::<Vec<_>>();

        let mut elements: Vec<Element<Message>> = vec!();
        elements.push(Element::from(game_board));
        elements.push(Element::from(score_row_headers));
        for row in score_rows {
            elements.push(Element::from(row));
        }

        let content = Column::with_children(elements)
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
    async fn sleep(duration: Duration) -> Result<String, SleeperError> {
        std::thread::sleep(duration);
        Ok("Booing".to_string())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum SleeperError {
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

            for tank in &self.tanks {
                frame.with_save(|frame| {
                    // Rotate tank.
                    let center_path = tank_path.transform(&center_transform);
                    let transform_rotation = Transform2D::rotation(Angle::degrees(tank.direction.degrees()));
                    let rotated_path = center_path.transform(&transform_rotation);
                    let restore_path = rotated_path.transform(&restore_transform);
                    let x = (tank.point.x * 20) as f32;
                    let y = (tank.point.y * 20) as f32;
                    frame.translate(Vector::new(x, y));
                    frame.fill(&restore_path, tank.color);

                    // Draw dead tank. A X over a tank if dead.
                    if tank.energy == 0 {
                        let dead_path1 = Path::line(Point::new(4.0, 4.0), Point::new(16.0, 16.0));
                        let dead_path2 = Path::line(Point::new(16.0, 4.0), Point::new(4.0, 16.0));

                        let dead_stroke = || -> Stroke {
                            Stroke {
                                width: 2.0,
                                style: stroke::Style::Solid(Color::BLACK),
                                line_cap: LineCap::Round,
                                ..Stroke::default()
                            }
                        };

                        frame.stroke(&dead_path1, dead_stroke());
                        frame.stroke(&dead_path2, dead_stroke());
                    }
                });
            }

            // Draw laser. A line from the shooting tank.
            if self.laser.is_visible {
                let laser_start = self.laser.point.with_offset(self.laser.direction, 1);
                let laser_end = self.laser.point.with_offset(self.laser.direction, self.laser.length as isize);
                let mut start_point = Point::new(laser_start.x as f32 * 20.0 + 10.0, laser_start.y as f32 * 20.0 + 10.0);
                let mut end_point = Point::new(laser_end.x as f32 * 20.0 + 10.0, laser_end.y as f32 * 20.0 + 10.0);

                // Adjust laser end points from the default center of board cell.
                match self.laser.direction {
                    Direction::North => {
                        start_point.y += 10.0;
                        end_point.y -= 10.0;
                    }
                    Direction::South => {
                        start_point.y -= 10.0;
                        end_point.y += 10.0;
                    }
                    Direction::East => {
                        start_point.x -= 10.0;
                        end_point.x += 10.0;
                    }
                    Direction::West => {
                        start_point.x += 10.0;
                        end_point.x -= 10.0;
                    }
                }

                let laser_path = Path::line(start_point, end_point);

                let laser_stroke = || -> Stroke {
                    Stroke {
                        width: 2.0,
                        style: stroke::Style::Solid(Color::BLACK),
                        line_cap: LineCap::Round,
                        ..Stroke::default()
                    }
                };

                frame.stroke(&laser_path, laser_stroke());
            }

            // Draw hit. A X over a tank if hit.
            if self.hit.is_visible {
                let hit_path1 = Path::line(Point::new(4.0, 4.0), Point::new(16.0, 16.0));
                let hit_path2 = Path::line(Point::new(16.0, 4.0), Point::new(4.0, 16.0));

                let hit_stroke = || -> Stroke {
                    Stroke {
                        width: 2.0,
                        style: stroke::Style::Solid(Color::BLACK),
                        line_cap: LineCap::Round,
                        ..Stroke::default()
                    }
                };

                let x = (self.hit.point.x * 20) as f32;
                let y = (self.hit.point.y * 20) as f32;
                frame.translate(Vector::new(x, y));
                frame.stroke(&hit_path1, hit_stroke());
                frame.stroke(&hit_path2, hit_stroke());
            }
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

    //#FF6347
    const TOMATO: Color = Color {
        r: 1.0,
        g: 0.387,
        b: 0.277,
        a: 1.0,
    };

    // #CD853F
    const PERU: Color = Color {
        r: 0.804,
        g: 0.519,
        b: 0.247,
        a: 1.0,
    };

    // #00FFFF
    const AQUA: Color = Color {
        r: 0.0,
        g: 1.,
        b: 1.0,
        a: 1.0,
    };

    // #FFD700
    const PINK: Color = Color {
        r: 1.0,
        g: 0.84,
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

#[derive(Default, Debug)]
struct Hit {
    point: BoardPoint,
    is_visible: bool,
}

#[derive(Default, Debug)]
struct Laser {
    point: BoardPoint,
    direction: Direction,
    length: usize,
    hit: bool,
    is_visible: bool,
}

struct Tank {
    strategy: Box<dyn Strategy>,
    color: Color,
    energy: usize,
    hits: usize,
    frags: usize,
    point: BoardPoint,
    // Set to random available Point when adding to Board.
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
            strategy: Box::new(Random::default()),
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
    const FIRE_RANGE: usize = 5;
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Move {
    Fire,
    TurnLeft,
    Forward,
    TurnRight,
    Wait,
}

impl Distribution<Move> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        match rng.gen_range(0..=2) {
            0 => Move::Fire,
            1 => Move::TurnLeft,
            2 => Move::Forward,
            3 => Move::TurnRight,
            _ => Move::Wait,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Dimension {
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Default for Dimension {
    fn default() -> Self {
        Self { width: 20, height: 20 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BoardPoint {
    pub(crate) x: isize,
    pub(crate) y: isize,
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
    pub(crate) fn with_offset(&self, direction: Direction, offset: isize) -> Self {
        Self {
            x: self.x + direction.x() * offset,
            y: self.y + direction.y() * offset
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Direction {
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

    pub(crate) fn clockwise(&self) -> Direction {
        match *self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub(crate) fn counter_clockwise(&self) -> Direction {
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
pub(crate) struct TankStatus {
    pub(crate) direction: Direction,
    pub(crate) location: BoardPoint,
    pub(crate) is_alive: bool,
}

#[derive(Debug, Default)]
pub(crate) struct NextMoveInput {
    pub(crate) game_board: Dimension,
    pub(crate) own_status: TankStatus,
    pub(crate) opponent_status: Vec<TankStatus>,
    pub(crate) fire_range: usize,
}

pub(crate) trait Strategy {
    fn name(&self) -> String;
    fn author(&self) -> String;
    fn next_move(&mut self, input: NextMoveInput) -> Move;
}