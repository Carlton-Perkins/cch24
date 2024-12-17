use std::{fmt::Display, sync::Mutex};

use actix_web::{get, post, web::Data, HttpResponse, Responder};

#[get("/12/board")]
pub async fn board(board_data: Data<BoardData>) -> impl Responder {
    let board = board_data.lock().unwrap();
    HttpResponse::Ok().body(board.to_string())
}

#[post("/12/reset")]
pub async fn reset(board_data: Data<BoardData>) -> impl Responder {
    let mut b = board_data.lock().unwrap();
    *b = Board::new();
    HttpResponse::Ok().body(b.to_string())
}

#[derive(Clone)]
pub enum Tile {
    Empty,
    Wall,
    Milk,
    Cookie,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "⬛"),
            Tile::Wall => write!(f, "⬜"),
            Tile::Milk => write!(f, "🥛"),
            Tile::Cookie => write!(f, "🍪"),
        }
    }
}

pub struct Board {
    board: Vec<Tile>,
}

impl Board {
    pub fn new() -> Self {
        const SIZE: usize = 4;
        Board {
            board: vec![Tile::Empty; SIZE * SIZE],
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const SIZE: usize = 4;
        for y in 0..SIZE {
            write!(f, "{}", Tile::Wall)?;
            for x in 0..SIZE {
                write!(f, "{}", self.board[y * SIZE + x])?;
            }
            write!(f, "{}", Tile::Wall)?;
            writeln!(f)?;
        }
        for _ in 0..SIZE + 2 {
            write!(f, "{}", Tile::Wall)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

pub type BoardData = Mutex<Board>;

pub fn board_data() -> BoardData {
    Mutex::new(Board::new())
}
