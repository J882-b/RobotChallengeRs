
use std::collections::VecDeque;
use crate::game::{
    BoardPoint,
    Direction,
    Move,
    NextMoveInput,
    Strategy
};

#[derive(Debug, Clone)]
pub(crate) struct Dummy {
    name: String,
    author: String,
    moves: Vec<Move>,
    move_index: usize
}

impl Default for Dummy {
    fn default() -> Self {
        Self {
            name: "Dummy".to_string(),
            author: "JMH".to_string(),
            moves: vec![Move::Fire, Move::TurnLeft, Move::Forward],
            move_index: 0,
        }
    }
}

impl Dummy {
    pub(crate) fn dummy2() -> Self {
        Self {
            name: "Dummy2".to_string(),
            author: "JMH".to_string(),
            moves: vec![Move::Fire, Move::TurnRight, Move::Forward],
            move_index: 0,
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

    fn next_move(&mut self, _input: NextMoveInput) -> Move {
        let next_move = self.moves.get(self.move_index).unwrap();
        self.move_index = (self.move_index + 1) % self.moves.len();
        next_move.clone()
    }
}

pub(crate) struct Random {
    name: String,
    author: String,
}

impl Default for Random {
    fn default() -> Self {
        Self {
            name: "Random".to_string(),
            author: "Martin".to_string(),
        }
    }
}

impl Random {
    pub(crate) fn random2() -> Self {
        Self {
            name: "Random2".to_string(),
            author: "Martin".to_string(),
        }
    }
}

impl Strategy for Random {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn author(&self) -> String {
        self.author.clone()
    }

    fn next_move(&mut self, _input: NextMoveInput) -> Move {
        rand::random()
    }
}

pub(crate) struct Slacker {
    name: String,
    author: String,
}

impl Default for Slacker {
    fn default() -> Self {
        Self {
            name: "Eric Idle".to_string(),
            author: "Martin".to_string(),
        }
    }
}

impl Strategy for Slacker {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn author(&self) -> String {
        self.author.clone()
    }

    fn next_move(&mut self, _input: NextMoveInput) -> Move {
        Move::Wait
    }
}

pub(crate) struct Spinner {
    name: String,
    author: String,
    shoot: bool,

}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            name: "Spinner".to_string(),
            author: "Martin".to_string(),
            shoot: true,
        }
    }
}

impl Strategy for Spinner {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn author(&self) -> String {
        self.author.clone()
    }

    fn next_move(&mut self, _input: NextMoveInput) -> Move {
        self.shoot = !self.shoot;
        if self.shoot { Move::Fire } else { Move::TurnRight }
    }
}

pub(crate) struct FireFire {
    name: String,
    author: String,
}

impl Default for FireFire {
    fn default() -> Self {
        Self {
            name: "FireFire".to_string(),
            author: "Johan".to_string(),
        }
    }
}

impl Strategy for FireFire {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn author(&self) -> String {
        self.author.clone()
    }

    fn next_move(&mut self, input: NextMoveInput) -> Move {
        let my_position = Position {
            point: input.own_status.location,
            direction: input.own_status.direction,
            moves: VecDeque::new(),
        };

        let mut alive_positions = Vec::new();
        let mut dead_positions = Vec::new();
        for other in input.opponent_status {
            let mut positions = Position::all(other.location);
            if other.is_alive {
                alive_positions.append(&mut positions);
            } else {
                dead_positions.append(&mut positions);
            }
        }

        let mut visited = Vec::new();
        visited.append(&mut alive_positions.clone());
        visited.append(&mut dead_positions.clone());

        self.find_move_to_closest_fire(my_position, alive_positions, visited,
                                       input.fire_range, dead_positions,
                                       input.game_board.width, input.game_board.height)
    }
}

impl FireFire {
    fn find_move_to_closest_fire(&self, root: Position, search: Vec<Position>, mut visited: Vec<Position>,
                                 fire_range: usize, dead_positions: Vec<Position>, width: usize, height: usize) -> Move {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while queue.len() > 0 {
            let mut current_position = queue.pop_front().unwrap();
            visited.push(current_position.clone());
            if current_position.is_fire_position(&search, fire_range, &dead_positions) {
                println!("visited.len(): {}", visited.len());
                current_position.moves.push_back(Move::Fire);
                return current_position.moves.pop_front().unwrap().clone();
            } else {
                let new_positions = vec![current_position.drive(),
                                         current_position.clockwise(), current_position.counter_clockwise()];

                for position in new_positions {
                    if position.is_valid(width, height) && !visited.contains(&position) {
                        queue.push_back(position);
                    }
                }
            }
        }
        Move::Forward
    }
}

#[derive(Debug, Clone)]
struct Position {
    point: BoardPoint,
    direction: Direction,
    moves: VecDeque<Move>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            point: BoardPoint::default(),
            direction: Direction::default(),
            moves: VecDeque::new(),
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.direction == other.direction
    }
}

impl Position {
    fn new(point: BoardPoint, direction: Direction) -> Self {
        Self {
            point,
            direction,
            moves: VecDeque::new(),
        }
    }
    fn all(point: BoardPoint) -> Vec<Self> {
        vec![
            Position::new(point.clone(), Direction::North),
            Position::new(point.clone(), Direction::East),
            Position::new(point.clone(), Direction::South),
            Position::new(point.clone(), Direction::West),
        ]
    }

    fn fire(&self, fire_range: usize, dead_positions: &Vec<Position>) -> Vec<Position> {
        let mut positions = Vec::new();
        for i in 1..fire_range {
            let test = Position::new(self.point.with_offset(self.direction, i as isize), self.direction);

            let test_in_dead_position = dead_positions.iter().any(|position| *position == test);

            if test_in_dead_position {
                break;
            } else {
                positions.push(test);
            }
        }
        positions
    }
    fn is_fire_position(&self, search: &Vec<Position>, fire_range: usize, dead_positions: &Vec<Position>) -> bool {
        let possible = self.fire(fire_range, dead_positions);
        for possible_position in possible {
            for searched_position in search {
                if possible_position == *searched_position {
                    return true;
                }
            }
        }
        return false;
    }

    fn drive(&self) -> Position {
        let mut moves = self.moves.clone();
        moves.push_back(Move::Forward);
        Self {
            point: self.point.with_offset(self.direction, 1),
            direction: self.direction,
            moves,
        }
    }

    fn clockwise(&self) -> Position {
        let mut moves = self.moves.clone();
        moves.push_back(Move::TurnRight);
        Self {
            point: self.point.clone(),
            direction: self.direction.clockwise(),
            moves,
        }
    }

    fn counter_clockwise(&self) -> Position {
        let mut moves = self.moves.clone();
        moves.push_back(Move::TurnLeft);
        Self {
            point: self.point.clone(),
            direction: self.direction.counter_clockwise(),
            moves,
        }
    }

    fn is_valid(&self, width: usize, height: usize) -> bool {
        0 <= self.point.x && self.point.x < width as isize
            && 0 <= self.point.y && self.point.y < height as isize
    }
}
