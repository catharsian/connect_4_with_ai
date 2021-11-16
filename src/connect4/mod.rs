use std::cmp::{
    PartialEq,
    PartialOrd,
};



const YELLOW: char = 'Y';
const RED: char = 'R';

#[derive(Copy, Clone, PartialEq)]
pub enum Color  {
    Red,
    Yellow,
}

pub enum Connect {
    Diagonally,
    Horizontally, 
    Vertically,
    None,
}

impl Connect {
    pub fn print(&self) {
        match self {
            Connect::Diagonally => print!("Connected diagonally. "),
            Connect::Horizontally => print!("Conected horizontally. "),
            Connect::Vertically => print!("Connected vertically. "),
            _ => { },
        }
    }
}

#[derive(Clone)]
pub struct Display {
    pub arr: [[char; 7]; 6],
}

impl Display {
    pub fn draw(&self) {
        for i in self.arr.iter() {
            println!("+---+---+---+---+---+---+---+");
            println!("|   |   |   |   |   |   |   |");
            print!("|");
            for j in i.iter() {
                print!(" {} |", *j as char);
            }
            println!("");
            println!("|   |   |   |   |   |   |   |");
        }
        println!("+---+---+---+---+---+---+---+");
    }
    pub fn new() -> Display {
        Display {
            arr: [[' '; 7]; 6],
        }
    }
}
#[derive(Clone)]
pub struct Game {
    pub disp:Display,
    pub turn: Color,
}




impl Game{
    pub fn change_turn(&mut self, col: Color) {
        self.turn = col;
    }
    pub fn new() -> Game {
        Game {
            disp: Display::new(),
            turn: Color::Red,
        }
    }

    pub fn make_turn(&mut self, n: usize) -> bool {
        for i in (0..6).rev() {
            if self.disp.arr[i][n] == ' ' {
                if self.turn == Color::Red {
                    self.disp.arr[i][n] = RED;
                    self.turn = Color::Yellow;
                    return true;
                }
                else {
                    self.disp.arr[i][n] = YELLOW;
                    self.turn = Color::Red;
                    return true;
                }
            }
        }
        false
    }
    pub fn hid_check(&self) -> Option<Color> {
        for (y,ys) in self.disp.arr.iter().enumerate() {
            for (x, col) in ys.iter().enumerate() {
                if *col != ' ' {
                    if let (true, _) = self.single_check(x,y,*col) {
                        if *col == RED {
                            return Some(Color::Red);
                        }
                        else {
                            return Some(Color::Yellow);
                        }
                    }
                }
            }
        }
        
        
        None
    }
    pub fn open_check(&self) -> Option<(Color, Connect)> {
        for (y,ys) in self.disp.arr.iter().enumerate() {
            for (x, col) in ys.iter().enumerate() {
                if *col != ' ' {
                    if let (true, con) = self.single_check(x,y,*col) {
                        if *col == RED {
                            return Some((Color::Red, con));
                        }
                        else {
                            return Some((Color::Yellow, con));
                        }
                    }
                }
            }
        }
        
        
        None
    }

    fn single_check(&self,x : usize, y: usize, col: char) -> (bool, Connect) {
        let mut ans = true;
        
        let right = x < 4;
        let left = x >= 3;
        let up = y < 3;
        if right {
            for k in x..x+4 {
                if self.disp.arr[y][k] != col {
                    ans = false;
                    break;
                }
            }
            if ans {
                return (ans, Connect::Horizontally);
            }
            ans = true;
        }
        if up {
            for k in y..y+4 {
                if self.disp.arr[k][x] != col {
                    ans = false;
                    break;
                }
            }
            if ans {
                return (ans, Connect::Vertically);
            }
            ans = true;
            if right {
                for (k,j) in (x..x+4).zip(y..y+4) {
                    if self.disp.arr[j][k] != col {
                        ans = false;
                        break;
                    }
                }
                if ans {
                    return (ans, Connect::Diagonally);
                }
                ans = true;
            }
            if left {
                for (k,j) in (x-3..=x).rev().zip((y..y+4).rev()) {
                    if self.disp.arr[j][k] != col {
                        ans = false;
                        break;
                    }
                }
                if ans {
                    return (true, Connect::Diagonally);
                }
            }
        }
        
        (false, Connect::None)
    }
    pub fn moves(&self) -> Vec<usize> {
        let mut moves: Vec<usize> = Vec::new();
        for i in 0usize..7usize {
            if self.disp.arr[0][i] == ' ' {
                moves.push(i);
            }
        }
        moves
    }
    pub fn no_moves(&self) -> bool {
        self.moves().len() == 0
    }
}



pub struct C4_AI{
    //board: &'a Game,
    color: Color,
    opponent: Color,
}

const MAX_DEPTH: i32 = 15;
const MAX_REWARD: i32 = 5000;
const MIN_REWARD: i32 = -5000;

impl C4_AI {
    pub fn think_of_turn(&self, board: &Game) -> usize{
        let mut best_val = -5000i32;
        let mut best_move = 0usize;
        for i in board.moves() {
            let mut fresh_board = board.clone();

            fresh_board.make_turn(i);

            let this_val = self.alpha_beta(fresh_board, MAX_DEPTH as usize, true, MIN_REWARD, MAX_REWARD);

            if this_val > best_val {
                best_move = i;
                best_val = this_val;
            }
        }
        //self.board.make_turn(best_move);
        best_move
    }
    fn minimax(&self, mut fresh_board: Game, depth: usize, is_maximizing: bool) -> i32 {
        let end = fresh_board.hid_check();
        // if depth > MAX_DEPTH {
        //     return -5;
        // }
        match end {
            Some(end_col) => {
                if end_col == self.color {
                    return 30;
                }
                else {
                    return -50;
                }
            },
            None => { },
        }
        if fresh_board.no_moves() {
            return 0;
        }

        if is_maximizing {
            let mut best_val: i32 = -5000;
            for i in fresh_board.moves() {
                fresh_board.make_turn(i);

                let value = self.minimax(fresh_board.clone(), depth + 1, false);
                best_val = i32::max(value, best_val);
            }
            best_val
        }
        else {
            let mut best_val: i32 = 5000;
            for i in fresh_board.moves() {
                fresh_board.make_turn(i);

                let value = self.minimax(fresh_board.clone(), depth + 1, true);
                best_val = i32::min(value, best_val);
            }
            best_val
        }
        
    }
    fn alpha_beta(&self, mut fresh_board: Game, depth: usize, is_maximizing: bool, mut alpha: i32, mut beta: i32) -> i32{
        
        let end = fresh_board.hid_check();
        if depth == 0 {
            return 0;
        }
        match end {
            Some(end_col) => {
                if end_col == self.color {
                    return MAX_DEPTH + depth as i32;
                }
                else {
                    return (-1 * MAX_DEPTH) - depth as i32;
                }
            },
            None => { },
        }
        if fresh_board.no_moves() {
            return 0;
        }


        if is_maximizing {
            let mut best: i32 = MIN_REWARD;
            for i in fresh_board.moves(){
                let mut next_board = fresh_board.clone();
                next_board.make_turn(i);


                let val = self.alpha_beta(next_board, depth-1, !is_maximizing, alpha, beta);
                //best = i32::max(best,val);
                best = i32::max(best,val);
                alpha = i32::max(best,alpha);
                if alpha <= beta {
                    break;
                }
            }
            best
        }
        else {
            let mut best: i32 = MAX_REWARD;
            for i in fresh_board.moves(){
                let mut next_board = fresh_board.clone();
                next_board.make_turn(i);

                let val = self.alpha_beta(next_board, depth-1, !is_maximizing, alpha, beta);
                //best = i32::min(best,val);
                best = i32::min(best, val);
                beta = i32::min(best, beta);
                if alpha <= beta {
                    break;
                }
            }
            best
        }
    }

    pub fn new(col: Color) -> C4_AI {
        C4_AI {
            //board: game,
            color: col,
            opponent: if col == Color::Yellow { Color::Red }
            else {Color::Yellow},
        }
    }
}