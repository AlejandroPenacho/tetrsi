use crossterm::{execute, cursor};

pub enum Tetromino {
    Straight,
    Square,
    Z(bool),
    L(bool),
    T,
}

#[derive(PartialEq, Eq)]
pub enum KeyOrder {
    Up,
    Down,
    Left,
    Right,
    Exit,
    Rotate
}

enum Orientation {
    Left,
    Right,
}

pub struct Game {
    board: TetrisBoard,
    state: Vec<bool>,
    next_piece: Tetromino,
    current_piece: Option<FallingTetromino>,
}

pub struct FallingTetromino {
    piece: Tetromino,
    position: (i16, i16),
    angle: u8
}

struct TetrisBoard {
    x_0: (i16, i16),
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: TetrisBoard { x_0: (2,2)},
            state: vec![false;200],
            next_piece: Tetromino::Straight,
            current_piece: Some(FallingTetromino {
                piece: Tetromino::Z(true),
                position: (3,7),
                angle: 0
            })
        }
    }

    pub fn draw_board(&self) {
        self.board.draw();
    }

    pub fn draw_piece(&self) {
        if let Some(piece) = &self.current_piece {
            piece.draw("#", self.board.x_0);
        }
    }

    pub fn erase_piece(&self) {
        if let Some(piece) = &self.current_piece {
            piece.draw(" ", self.board.x_0);
        }
    }

    fn position_is_valid(&self) -> bool {
        let piece_grid = self.current_piece.as_ref().unwrap().get_rotated_grid();
        let center_position = self.current_piece.as_ref().unwrap().position;

        for grid in piece_grid {
            let point = (
                center_position.0 + grid.0,
                center_position.1 + grid.1
            );

            if  point.0 < 0 ||
                point.0 > 9 ||
                point.1 < 0 ||
                point.1 > 19 {


                return false
            }
            
            if self.state[point.0 as usize + 10 * point.1 as usize] {
                return false
            }

        }
        return true
    }

    pub fn move_piece(&mut self, delta_x: (i16, i16), rotation: u8) {

        let original_position = self.current_piece.as_ref().unwrap().position;
        let original_angle = self.current_piece.as_ref().unwrap().angle;

        self.erase_piece();

        self.current_piece.as_mut().unwrap().position = (
            original_position.0 + delta_x.0,
            original_position.1 + delta_x.1
        );

        self.current_piece.as_mut().unwrap().angle = (self.current_piece.as_mut().unwrap().angle + rotation) % 4;

        if !self.position_is_valid() {
            self.current_piece.as_mut().unwrap().position = original_position;
            self.current_piece.as_mut().unwrap().angle = original_angle;
        }

        self.draw_piece();
    }
}

impl TetrisBoard {
    fn draw(&self) {
        let mut stdout = std::io::stdout();
        let border_limit = (self.x_0.0-1, self.x_0.1);
        execute!(stdout, cursor::MoveTo(border_limit.0 as u16, border_limit.1 as u16)).unwrap();

        for i in 0..20 {
            print!("#");
            execute!(stdout, cursor::MoveRight(10)).unwrap();
            print!("#");
            execute!(stdout, cursor::MoveDown(1), cursor::MoveLeft(12)).unwrap();
        }
        for i in 0..12 {
            print!("#");
        }
        print!("\n");

    }
}

impl FallingTetromino {

    pub fn new(piece: Tetromino, position: (i16, i16), angle: u8) -> FallingTetromino {
        FallingTetromino {
            piece,
            position,
            angle
        }
    }

    fn get_rotated_grid(&self) -> Vec<(i16,i16)> {
        let mut piece = self.piece.get_grid();

        let rotation_matrix = match self.angle {
            0 => [1, 0, 0, 1],
            1 => [0, 1, -1, 0],
            2 => [-1, 0, 0, -1],
            3 => [0, -1, 1, 0],
            n => panic!("Wrong rotation")
        };

        for point in piece.iter_mut() {
            *point = (
                point.0 * rotation_matrix[0] + point.1 * rotation_matrix[1],
                point.0 * rotation_matrix[2] + point.1 * rotation_matrix[3]
            );
        }

        return piece

    }

    pub fn draw(&self, fill: &str, board_offset: (i16, i16)) {
        let mut stdout = std::io::stdout();
        let points = self.get_rotated_grid();
        for point in points {
            let position = (
                self.position.0 + point.0 + board_offset.0,
                self.position.1 + point.1 + board_offset.1,
            );
            execute!(stdout, cursor::MoveTo(position.0 as u16, position.1 as u16)).unwrap();
            print!("{}\n", fill);
        }
    }


}



impl Tetromino {
    fn get_grid(&self) -> Vec<(i16,i16)> {
        match self {
            Tetromino::Straight => {
                return vec![(0,-2), (0,-1), (0,0), (0,1)]
            },
            Tetromino::Square => {
                return vec![(0,0), (0,1), (1,0),(1,1)]
            },
            Tetromino::T => {
                return vec![(0,0), (1,0), (0, -1), (-1,0)]
            },
            Tetromino::L(real_l) => {
               if *real_l {
                   return vec![(0,-1), (0,0), (0,1), (1,1)]
               } else {
                   return vec![(0,-1), (0,0), (0,1), (-1,1)]
               }
            },
            Tetromino::Z(real_z) => {
                if *real_z {
                    return vec![(0,0), (0,-1), (-1,-1), (1,0)]
                } else {
                    return vec![(0,0), (0,-1), (1,-1), (-1,0)]
                }
            }
        } 
     }
 }