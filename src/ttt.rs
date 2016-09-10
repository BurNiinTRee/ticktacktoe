use std::io;
use std::io::prelude::*;
use super::ttterror::TttError;


const WINNERS: [[u8; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [2, 4, 7],
    [3, 5, 8],
    [0, 4, 8],
    [2, 4, 6]
];





#[derive(Debug, Copy, Clone)]
pub struct Ttt {
    pub board: [Field; 9]
}

impl Ttt {
    pub fn new() -> Ttt {
        Ttt { board: [Field::Empty; 9] }
    }

    pub fn tick(&mut self, field: usize, player: Field) -> Result<(), TttError> {
        if field > 9 || field < 1 {
            return Err(TttError::InvalidField);
        }
        if let Field::Empty = player {
            return Err(TttError::NonPlayer);
        }
        if self.board[field-1] == Field::Empty {
            self.board[field-1] = player;
            Ok(())
        } else {
            Err(TttError::OccupiedField)
        }
    }

    pub fn line(&self, line: usize) -> String {
        let mut output: String = String::with_capacity(7);
        output.push('|');
        for i in 0..3 {
            output.push(match self.board[i+line*3] {
                Field::Empty => (i+line*3+1).to_string().chars().nth(0).unwrap(),
                Field::Circle => 'O',
                Field::Cross => 'X',
            });
            output.push('|');
        }
        output
    }
    pub fn read_input(&mut self, player: Field) -> (Field, usize) {
        println!("Player {}: Enter field to tick", player);
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        if stdin.read_line(&mut buffer).is_ok(){
            let numres = buffer.trim().parse::<usize>();
            if let Ok(num) = numres {
                return (player, num)
            }
        }
        return self.read_input(player);
    }
    pub fn is_won(&self) -> Field {
        let mut winner = 0;
        let players = [Field::Cross, Field::Circle];
        for player in players.into_iter() {
            for win in WINNERS.into_iter() {
                for field in win.into_iter() {
                    match self.board[*field as usize] == *player {
                         true => {
                            winner += 1;
                            continue;
                        },
                        false => {
                            winner = 0;
                            break;
                        }
                    };
                }
                if winner == 3 {
                    return *player;
                }
            };
        }
        return Field::Empty;
    }
}

impl ::std::fmt::Display for Ttt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        fn seperator() -> String {
            "|-----|".to_string()
        }
        write!(f, "{}\n{}\n{}\n{}\n{}\n{}\n{}\n", seperator(), self.line(0), seperator(), self.line(1), seperator(), self.line(2), seperator())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Field {
    Empty,
    Cross,
    Circle
}

impl ::std::fmt::Display for Field {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Field::Empty => write!(f, "Empty Field"),
            Field::Cross => write!(f, "Cross"),
            Field::Circle => write!(f, "Circle"),
        }
    }
}
