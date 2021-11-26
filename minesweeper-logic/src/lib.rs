use rand::{thread_rng, Rng};

const GRID_SIZE: usize = 10;
const BOMB_COUNT: u8 = 10;

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    state: CellState,
    bombed: bool,
    value: u8
}

#[derive(Copy, Clone, Debug)]
pub enum CellState {
    EXPOSED,
    SEALED,
    HIDDEN
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            state: CellState::HIDDEN,
            bombed: false,
            value: 0
        }
    }

    pub fn expose(&mut self) {
        self.state = CellState::EXPOSED;
    }

    pub fn seal(&mut self) {
        match self.state {
            CellState::EXPOSED => (),
            _ => self.state = CellState::SEALED
        }
    }

    pub fn unseal(&mut self) {
        match self.state {
            CellState::EXPOSED => (),
            _ => self.state = CellState::HIDDEN
        }
    }

    pub fn set_bombed(&mut self) {
        if self.value > 0 {
            panic!("Cannot change a valued cell to a bomb cell");
        }
        self.bombed = true;
    }

    pub fn increment(&mut self) {
        if self.bombed {
            panic!("Cannot increment a bombed cell");
        }
        self.value += 1;
    }
}

#[derive(Debug)]
pub struct Grid {
    cells: [[Cell; GRID_SIZE]; GRID_SIZE]
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = Grid {
            cells: [[Cell::new(); GRID_SIZE]; GRID_SIZE]
        };
        grid.assign_bombs();
        grid.calculate_valued_cells();
        grid
    }

    fn assign_bombs(&mut self) {
        let mut rng = thread_rng();
        let mut unique_bomb_count = 0;
        while unique_bomb_count < BOMB_COUNT {
            let row = rng.gen_range(0..GRID_SIZE);
            let col = rng.gen_range(0..GRID_SIZE);
            if self.cells[row][col].bombed == false {
                self.cells[row][col].set_bombed();
                unique_bomb_count += 1;
            }
        }
    }

    fn is_out_of_bounds(row: isize, col: isize) -> bool {
        row < 0 || row > isize::try_from(GRID_SIZE).unwrap() - 1 || col < 0 || col > isize::try_from(GRID_SIZE).unwrap() -1
    }

    fn calculate_valued_cells(&mut self) {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                for adj_i in -1i8..=1i8 {
                    for adj_j in -1i8..=1i8 {
                        let adj_cell_row = isize::try_from(i8::try_from(row).unwrap() + adj_i).unwrap();
                        let adj_cell_col = isize::try_from(i8::try_from(col).unwrap() + adj_j).unwrap();
                        if !Grid::is_out_of_bounds(adj_cell_row, adj_cell_col)
                            && self.cells[usize::try_from(adj_cell_row).unwrap()][usize::try_from(adj_cell_col).unwrap()].bombed == true
                            && self.cells[row][col].bombed == false {
                            self.cells[row][col].increment();
                        }
                    }
                }
            }
        }
    }
}

pub struct GameState {
}

#[cfg(test)]
mod cell_tests {
    use crate::CellState;
    use crate::Cell;

    #[test]
    fn it_should_be_hidden_by_default() {
        let cell = Cell::new();
        matches!(cell.state, CellState::HIDDEN);
    }

    #[test]
    fn it_should_expose_a_cell() {
        let mut cell = Cell::new();
        cell.expose();
        matches!(cell.state, CellState::EXPOSED);
    }

    #[test]
    fn it_should_seal_a_cell() {
        let mut cell = Cell::new();
        cell.seal();
        matches!(cell.state, CellState::SEALED);
    }

    #[test]
    fn it_should_unseal_a_cell_and_make_it_hidden() {
        let mut cell = Cell::new();
        cell.seal();
        cell.unseal();
        matches!(cell.state, CellState::HIDDEN);
    }

    #[test]
    fn it_should_not_be_able_to_seal_or_unseal_an_exposed_cell() {
        let mut cell = Cell::new();
        cell.expose();

        cell.seal();
        matches!(cell.state, CellState::EXPOSED);

        cell.unseal();
        matches!(cell.state, CellState::EXPOSED);
    }

    #[test]
    fn it_should_be_not_be_bombed_by_default() {
        let cell = Cell::new();
        assert_eq!(cell.bombed, false);
    }

    #[test]
    fn it_should_have_a_0_value_by_default() {
        let cell = Cell::new();
        assert_eq!(cell.value, 0);
    }

    #[test]
    #[should_panic(expected = "Cannot change a valued cell to a bomb cell")]
    fn it_should_not_allow_setting_value_for_bombed_cell() {
        let mut cell = Cell::new();
        cell.increment();
        cell.set_bombed();
    }

    #[test]
    #[should_panic(expected = "Cannot increment a bombed cell")]
    fn it_should_not_allow_setting_valued_cell_to_a_bombed_cell() {
        let mut cell = Cell::new();
        cell.set_bombed();
        cell.increment();
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::{BOMB_COUNT, CellState, GRID_SIZE, Grid, Cell};

    #[test]
    fn it_should_initialize_2d_grid_of_hidden_cells() {
        let grid = Grid::new();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let cell = grid.cells[i][j];
                matches!(cell.state, CellState::HIDDEN);
            }
        }
    }

    #[test]
    fn it_should_initialize_specified_number_of_bombs() {
        let grid = Grid::new();
        let mut actual_bomb_count = 0;
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if grid.cells[i][j].bombed == true {
                    actual_bomb_count += 1;
                }
            }
        }
        assert_eq!(actual_bomb_count, BOMB_COUNT);
    }

    #[test]
    fn it_should_be_out_of_bounds() {
        assert_eq!(Grid::is_out_of_bounds(-1, 0), true);
        assert_eq!(Grid::is_out_of_bounds(0, -1), true);
        assert_eq!(Grid::is_out_of_bounds(0, isize::try_from(GRID_SIZE).unwrap() + 1), true);
        assert_eq!(Grid::is_out_of_bounds(isize::try_from(GRID_SIZE).unwrap() + 1, 0), true);
    }

    #[test]
    fn it_should_calculate_the_correct_count_of_adj_bombs() {
        let mut grid = Grid {
            cells: [[Cell::new(); GRID_SIZE]; GRID_SIZE]
        };
        grid.cells[0][0].set_bombed();
        grid.cells[2][2].set_bombed();
        grid.calculate_valued_cells();

        assert_eq!(grid.cells[0][1].value, 1);
        assert_eq!(grid.cells[1][1].value, 2);
    }
}
