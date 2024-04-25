extern crate scaffold;

#[cfg(test)]
mod unit_tests {

    use scaffold::object::GameOfLifeGrid;
    fn fixture_gol_grid() {}
    fn test_gol_can_initialize_with_a_2d_list_containing_any_type() {
        let mut grid = Vec::new();
        for _ in 0..15 {
            grid.push(vec![Some(""); 10])
        }
       let _gol = GameOfLifeGrid::new(grid);
    }
}
