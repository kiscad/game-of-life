use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Read, Write};
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

    fn new_by_loading_map_file(filename: &str) -> std::io::Result<Self> {
        let file = File::open(filename)?;
        let mut text = String::new();
        BufReader::new(file).read_to_string(&mut text);
        let lines: Vec<_> = text.split('\n').map(|s| s.trim()).collect();
        let grid: Vec<_> = lines
            .iter()
            .map(|line| line.split(',').collect::<Vec<_>>())
            .collect();
        let (dimy, dimx) = (grid.len(), grid[0].len());
        let mut cells = vec![];
        for line in grid {
            cells.extend(line.iter().map(|&c| match c {
                "0" => State::Dead,
                "1" => State::Live,
                _ => panic!("Invalid character in file content!"),
            }))
        }
        // println!("{}, {}", dimx, dimy);
        assert!(cells.len() == dimx * dimy);
        Ok(Self {
            dim: GridPos {
                x: dimx as u16,
                y: dimy as u16,
            },
            cells,
        })
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
    queue!(buffer, cursor::Hide)?;
    let mut game = Grid::new_by_loading_map_file("./glider_gun.txt")?;

    for _ in 0..100 {
        game.render(&mut buffer)?;
        sleep(Duration::from_millis(300));
        game.update_grid();
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
