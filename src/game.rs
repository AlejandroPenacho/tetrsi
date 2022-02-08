use crossterm::{execute, cursor};
use rand;
use rand::seq::SliceRandom;

pub enum Tetromino {
    Straight,
    Square,
    Z(bool),
    L(bool),
    T,
}

#[derive(PartialEq, Eq)]
pub enum KeyOrder {
    Down,
    SoftDrop,
    HardDrop,
    Left,
    Right,
    Exit,
    Rotate
}

pub struct Game {
    board: TetrisBoard,
    state: Vec<bool>,
    next_piece: Tetromino,
    current_piece: Option<FallingTetromino>,
    score: u64
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
            next_piece: get_random_tetromino(),
            current_piece: Some(FallingTetromino {
                piece: get_random_tetromino(),
                position: (3,7),
                angle: 0
            }),
            score: 0
        }
    }

    pub fn draw_board(&self) {
        self.board.draw();
        self.draw_next_piece();
    }

    pub fn draw_piece(&self) {
        if let Some(piece) = &self.current_piece {
            piece.draw("@", self.board.x_0);
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

    pub fn move_piece(&mut self, delta_x: (i16, i16), rotation: u8, falling: bool) -> bool {

        let original_position = self.current_piece.as_ref().unwrap().position;
        let original_angle = self.current_piece.as_ref().unwrap().angle;

        self.erase_piece();

        self.current_piece.as_mut().unwrap().position = (
            original_position.0 + delta_x.0,
            original_position.1 + delta_x.1
        );

        self.current_piece.as_mut().unwrap().angle = (self.current_piece.as_mut().unwrap().angle + rotation) % 4;

        let mut has_fixed: bool = false;

        if !self.position_is_valid() {
            self.current_piece.as_mut().unwrap().position = original_position;
            self.current_piece.as_mut().unwrap().angle = original_angle;

            if falling {
                self.draw_piece();
                self.fix_piece();
                has_fixed = true;
            }
        }

        self.draw_piece();
        has_fixed
    }

    fn fix_piece(&mut self) {
        let current_piece = self.current_piece.as_mut().unwrap();
        let center = current_piece.position;
        for grid in current_piece.get_rotated_grid() {
            let real_point = (
                center.0 + grid.0,
                center.1 + grid.1
            );
            self.state[real_point.0 as usize + real_point.1 as usize *10] = true;
        }

        self.current_piece = Some(FallingTetromino::new(
            std::mem::replace(&mut self.next_piece, get_random_tetromino()),
            (5,5),
            0
        ));
        self.draw_next_piece();

        self.clean_lines();
    }

    fn clean_lines(&mut self) {

        let mut row = 20;

        let mut n_clears = 0;

        while row >0 {
            row -= 1;

            let hole = (0..10).any(|i| !self.state[row*10 + i]);

            if !hole {
                n_clears += 1;
                (0..10).for_each(|i| self.state[row*10 + i] = false);

                (0..10).for_each(|i| self.state[i] = false);
                for sub_row in (1..(row+1)).rev() {
                    (0..10).for_each(|i| self.state[sub_row*10 + i] = self.state[(sub_row-1)*10 + i]);
                }
                row += 1;
            }

        }

        if n_clears > 0 {
            self.redraw_interior();
            match n_clears {
                1 => {self.update_score(40)},
                2 => {self.update_score(100)},
                3 => {self.update_score(300)},
                4 => {self.update_score(1200)},
                _ => {panic!()}
            }
        }


    }

    fn redraw_interior(&mut self) {
        let mut stdout = std::io::stdout();
        execute!(stdout, cursor::MoveTo(self.board.x_0.0 as u16, self.board.x_0.1 as u16));
        for row in 0..20 {
            for i in 0..10 {
                if self.state[i + row*10] {
                    print!("@");
                } else {
                    print!(" ");
                }
            }
            print!("\n");
            execute!(stdout, cursor::MoveLeft(10));
        }
    }

    fn draw_next_piece(&self) {
        let mut stdout = std::io::stdout();
        for i in 6..12 {
            execute!(stdout, cursor::MoveTo(15, i));
            print!("         \n");
        }
        let center = (20, 9);
        let points: Vec<(i16, i16)> = self.next_piece.get_grid().iter().map(|x| (x.0 + center.0, x.1 + center.1)).collect();

        for point in points {
            execute!(stdout, cursor::MoveTo(point.0 as u16, point.1 as u16)).unwrap();
            print!("@\n");
        }
    }

    pub fn update_score(&mut self, points: u64) {
        self.score += points;
        execute!(std::io::stdout(), cursor::MoveTo(15,3));
        print!("{}", self.score);
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

        execute!(stdout, cursor::MoveTo(15, 2));
        print!("Score:\n");
        execute!(stdout, cursor::MoveTo(15, 3));
        print!("{}", 0);

        execute!(stdout, cursor::MoveTo(15, 5));
        print!("Next piece:\n")
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


 fn get_random_tetromino() -> Tetromino {

    let mut rng = rand::thread_rng();
    let x = [0,1,2,3,4];
    let dice = x.choose(&mut rng).unwrap();
    let oriented: bool = rand::random();

    match dice {
        0 => Tetromino::Straight,
        1 => Tetromino::Square,
        2 => Tetromino::T,
        3 => Tetromino::L(oriented),
        4 => Tetromino::Z(oriented),
        _ => panic!()
    }
 }