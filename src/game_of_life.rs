#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn evolve(&self, alive_neighbours: i32) -> Cell {
        match self {
            Cell::Alive => self.evolve_alive_cell(alive_neighbours),
            Cell::Dead => self.evolve_dead_cell(alive_neighbours),
        }
    }

    fn evolve_alive_cell(&self, alive_neighbours: i32) -> Cell {
        match alive_neighbours {
            2 => Cell::Alive,
            3 => Cell::Alive,
            _ => Cell::Dead,
        }
    }

    fn evolve_dead_cell(&self, alive_neighbours: i32) -> Cell {
        match alive_neighbours {
            3 => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    fn evolve(&self) -> Board {
        let mut evolved_cells = self.cells.clone();
        let original_cells = &self.cells;
        for x in 0..=original_cells.len() - 1 {
            for y in 0..=original_cells[x].len() - 1 {
                let alive_neighbours = self.count_alive_neighbours(x as i32, y as i32);
                evolved_cells[x][y] = original_cells[x][y].evolve(alive_neighbours);
            }
        }
        Board {
            cells: evolved_cells,
        }
    }

    fn count_alive_neighbours(&self, x: i32, y: i32) -> i32 {
        let neighbours = [
            self.get_cell_at_position(x - 1, y - 1),
            self.get_cell_at_position(x, y - 1),
            self.get_cell_at_position(x + 1, y - 1),
            self.get_cell_at_position(x - 1, y),
            self.get_cell_at_position(x + 1, y),
            self.get_cell_at_position(x - 1, y + 1),
            self.get_cell_at_position(x, y + 1),
            self.get_cell_at_position(x + 1, y + 1),
        ];
        let alive_neighbours = neighbours
            .iter()
            .filter(|&cell| *cell == Some(Cell::Alive))
            .count();
        alive_neighbours as i32
    }

    fn get_cell_at_position(&self, x: i32, y: i32) -> Option<Cell> {
        let x_is_in_range = x >= 0 && x < self.cells.len() as i32;
        if x_is_in_range && y >= 0 && y < self.cells[x as usize].len() as i32 {
            return Some(self.cells[x as usize][y as usize]);
        }
        None
    }
}

#[cfg(test)]
mod cell_tests {
    use super::Cell;

    #[test]
    fn any_alive_cell_with_less_than_2_alive_neighbours_dies() {
        assert_eq!(Cell::Dead, Cell::Alive.evolve(1));
    }

    #[test]
    fn any_alive_cell_with_more_than_3_alive_neighbours_dies() {
        assert_eq!(Cell::Dead, Cell::Alive.evolve(4));
    }

    #[test]
    fn any_alive_cell_with_3_alive_neighbours_remains_alive() {
        assert_eq!(Cell::Alive, Cell::Alive.evolve(3));
    }

    #[test]
    fn any_dead_cell_with_3_neighbours_becomes_an_alive_cell() {
        assert_eq!(Cell::Alive, Cell::Dead.evolve(3));
    }

    #[test]
    fn any_dead_cell_with_less_than_3_neighbours_remains_dead() {
        assert_eq!(Cell::Dead, Cell::Dead.evolve(2));
    }

    #[test]
    fn any_dead_cell_with_more_than_3_neighbours_remains_dead() {
        assert_eq!(Cell::Dead, Cell::Dead.evolve(4));
    }
}

#[cfg(test)]
mod board_tests {
    use super::Board;
    use super::Cell;

    #[test]
    fn a_dead_board_should_remain_dead_after_one_evolutio() {
        let board = dead_board();

        let result = board.evolve();

        assert_eq!(board, result);
    }

    #[test]
    fn a_board_with_only_one_alive_cell_shoud_die() {
        let cells = vec![
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
        ];
        let board = Board { cells: cells };

        let result = board.evolve();

        assert_eq!(dead_board(), result);
    }

    #[test]
    fn a_board_with_three_cells_in_a_rrow_should_generate_three_cells_in_a_colum() {
        let cells = vec![
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Alive],
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
        ];
        let board = Board { cells: cells };

        let result = board.evolve();

        let expected_cells = vec![
            vec![Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Alive, Cell::Dead],
        ];
        let expected_board = Board {
            cells: expected_cells,
        };
        assert_eq!(expected_board, result);
    }

    #[test]
    fn a_board_with_three_cells_in_a_column_should_generate_three_cells_in_a_row() {
        let cells = vec![
            vec![Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Alive, Cell::Dead],
        ];
        let board = Board { cells: cells };

        let result = board.evolve();

        let expected_cells = vec![
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Alive],
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
        ];
        let expected_board = Board {
            cells: expected_cells,
        };
        assert_eq!(expected_board, result);
    }

    fn dead_board() -> Board {
        let dead_cells = vec![
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
        ];
        return Board { cells: dead_cells };
    }
}
