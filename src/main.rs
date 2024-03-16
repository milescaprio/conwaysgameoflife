use std::thread;

fn square_neighbor_indexes_list(r: i32, c: i32, s: i32) -> Vec::<(i32,i32)> {
    let left = c != 0;
    let right = c != s - 1;
    let top = r != 0;
    let bottom = r != s - 1;
    // Go through:
    //0.0
    //0.0
    //0.0
    //slices either the top or bottom if needed
    let mut ret = Vec::<(i32,i32)>::new();
    for i in (-1 * top as i32)..=(1 * bottom as i32) {
        if left {
            ret.push((r+i,c-1))
        }
        if right {
            ret.push((r+i,c+1))
        }
    }
    if top {
        ret.push((r-1,c))
    }
    if bottom {
        ret.push((r+1,c))
    }
    ret
}

struct Board {
    board: Vec<Vec<bool>>, 
}

impl Board {
    fn next(mut self) -> Board {
        /// Algorithm 1 for next board.
        /// Time complexity for array operations:
        /// 9 * size for neighbor iterating and counting (8 for neighbor, 1 for write in new vec)
        /// size for regenerating
        /// Total 10 * size 
        /// Assumes that the vector is square
        ///
        /// Conway's game of life rules:
        /// Death (by isolation): 0 or 1 adjacent cells
        /// Survival: 2 or 3 adjacent cells
        /// Birth: 3 adjacent cells
        /// Death (by overcrowding): 4+ adjacent cells
        let length = self.board.len();
        let mut neighbor_tracker = vec![vec![0; length]; length];
        for i in 0..length {
            for j in 0..length {
                for x in &square_neighbor_indexes_list(i as i32, j as i32, length as i32) {
                    neighbor_tracker[i][j] += self.board[x.0 as usize][x.1 as usize] as i32;
                }
            }
        }
        for i in 0..length {
            for j in 0..length {
                self.board[i][j] = match neighbor_tracker[i][j] {
                    0 | 1 => false, //isolation
                    2 => self.board[i][j], //survival, only if currently alive
                    3 => true, //survival or birth
                    4..=8 | _ => false, //overcrowding / catchall
                }
            }
        }
        self
    }
    fn print(&self) {
        println!("");
        for i in &self.board {
            for j in i {
                print!("{}",
                    match j {
                        false => ".",
                        true => "O",
                    }
                )
            }
            println!("");
        }
        println!("");
    }
}

fn main() {
    let i = false;
    let O = true;
    let mut board = Board {board: 
    vec![
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,O,i,i,i,i,O,i,i,i,i,i],
        vec![i,i,i,O,O,i,O,O,O,O,i,O,O,i,i,i],
        vec![i,i,i,i,i,O,i,i,i,i,O,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
        vec![i,i,i,i,i,i,i,i,i,i,i,i,i,i,i,i],
    ]};
    loop {
        board = board.next();
        board.print();
        thread::sleep(std::time::Duration::from_millis(100));
    }
}
