const CELL_SIZE: (i32, i32) = (2, 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Live,
    Dead,
}

#[derive(Debug)]
struct GridPos {
    x: i32,
    y: i32,
}

impl GridPos {
    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
struct Field {
    dim: GridPos, // field dimension: (#cells along x-dir, #cells along y-dir)
    cells: Vec<State>,
}

impl Field {
    pub fn new(sizex: i32, sizey: i32) -> Self {
        Field {
            dim: GridPos { x: sizex, y: sizey },
            cells: (0..sizex * sizey).map(|_| State::Dead).collect::<Vec<_>>(),
        }
    }

    fn idx2pos(&self, idx: usize) -> GridPos {
        GridPos {
            x: idx as i32 % self.dim.x,
            y: idx as i32 / self.dim.y,
        }
    }

    fn pos2idx(&self, pos: GridPos) -> usize {
        (pos.x + pos.y * self.dim.x) as usize
    }

    fn count_live_neighs(&self, idx: usize) -> usize {
        let GridPos { x, y } = self.idx2pos(idx);
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
        .filter(|(x, y)| *x >= 0 && *x < self.dim.x && *y >= 0 && *y < self.dim.y)
        .map(|&(x, y)| self.pos2idx(GridPos { x, y }))
        .map(|i| self.cells[i])
        .filter(|s| *s == State::Dead)
        .count()
    }
}

fn main() {
    let f = Field::new(3, 3);
    println!(
        "{:?}",
        (0..f.dim.x * f.dim.y)
            .map(|i| f.count_live_neighs(i as usize))
            .collect::<Vec<_>>()
    );
}
