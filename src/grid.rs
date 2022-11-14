use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;

const DIM: usize = 100;

const DIRECTIONS: [Move; 4] = [
    Move(0, -1, Direction::Up),
    Move(1, 0, Direction::Right),
    Move(0, 1, Direction::Down),
    Move(-1, 0, Direction::Left)
];

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Move (
    i64,
    i64,
    Direction,
);

#[derive(Copy, Clone)]
pub struct Tile {
    occupied: bool,
    i: usize,
    j: usize,
    tried: [bool; 4],
}

impl Tile {
    fn new() -> Tile {
        Tile {
            occupied: false,
            i: usize::default(),
            j: usize::default(),
            tried: [false; 4],
        }
    }

    fn init(&mut self, i: usize, j: usize) {
        self.i = i;
        self.j = j;
    }

    fn set_occupied(&mut self, visited: bool) {
        self.occupied = visited;
    }

    fn add_tried(&mut self, direction: &Direction) {
        self.tried[match direction {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }] = true
    }

    fn can_try(&self) -> Vec<&Move> {
        self.tried.iter().enumerate().filter_map(|(i, x)| if !x {
            Some(&DIRECTIONS[i])
        } else {
            None
        }).collect()
    }

    fn clear_tried(&mut self) {
        self.tried = [false; 4]
    }
}

pub struct Grid {
    grid: Vec<Vec<Tile>>,
    pub size: usize,
    pub path: Vec<[usize; 2]>,
    undo_path: Vec<[usize; 2]>,
    pub rng: ThreadRng,
    done: bool,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        let rng = rand::thread_rng();
        let mut grid= vec![vec![Tile::new(); size]; size];
        for (i, column) in grid.iter_mut().enumerate() {
            for (j, tile) in column.iter_mut().enumerate() {
                tile.init(i, j);
            }
        }
        Grid {
            grid,
            size: DIM,
            path: vec![],
            undo_path: vec![],
            rng,
            done: DIM == 1,
        }
    }

    pub fn init(&mut self) {
        let first_path = [self.rng.gen_range(0..DIM),
            self.rng.gen_range(0..DIM)];
        self.path.push(first_path);
        self.grid[first_path[0]][first_path[1]].set_occupied(true);
    }

    pub fn next_path(&mut self) {
        let current_pos = self.path.last().unwrap();
        let avail_dir = self.grid[current_pos[0]][current_pos[1]].can_try();
        let mut avail_pos: Vec<Move> = vec![];
        for dir in avail_dir.iter() {
            if (current_pos[0] as i64 + dir.0) >= 0 && (current_pos[1] as i64 + dir.1) >= 0 {
                avail_pos.push(Move(current_pos[0] as i64 + dir.0, current_pos[1] as i64 + dir.1, dir.2.clone()));
            }
        };
        let valid_pos = avail_pos.iter().filter(|x| self.grid.get(x.0 as usize)
            .and_then(|y| y.get(x.1 as usize)
                .and_then(|tile| if tile.occupied { None } else { Some(()) }))
            .is_some());

        match valid_pos.choose(&mut self.rng) {
            Some(tile) => {
                self.grid[current_pos[0]][current_pos[1]].add_tried(&tile.2);
                self.path.push([tile.0 as usize, tile.1 as usize]);
                self.grid[tile.0 as usize][tile.1 as usize].set_occupied(true);
                self.reset_undone()
            },
            None => {
                if self.path.len() == self.size.pow(2) {
                    self.done = true
                } else {
                    self.undo_path()
                }
            }
        }
    }

    fn undo_path(&mut self) {
        let last = self.path.pop();
        match last {
            Some(tile) => {
                self.undo_path.push(tile);
            },
            None => ()
        }
    }

    fn reset_undone(&mut self) {
        for tile in self.undo_path.iter() {
            self.grid[tile[0]][tile[1]].set_occupied(false);
            self.grid[tile[0]][tile[1]].clear_tried();
        }

        self.undo_path = vec![]
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}