
use Coordinate;
use {Grid, State};

pub struct GameOfLifeGrid {
    grid: Grid,
}

impl GameOfLifeGrid {
    pub fn new<T>(initial_grid: Vec<Vec<Option<T>>>) -> Self {
        let grid = initial_grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|b| {
                        if b.is_some() {
                            State::ALIVE
                        } else {
                            State::DEAD
                        }
                    })
                    .collect()
            })
            .collect();
        Self {
            grid: Grid { cells: grid },
        }
    }

    fn _is_off_grid(&self, coordinates: &Coordinate) -> bool {
        let height = self.grid.cells.len();
        if 0 <= height && coordinates.x < height {
            !(0 <= coordinates.y && coordinates.y < self.grid.cells[0].len())
        } else {
            true
        }
    }

    fn get_state(&self, coordinates: &Coordinate) -> Option<State> {
        if self._is_off_grid(coordinates) {
            None
        } else {
            Some(self.grid.cells[coordinates.x][coordinates.y])
        }
    }

    fn get_neighbours_states(&self, coordinates: &Coordinate) -> Vec<State> {
        let mut neighbour_states = Vec::new();
        for d_x in [-1, 0, 1] {
            for d_y in [-1, 0, 1] {
                if d_x == 0 && d_y == 0 {
                    continue;
                }
                let neighbour_coordinates = Coordinate {
                    x: coordinates.x + d_x as usize,
                    y: coordinates.y + d_y as usize,
                };
                let neighbour_state = self.get_state(&neighbour_coordinates);
                if let Some(neighbour_state) = neighbour_state {
                    neighbour_states.push(neighbour_state)
                }
            }
        }
        neighbour_states
    }
}
