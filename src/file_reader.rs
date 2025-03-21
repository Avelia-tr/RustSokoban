use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    usize,
};

use crate::{Board, Entity};

// we take a file as arg
// we parse it
// TODO: decide file structure
//
//size of the sprites
//s:<number>
//
//player
//p:<string>
//
//crate
//c:<string>
//
//win
//x:<string>
//
//wall
//w:<string>
//
//separation
//<
//
//board
//
// return a board
//
// exemple of file
//s:2
//p:()
//c:[]
//x:++
//w:==
//<
//==,==,==,==
//==,(),[],==
//==,  ,++,==
//==,==,==,==

#[allow(dead_code)]
struct TemporaryBoard {
    player: String,
    wall: String,
    objective: String,
    crates: String,
}

#[derive(Debug)]
pub enum ParsingError {
    BoardNotFound,
    InvalidLine,
    CantReadFile,
}

fn parse_line<T: FromStr>(file: &mut BufReader<File>, pattern: &str) -> T {
    file.lines()
        .filter_map(|f| {
            f.expect("idk")
                .trim()
                .strip_prefix(pattern)
                .map(|x| x.to_owned())
        })
        .next()
        .and_then(|s| s.parse::<T>().ok())
        .expect(pattern)
}

fn parse_sprite(file: &mut BufReader<File>, size: usize) -> TemporaryBoard {
    let l_player = parse_line::<String>(file, "p:").split_off(size);
    let l_crates = parse_line::<String>(file, "c:").split_off(size);
    let l_objective = parse_line::<String>(file, "x:").split_off(size);
    let l_wall = parse_line::<String>(file, "w:").split_off(size);
    return TemporaryBoard {
        player: l_player,
        wall: l_wall,
        objective: l_objective,
        crates: l_crates,
    };
}

fn parse_tile<'a>(board: &'a mut Board, char_ref: &'a TemporaryBoard, tile: &'a str) -> &'a str {
    let boardlen = board.board.len() - 1;

    if tile == char_ref.wall || tile == char_ref.objective {
        println!("pushed {} at {}", tile, boardlen);
        board.board[boardlen].push(tile.to_string());
    } else if tile == char_ref.crates {
        let pos = (
            board.board.len() as i32,
            board.board.last().iter().len() as i32,
        );

        board.crates.push(Entity {
            character: char_ref.crates.clone(),
            position: pos,
        });
    } else if tile == char_ref.player {
        board.player.character = char_ref.player.clone();
    }

    board.board[boardlen].push("  ".to_string());
    return tile;
}

fn parse_board(
    file: &mut BufReader<File>,
    tempboard: TemporaryBoard,
) -> Result<Board, ParsingError> {
    let mut iterline = file.lines();

    loop {
        let current_line_result = iterline.next();

        if current_line_result.is_none() {
            return Err(ParsingError::BoardNotFound);
        }

        if current_line_result
            .is_some_and(|line| line.is_ok_and(|stringline| stringline == "<".to_string()))
        {
            break;
        }
    }

    let mut l_final_board: Board = Board {
        wall: tempboard.wall.clone(),
        player: Entity {
            position: (0, 0),
            character: tempboard.player.clone(),
        },
        crates: vec![],
        win_char: tempboard.objective.clone(),
        board: vec![],
    };

    loop {
        let current_line = iterline.next();

        if current_line.is_none() {
            return Ok(l_final_board);
        }

        let current_string_line_result = current_line.unwrap();

        if current_string_line_result.is_err() {
            return Err(ParsingError::InvalidLine);
        }

        l_final_board.board.push(Vec::new());

        current_string_line_result
            .unwrap()
            .split(",")
            .into_iter()
            .for_each(|t| {
                parse_tile(&mut l_final_board, &tempboard, t);
            });
    }
}

#[allow(dead_code)]
pub fn parse_file(file_path: &str) -> Result<Board, ParsingError> {
    let file = File::open(file_path);

    if file.is_err() {
        return Err(ParsingError::CantReadFile);
    }

    let mut reader = BufReader::new(file.unwrap());

    let size: usize = parse_line::<usize>(&mut reader, "s:");

    let temp_board_holder = parse_sprite(&mut reader, size);

    parse_board(&mut reader, temp_board_holder)
}

//
//
//
//
//
