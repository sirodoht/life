use std::thread::sleep;
use std::time::Duration;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[derive(Debug)]
struct State {
    generation: u64,
    grid: [[bool; WIDTH]; HEIGHT],
}

fn render(state: &State) {
    for row in state.grid.iter() {
        for square in row.iter() {
            if *square {
                print!("■ ")
            } else {
                print!("□ ")
            }
        }
        println!();
    }
    println!();
}

fn get_number_of_neighbours(state: &State, row: usize, col: usize) -> u64 {
    let mut neighbours: u64 = 0;

    // go clockwise, first top
    if row > 0 {
        if state.grid[row - 1][col] {
            neighbours += 1;
        }
    }

    // top-right
    if row > 0 && col < WIDTH - 1 {
        if state.grid[row - 1][col + 1] {
            neighbours += 1;
        }
    }

    // right
    if col < WIDTH - 1 {
        if state.grid[row][col + 1] {
            neighbours += 1;
        }
    }

    // bottom-right
    if row < HEIGHT - 1 && col < WIDTH - 1 {
        if state.grid[row + 1][col + 1] {
            neighbours += 1;
        }
    }

    // bottom
    if row < HEIGHT - 1 {
        if state.grid[row + 1][col] {
            neighbours += 1;
        }
    }

    // bottom-left
    if row < HEIGHT - 1 && col > 0 {
        if state.grid[row + 1][col - 1] {
            neighbours += 1;
        }
    }

    // left
    if col > 0 {
        if state.grid[row][col - 1] {
            neighbours += 1;
        }
    }

    // top-left
    if row > 0 && col > 0 {
        if state.grid[row - 1][col - 1] {
            neighbours += 1;
        }
    }

    neighbours
}

fn evolve(old_state: State) -> State {
    let mut new_state = State {
        generation: old_state.generation + 1,
        grid: old_state.grid.clone(),
    };

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let is_live = old_state.grid[row][col];
            let cell_neighbours = get_number_of_neighbours(&old_state, row, col);
            if is_live && cell_neighbours < 2 {
                new_state.grid[row][col] = false;
            } else if is_live && cell_neighbours < 4 {
                new_state.grid[row][col] = true;
            } else if is_live {
                new_state.grid[row][col] = false;
            } else if !is_live && cell_neighbours == 3 {
                new_state.grid[row][col] = true;
            }
        }
    }

    new_state
}

fn from_binary(binary_grid: [[u8; WIDTH]; HEIGHT]) -> [[bool; WIDTH]; HEIGHT] {
    let mut bool_grid: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if binary_grid[i][j] == 1 {
                bool_grid[i][j] = true
            }
        }
    }
    bool_grid
}

fn main() {
    let mut state = State {
        generation: 0,
        grid: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, true, true, true, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
    };
    loop {
        render(&state);
        sleep(Duration::from_secs(1));
        state = evolve(state);
    }
}
