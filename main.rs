#[derive(Debug)]
struct State {
    epoch: u64,
    grid: [[bool; 5]; 5],
}

fn render(_state: State) {
    for row in _state.grid.iter() {
        for square in row.iter() {
            if *square {
                print!("■ ")
            } else {
                print!("□ ")
            }
        }
        println!();
    }
}

fn main() {
    let _seed = State {
        epoch: 0,
        grid: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, true, true, true, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
    };
    render(_seed);
}
