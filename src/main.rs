const CELL_SIZE: (i32, i32) = (2, 1);

#[derive(Debug)]
struct Cell {
    pos: (i32, i32),
    size: (i32, i32),
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Live,
    Dead,
}

#[derive(Debug)]
struct Field {
    dim: (i32, i32), // field dimension: (#cells along x-dir, #cells along y-dir)
    cells: Vec<Cell>,
}

impl Field {
    pub fn new(dim: (i32, i32)) -> Self {
        Field {
            dim,
            cells: (0..dim.0 * dim.1)
                .map(|i| Cell {
                    pos: (i % dim.0 * CELL_SIZE.0, i / dim.0 * CELL_SIZE.1),
                    size: CELL_SIZE,
                    state: State::Dead,
                })
                .collect::<Vec<_>>(),
        }
    }

    fn find_neighs(&self, idx: usize) -> Vec<usize> {
        match self.cells[idx].pos {
            // four corners
            (0, 0) => vec![1, self.dim.0, self.dim.0 + 1],
            (0, self.dim.1 - 1) => vec![self.dim.0-2, self.dim.0*2-2, self.dim.0*2-1],
            (self.dim.0 - 1, 0) => vec![],
            (self.dim.0 - 1, self.dim.1 - 1) => vec![],
            // four edges
            (0, _) => vec![],
            (_, 0) => vec![],
            (self.dim.0-1, _) => vec![],
            (_, self.dim.1-1) => vec![],
            // inner field
            (x, y) => vec![],
        }
        // let (x, y) = self.cells[idx].pos;
        // (0..self.cells.len())
        //     .filter(|&i| {
        //         (self.cells[i].pos.0 == x && i32::abs(self.cells[i].pos.1 - y) == 1)
        //             || (self.cells[i].pos.1 == y && i32::abs(self.cells[i].pos.0 - x) == 1)
        //             || (i32::abs(self.cells[i].pos.0 - x) == 1
        //                 && i32::abs(self.cells[i].pos.1 - y) == 1)
        //     })
        //     .collect()
    }

    pub fn count_live_neigh(&self, i: usize) -> i32 {
        self.find_neighs(i).len() as i32
        // .iter()
        // .filter(|&idx| self.cells[*idx].state == State::Dead)
        // .count() as i32
    }
}

fn main() {
    let f = Field::new((3, 3));
    println!(
        "{:?}",
        (0..f.dim.0 * f.dim.1)
            .map(|i| f.count_live_neigh(i as usize))
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_field_new() {
    let f = Field::new((2, 3));
    let positions: Vec<_> = f.cells.iter().map(|x| x.pos).collect();
    assert_eq!(
        positions,
        vec![(0, 0), (2, 0), (0, 1), (2, 1), (0, 2), (2, 2)]
    );
}
