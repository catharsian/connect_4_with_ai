use std::io;
mod connect4;

use connect4::*;

const DESCRIPTION: &str = "\
This is a two-player game of Connect-4.
Each turn one player has to drop a colored disc into one of seven columns.
Discs are colored Red(R) and Yellow(Y). Red makes first turn.
If one player gets 4 discs connected horizontally, vertically or diagonally, he wins. \
";


fn main(){
    println!("{}",DESCRIPTION);
    println!("Do you want to play with a fresh new AI? Type \"1\" to accept(no quotes).");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    if inp.trim() == "1" {
        let mut game = Game::new();
        let my_col = loop_chose();
        let mut ai = C4_AI::new(not_color(my_col));
        if my_col == Color::Yellow {
            let ai_move = ai.think_of_turn(&game);
            println!("AI makes turn: {}", ai_move+1);
            game.make_turn(ai_move);
            game.disp.draw();      
        }
        loop {
            let mut inp = String::new();
            io::stdin().read_line(&mut inp).unwrap();

            match inp.trim() {
                "quit" => break,
                "r"    => game.change_turn(Color::Red),
                "y"    => game.change_turn(Color::Yellow),
                _      => { },
            }
            let inp: usize = match inp.trim().parse() {
                Err(e) => {println!("You must enter a number."); continue},
                Ok(x) => x,
            };
            if inp > 7 || inp <= 0 {
                continue;
            }
            if game.moves().contains(&(inp-1)) {
                game.make_turn(inp-1);
            }
            else {

                continue;
            }
            game.disp.draw();
            match game.open_check() {
                None => { },
                Some((Color::Red, con)) => {
                    con.print();
                    println!("Red won!");
                    break;
                }
                Some((Color::Yellow, con)) => {
                    con.print();
                    println!("Yellow won!");
                    break;
                }
            }
            let ai_move = ai.think_of_turn(&game);
            println!("AI makes the move: {}", ai_move+1);
            game.make_turn(ai_move);            
            
            
            game.disp.draw();
            match game.open_check() {
                None => { },
                Some((Color::Red, con)) => {
                    con.print();
                    println!("Red won!");
                    break;
                }
                Some((Color::Yellow, con)) => {
                    con.print();
                    println!("Yellow won!");
                    break;
                }
            }
        }
        io::stdin().read_line(&mut String::new()).unwrap();
    }
    else {
        let mut game = Game::new();
        game.disp.draw();
        loop {
            let mut inp = String::new();
            io::stdin().read_line(&mut inp).unwrap();

            match inp.trim() {
                "quit" => break,
                "r"    => game.change_turn(Color::Red),
                "y"    => game.change_turn(Color::Yellow),
                _      => { },
            }
            let inp: usize = match inp.trim().parse() {
                Err(e) => {println!("You must enter a number."); continue},
                Ok(x) => x,
            };
            if inp > 7 || inp <= 0 {
                continue;
            }
            game.make_turn(inp-1);
            game.disp.draw();
            match game.open_check() {
                None => { },
                Some((Color::Red, con)) => {
                    con.print();
                    println!("Red won!");
                    break;
                }
                Some((Color::Yellow, con)) => {
                    con.print();
                    println!("Yellow won!");
                    break;
                }
            }
        }
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}

fn make_two_moves(mut board: Game, turn1: usize, turn2: usize) -> Game {
    board.make_turn(turn1);
    board.make_turn(turn2);
    board
}
fn loop_chose() -> Color {
    println!("Who do you want to play as? Y or R: ");
    loop {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).unwrap();

        match inp.trim().to_ascii_lowercase().as_str() {
            "y" => return Color::Yellow,
            "r" => return Color::Red,
            _ => {},
        }
    }
}
fn not_color(col: Color) -> Color {
    match col {
        Color::Red => Color::Yellow,
        Color::Yellow => Color::Red,
    }
}