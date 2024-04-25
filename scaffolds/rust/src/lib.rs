pub mod object;

#[derive(Copy, Clone)]
enum State {
    ALIVE,
    DEAD,
}

struct Coordinate {
    x: usize,
    y: usize,
}

struct Grid {
    cells: Vec<Vec<State>>,
}