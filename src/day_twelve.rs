use std::{fmt::Display, sync::Mutex};

use actix_web::{
    get, post,
    web::{Data, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;

#[get("/12/board")]
pub async fn board(board_data: Data<BoardData>) -> impl Responder {
    let board = board_data.lock().unwrap();
    HttpResponse::Ok().body(board.to_string())
}

#[post("/12/place/{team}/{column}")]
pub async fn place(path: Path<(Team, u8)>, board_data: Data<BoardData>) -> impl Responder {
    let mut b = board_data.lock().unwrap();
    if b.winner() != State::Incomplete {
        return HttpResponse::ServiceUnavailable().body(b.to_string());
    }

    let (team, column) = path.into_inner();
    if !(1..=4).contains(&column) {
        return HttpResponse::BadRequest().body(b.to_string());
    }

    let res = b.place(team, column);

    if let Err(()) = res {
        return HttpResponse::ServiceUnavailable().body(b.to_string());
    }

    HttpResponse::Ok().body(b.to_string())
}

#[post("/12/reset")]
pub async fn reset(board_data: Data<BoardData>) -> impl Responder {
    let mut b = board_data.lock().unwrap();
    *b = Board::new();
    HttpResponse::Ok().body(b.to_string())
}

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Team {
    Milk,
    Cookie,
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Team::Milk => write!(f, "🥛"),
            Team::Cookie => write!(f, "🍪"),
        }
    }
}

impl Team {
    pub fn to_tile(&self) -> Tile {
        match self {
            Team::Milk => Tile::Milk,
            Team::Cookie => Tile::Cookie,
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

    pub fn count_placed(&self) -> usize {
        self.board.iter().filter(|&t| *t != Tile::Empty).count()
    }

    pub fn place(&mut self, team: Team, column: u8) -> Result<(), ()> {
        // users are 1 indexed, system is 0 indexed
        let column = column - 1;

        const SIZE: usize = 4;
        if column >= SIZE as u8 {
            return Err(());
        }

        let mut placed = false;
        for y in (0..SIZE).rev() {
            let i = y * SIZE + column as usize;
            if self.board[i] == Tile::Empty {
                self.board[i] = team.to_tile();
                placed = true;
                break;
            }
        }

        if !placed {
            return Err(());
        }

        Ok(())
    }

    pub fn winner(&self) -> State {
        let mut cookie = false;
        let mut milk = false;

        let mut winning_states = vec![];
        // All vertical wins
        winning_states.extend(vec![
            [0, 4, 8, 12],
            [1, 5, 9, 13],
            [2, 6, 10, 14],
            [3, 7, 11, 15],
        ]);
        // All horizontal wins
        winning_states.extend(vec![
            [0, 1, 2, 3],
            [4, 5, 6, 7],
            [8, 9, 10, 11],
            [12, 13, 14, 15],
        ]);
        // All diagonal wins
        winning_states.extend(vec![[0, 5, 10, 15], [3, 6, 9, 12]]);

        for state in winning_states {
            let mut cookie_count = 0;
            let mut milk_count = 0;
            for i in state {
                match self.board[i] {
                    Tile::Cookie => cookie_count += 1,
                    Tile::Milk => milk_count += 1,
                    _ => (),
                }
            }
            if cookie_count == 4 {
                cookie = true;
            }
            if milk_count == 4 {
                milk = true;
            }
        }

        let placed = self.count_placed();
        match (cookie, milk, placed) {
            (true, false, _) => State::Cookie,
            (false, true, _) => State::Milk,
            (true, true, _) => State::None,
            (false, false, 16) => State::None,
            _ => State::Incomplete,
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

        match self.winner() {
            State::Cookie => writeln!(f, "{} wins!", Team::Cookie)?,
            State::Milk => writeln!(f, "{} wins!", Team::Milk)?,
            State::None => writeln!(f, "No winner.")?,
            State::Incomplete => (),
        };

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum State {
    Cookie,
    Milk,
    None,
    Incomplete,
}

pub type BoardData = Mutex<Board>;

pub fn board_data() -> BoardData {
    Mutex::new(Board::new())
}
