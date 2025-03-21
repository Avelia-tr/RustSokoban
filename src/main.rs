use std::{
    env,
    fmt::{self},
    i32, io, usize,
};

use file_reader::parse_file;
mod file_reader;

// in this language struct are the same things as object
// we need to first declare all the field/properties of the object before implementing things in it
struct Board {
    board: Vec<Vec<String>>,
    player: Entity,
    wall: String,
    crates: Vec<Entity>,
    win_char: String,
}

// this represent the player and the crates on the board
struct Entity {
    position: (i32, i32),
    character: String,
}

// the impl is just where we declare the methods in this language
impl Board {
    pub fn move_player(&mut self, input: &str) {
        let input_vector: (i32, i32) = translate_input(input);

        let futur_player_position: (i32, i32) = (
            self.player.position.0 + input_vector.0,
            self.player.position.1 + input_vector.1,
        );

        let inbound_check: bool = self.is_inbound(as_usize(futur_player_position));

        let wall_check: bool = self.board[futur_player_position.0 as usize]
            [futur_player_position.1 as usize]
            == self.wall;

        if inbound_check || wall_check {
            println!("Move Cancelled");
            return;
        }

        // I know that the position is valid since we check it just above

        // now we check if we can move the crate
        let future_crate_position: (i32, i32) = (
            futur_player_position.0 + input_vector.0,
            futur_player_position.1 + input_vector.1,
        );

        let usize_future_crate_position = as_usize(future_crate_position);

        let inbound_crate_check: bool = self.is_inbound(usize_future_crate_position);

        let wall_crate_check: bool =
            self.board[usize_future_crate_position.0][usize_future_crate_position.1] == self.wall;

        // you can consider an Option like a nullable, this basically say that this fonction can
        // fail to return
        let possible_crate: Option<&mut Entity> = self.first_crate_mut(futur_player_position);

        // you can see that like a null check
        if possible_crate.is_some() {
            if inbound_crate_check || wall_crate_check {
                return;
            }

            // don't pay attention to the unwrap there
            possible_crate.unwrap().position = future_crate_position;
        }

        self.player.position = futur_player_position;
    }

    fn is_inbound(&self, position: (usize, usize)) -> bool {
        self.board.len() < position.0 && self.board[position.0].len() < position.1
    }

    // we get the crate at pos X, you can see the Option type here, this just means this fonction
    // can fail (ie : there is no crate satisfying the condition this just let me to check just
    // above )
    fn first_crate(&self, position: (i32, i32)) -> Option<&Entity> {
        self.crates.iter().find(|f| f.position == position)
    }

    // this is necessery bc of some quirks of the language you can ignore it
    fn first_crate_mut(&mut self, position: (i32, i32)) -> Option<&mut Entity> {
        self.crates.iter_mut().find(|f| f.position == position)
    }

    // check if all the crates are under win strings
    fn win_check(&self) -> bool {
        self.crates.iter().all(|f| {
            let pos: (usize, usize) = as_usize(f.position);
            self.board[pos.0][pos.1] == self.win_char
        })
    }
}

// this is just the fonction that transform a board to a string
// fmt::Display is like an interface but you don't need to worry about that
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = Default::default();

        // 0..self.board.len() is just a way of declaring a for loop that iterate on every possible
        //   position
        for row in 0..self.board.len() {
            for col in 0..self.board[row].len() {
                if (row as i32, col as i32) == self.player.position {
                    result += &self.player.character;
                } else {
                    let possible_crate = self.first_crate((row as i32, col as i32));

                    if possible_crate.is_some() {
                        result += &possible_crate.unwrap().character;
                    } else {
                        result += &self.board[row][col];
                    }
                }
            }
            result += "\n";
        }
        write!(f, "{}", result)
    }
}

fn translate_input(input: &str) -> (i32, i32) {
    match input.trim() {
        "w" | "z" => (-1, 0),
        "s" => (1, 0),
        "a" | "q" => (0, -1),
        "d" => (0, 1),
        _ => (0, 0),
    }
}

#[deprecated]
#[rustfmt::skip]
mod unformatted {
    use crate::Entity;

#[allow(dead_code)]
pub fn create_default_board(wall:String,space:String,win_space:String) -> Vec<Vec<String>> {
    vec![
            vec![ wall.clone(), wall.clone(), wall.clone(),  wall.clone(), wall.clone(),wall.clone(), wall.clone()],
            vec![ wall.clone(), space.clone(), space.clone(), space.clone(), space.clone(), space.clone(), wall.clone()],
            vec![ wall.clone(), space.clone(), space.clone(), space.clone(), space.clone(), space.clone(), wall.clone()],
            vec![ wall.clone(), space.clone(), space.clone(), space.clone(), space.clone(), space.clone(), wall.clone()],
            vec![ wall.clone(), space.clone(), space.clone(), space.clone(), space.clone(), space.clone(), wall.clone()],
            vec![ wall.clone(), space.clone(), space.clone(), win_space.clone(), space.clone(), space.clone(), wall.clone()],
            vec![ wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),wall.clone(), wall.clone()],
        ]
    }

#[allow(dead_code)]
pub fn create_crate(crate_character:String) -> Vec<Entity> {
        vec![ Entity { position:(2,3), character: crate_character.clone()},]
    }
}

fn as_usize(position: (i32, i32)) -> (usize, usize) {
    (position.0 as usize, position.1 as usize)
}

fn main() {
    let mut path_to_file = env::args();

    let mut board = parse_file(&path_to_file.nth(1).expect("wrongArg")).expect("smh error");

    let mut input: String = "".to_string();

    loop {
        //main game loop
        println!("{}", board);

        if board.win_check() {
            break;
        }

        println!("wasd or zqsd to move");

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        board.move_player(&input);

        input = String::new();
    }

    println!("You won !!!!");
}
