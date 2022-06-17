use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
    terminal, Result,
};

const CELL_SIZE: (u16, u16) = (2, 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Live,
    Dead,
}

#[derive(Debug)]
struct GridPos {
    x: u16,
    y: u16,
}

impl GridPos {
    pub fn to_tuple(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
struct Grid {
    dim: GridPos, // field dimension: (#cells along x-dir, #cells along y-dir)
    cells: Vec<State>,
}

impl Grid {
    pub fn new(sizex: u16, sizey: u16) -> Self {
        Grid {
            dim: GridPos { x: sizex, y: sizey },
            cells: (0..sizex * sizey).map(|_| State::Live).collect::<Vec<_>>(),
        }
    }

    fn idx2pos(&self, idx: usize) -> GridPos {
        GridPos {
            x: idx as u16 % self.dim.x,
            y: idx as u16 / self.dim.y,
        }
    }

    fn pos2idx(&self, pos: GridPos) -> usize {
        (pos.x + pos.y * self.dim.x) as usize
    }

    fn count_live_neibos(&self, idx: usize) -> usize {
        let pos = self.idx2pos(idx);
        let (x, y) = (pos.x as i32, pos.y as i32);
        vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .iter()
        .filter(|(x, y)| *x >= 0 && *x < self.dim.x as i32 && *y >= 0 && *y < self.dim.y as i32)
        .map(|&(x, y)| {
            self.pos2idx(GridPos {
                x: x as u16,
                y: y as u16,
            })
        })
        .map(|i| self.cells[i])
        .filter(|s| *s == State::Live)
        .count()
    }

    fn game_rules(state: &State, live_neibos: usize) -> State {
        match (state, live_neibos) {
            (State::Live, nb) if nb < 2 => State::Dead,
            (State::Live, nb) if nb > 3 => State::Dead,
            (State::Dead, nb) if nb == 3 => State::Live,
            _ => *state,
        }
    }

    fn update_grid(&mut self) {
        let neibos: Vec<_> = (0..self.cells.len())
            .map(|i| self.count_live_neibos(i))
            .collect();
        for (s, nb) in self.cells.iter_mut().zip(neibos) {
            *s = Grid::game_rules(s, nb);
        }
    }

    fn render<T: Write>(&self, buffer: &mut T) -> Result<()> {
        queue!(buffer, terminal::Clear(terminal::ClearType::All))?;
        for (i, s) in self.cells.iter().enumerate() {
            let (x, y) = (i as u16 % self.dim.x, i as u16 / self.dim.y);
            for iy in 0..CELL_SIZE.1 {
                for ix in 0..CELL_SIZE.0 {
                    queue!(
                        buffer,
                        cursor::MoveTo(x * CELL_SIZE.0 + ix, y * CELL_SIZE.1 + iy),
                        style::PrintStyledContent(match *s {
                            State::Dead => "█".black(),
                            State::Live => "█".white(),
                        })
                    )?;
                }
            }
        }
        buffer.flush()?;
        Ok(())
    }
}

#[test]
fn test_count_live_neibos() {
    let f = Grid::new(3, 3);
    let live_neibos = (0..f.dim.x * f.dim.y)
        .map(|i| f.count_live_neibos(i as usize))
        .collect::<Vec<_>>();
    assert_eq!(live_neibos, vec![3, 5, 3, 5, 8, 5, 3, 5, 3]);
}

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    let mut buffer = stdout();
    let mut game = Grid::new(32, 32);
    game.render(&mut buffer)?;
    sleep(Duration::from_millis(1000));
    game.update_grid();
    game.render(&mut buffer);

    terminal::disable_raw_mode()?;
    Ok(())
}
