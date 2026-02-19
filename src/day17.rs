use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

const CYCLES: usize = 6;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Active,
    Inactive,
}

#[derive(Clone)]
struct GridND {
    dimentions: Box<[usize]>,
    grid: Box<[Cell]>,
}

impl GridND {
    fn neighboorhood(&self, src: usize) -> Vec<usize> {
        let mut neighboorhood = Vec::with_capacity(3usize.pow(self.dimentions.len() as u32));
        neighboorhood.push(src);
        // build neighboorhood by iterativly extruding
        // neighboorhood forward and backwards in each dimention
        let mut dimention_offset = 1;
        for i in 0..self.dimentions.len() {
            for j in 0..neighboorhood.len() {
                // NB: wraps around board in all but the last dimention
                // should not be a problem since board shouldn't be
                // iterated with active cells along the edges
                let forward = neighboorhood[j] + dimention_offset;
                if forward < self.grid.len() {
                    neighboorhood.push(forward);
                }
                if let Some(backward) = neighboorhood[j].checked_sub(dimention_offset) {
                    neighboorhood.push(backward);
                }
            }
            dimention_offset *= self.dimentions[i];
        }
        neighboorhood
    }
}

fn iterate_grid(grid: &GridND, new_grid: &mut GridND) {
    // TODO: limit updates to just centre?
    new_grid
        .grid
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, new_cell)| {
            let active_neighbor_count = grid.neighboorhood(i)[1..]
                .iter()
                .filter(|adj| grid.grid[**adj] == Cell::Active)
                .count();
            *new_cell = match (grid.grid[i], active_neighbor_count) {
                (Cell::Active, 2..=3) => Cell::Active,
                (Cell::Inactive, 3) => Cell::Active,
                _ => Cell::Inactive,
            };
        });
}

fn create_grid(input: &str, n_dimentions: usize) -> GridND {
    let mut dimentions = vec![1 + 2 * CYCLES; n_dimentions].into_boxed_slice();
    dimentions[0] = input.lines().next().unwrap().len() + 2 * CYCLES;
    dimentions[1] = input.lines().count() + 2 * CYCLES;

    let size = dimentions.iter().product();

    let mut grid = GridND {
        dimentions,
        grid: vec![Cell::Inactive; size].into_boxed_slice(),
    };

    // find middle
    let mut start_index = 0;
    let mut offset_length = 1;
    for dim in 0..grid.dimentions.len() {
        start_index += offset_length * CYCLES;
        offset_length *= grid.dimentions[dim]
    }

    // initilize middle
    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.bytes().enumerate() {
            if cell == b'#' {
                grid.grid[start_index + x + y * grid.dimentions[0]] = Cell::Active;
            }
        }
    }
    grid
}

pub fn task1(input: &str) -> String {
    let mut grid = create_grid(input, 3);
    let mut new_grid = grid.clone();

    for _ in 0..CYCLES {
        iterate_grid(&grid, &mut new_grid);
        (grid, new_grid) = (new_grid, grid);
    }

    grid.grid
        .iter()
        .filter(|cell| **cell == Cell::Active)
        .count()
        .to_string()
}

pub fn task2(input: &str) -> String {
    let mut grid = &mut create_grid(input, 4);
    let mut new_grid = &mut grid.clone();

    for _ in 0..CYCLES {
        iterate_grid(grid, new_grid);
        (grid, new_grid) = (new_grid, grid);
    }

    grid.grid
        .iter()
        .filter(|cell| **cell == Cell::Active)
        .count()
        .to_string()
}
