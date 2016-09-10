mod ttterror;
mod ttt;
#[cfg(test)]
mod tests;

use ttt::{Ttt, Field};


fn main() {
    let mut running: bool = true;
    let mut turn: Field = Field::Cross;
    let mut game = Ttt::new();
    while running {
        println!("{}", game);
        let (player, cell) = game.read_input(turn);
        println!("{} ticked {}", player, cell);
        match game.tick(cell, player){
            Ok(_) => {},
            Err(ttterror::TttError::OccupiedField) => {
                println!("This field is already occupied!\nchoose a free one!");
                continue;
            },
            Err(ttterror::TttError::InvalidField) => {
                println!("Please enter a number between 1 and 9");
                continue;
            },
            Err(ttterror::TttError::NonPlayer) => {
                println!("Fatal Error!!!\nexiting...");
                std::process::exit(1);
            },
        };
        match game.is_won() {
            Field::Empty => {},
            player => {
                running = false;
                println!("{} won!!!!!\nCongratulations", player);
            }
        }
        turn = match turn {
            Field::Cross => Field::Circle,
            Field::Circle => Field::Cross,
            _ => unreachable!()
        }
    }
    print!("{}", game);
}
