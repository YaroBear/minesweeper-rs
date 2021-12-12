use rand::{thread_rng, Rng};

const GRID_SIZE: usize = 10;
const BOMB_COUNT: u8 = 10;

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub state: CellState,
    pub bombed: bool,
    pub value: u8,
}

#[derive(Copy, Clone, Debug)]
pub enum CellState {
    EXPOSED,
    SEALED,
    HIDDEN,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            state: CellState::HIDDEN,
            bombed: false,
            value: 0,
        }
    }

    pub fn expose(&mut self) {
        self.state = CellState::EXPOSED;
    }

    pub fn seal(&mut self) {
        match self.state {
            CellState::EXPOSED => (),
            _ => self.state = CellState::SEALED,
        }
    }

    pub fn unseal(&mut self) {
        match self.state {
            CellState::EXPOSED => (),
            _ => self.state = CellState::HIDDEN,
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
    pub cells: [[Cell; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = Grid {
            cells: [[Cell::new(); GRID_SIZE]; GRID_SIZE],
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

    #[allow(unused_comparisons)]
    pub fn expose_cell(&mut self, row: usize, col: usize) {
        if Grid::is_out_of_bounds(isize::try_from(row).unwrap(), isize::try_from(col).unwrap()) {
            panic!(
                "The selected coordinates {},{} are outside of the grid",
                row, col
            );
        }
        match self.cells[row][col].state {
            CellState::SEALED => (),
            CellState::EXPOSED => (),
            CellState::HIDDEN => {
                if self.cells[row][col].bombed || self.cells[row][col].value >= 0 {
                    self.cells[row][col].expose();
                    if self.cells[row][col].value == 0 {
                        self.expose_neighbors_of(row, col);
                    }
                }
            }
        }
    }

    pub fn toggle_seal(&mut self, row: usize, col: usize) {
        if Grid::is_out_of_bounds(isize::try_from(row).unwrap(), isize::try_from(col).unwrap()) {
            panic!(
                "The selected coordinates {},{} are outside of the grid",
                row, col
            );
        }
        match self.cells[row][col].state {
            CellState::SEALED => self.cells[row][col].unseal(),
            _ => self.cells[row][col].seal(),
        }
    }

    fn expose_neighbors_of(&mut self, row: usize, col: usize) {
        for adj_i in -1i8..=1i8 {
            for adj_j in -1i8..=1i8 {
                let adj_cell_row = isize::try_from(i8::try_from(row).unwrap() + adj_i).unwrap();
                let adj_cell_col = isize::try_from(i8::try_from(col).unwrap() + adj_j).unwrap();
                if !Grid::is_out_of_bounds(adj_cell_row, adj_cell_col)
                    && !self.cells[row][col].bombed
                {
                    self.expose_cell(
                        adj_cell_row.try_into().unwrap(),
                        adj_cell_col.try_into().unwrap(),
                    );
                }
            }
        }
    }

    fn is_out_of_bounds(row: isize, col: isize) -> bool {
        row < 0
            || row > isize::try_from(GRID_SIZE).unwrap() - 1
            || col < 0
            || col > isize::try_from(GRID_SIZE).unwrap() - 1
    }

    fn calculate_valued_cells(&mut self) {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                for adj_i in -1i8..=1i8 {
                    for adj_j in -1i8..=1i8 {
                        let adj_cell_row =
                            isize::try_from(i8::try_from(row).unwrap() + adj_i).unwrap();
                        let adj_cell_col =
                            isize::try_from(i8::try_from(col).unwrap() + adj_j).unwrap();
                        if !Grid::is_out_of_bounds(adj_cell_row, adj_cell_col)
                            && self.cells[usize::try_from(adj_cell_row).unwrap()]
                                [usize::try_from(adj_cell_col).unwrap()]
                            .bombed
                                == true
                            && self.cells[row][col].bombed == false
                        {
                            self.cells[row][col].increment();
                        }
                    }
                }
            }
        }
    }
}

pub struct Game {
    pub grid: Grid,
    pub state: GameState,
}

pub enum GameState {
    INPROGRESS,
    LOST,
    WON,
}

impl Game {
    pub fn new() -> Self {
        Game {
            grid: Grid::new(),
            state: GameState::INPROGRESS,
        }
    }

    pub fn update_game_state(&mut self) {
        let mut exposed_cell_count = 0;
        for cells in self.grid.cells {
            for cell in cells {
                if let CellState::EXPOSED = cell.state {
                    if cell.bombed == true {
                        self.state = GameState::LOST;
                    } else {
                        exposed_cell_count += 1;
                    }
                }
            }
        }
        if exposed_cell_count == (GRID_SIZE * GRID_SIZE - usize::try_from(BOMB_COUNT).unwrap()) {
            self.state = GameState::WON;
        }
    }
}

#[cfg(test)]
mod cell_tests {
    use crate::Cell;
    use crate::CellState;

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
    use crate::{Cell, CellState, Grid, BOMB_COUNT, GRID_SIZE};

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
        assert_eq!(
            Grid::is_out_of_bounds(0, isize::try_from(GRID_SIZE).unwrap() + 1),
            true
        );
        assert_eq!(
            Grid::is_out_of_bounds(isize::try_from(GRID_SIZE).unwrap() + 1, 0),
            true
        );
    }

    #[test]
    fn it_should_calculate_the_correct_count_of_adj_bombs() {
        let mut grid = Grid {
            cells: [[Cell::new(); GRID_SIZE]; GRID_SIZE],
        };
        grid.cells[0][0].set_bombed();
        grid.cells[2][2].set_bombed();
        grid.calculate_valued_cells();

        // -------------
        // | * | 1 | 0 |
        // | 1 | 2 | 1 |
        // | 0 | 1 | * |

        assert_eq!(grid.cells[0][1].value, 1);
        assert_eq!(grid.cells[1][1].value, 2);
    }

    #[test]
    fn it_should_expose_the_cell_if_its_bombed() {
        let mut grid = Grid {
            cells: [[Cell::new(); GRID_SIZE]; GRID_SIZE],
        };
        grid.cells[0][0].set_bombed();
        grid.expose_cell(0, 0);
        matches!(grid.cells[0][0].state, CellState::EXPOSED);
    }

    #[test]
    fn it_should_expose_the_cell_and_resursively_expose_neighbors_when_its_value_is_0() {
        let mut grid = Grid {
            cells: [[Cell::new(); GRID_SIZE]; GRID_SIZE],
        };
        grid.expose_cell(0, 0);
        for cells in grid.cells {
            for cell in cells {
                matches!(cell.state, CellState::EXPOSED);
            }
        }
    }

    #[test]
    fn it_should_only_expose_the_numbered_cell() {
        let mut grid = Grid {
            cells: [[Cell::new(); GRID_SIZE]; GRID_SIZE],
        };
        grid.cells[1][1].set_bombed();
        grid.expose_cell(0, 0);
        matches!(grid.cells[0][0].state, CellState::EXPOSED);
        for row in 1..GRID_SIZE {
            for col in 1..GRID_SIZE {
                matches!(grid.cells[row][col].state, CellState::HIDDEN);
            }
        }
    }
}

#[cfg(test)]
mod game_tests {
    use crate::{Cell, Game, GameState, Grid, GRID_SIZE};

    #[test]
    fn it_should_initialize_with_the_inprogress_state() {
        let game = Game::new();
        matches!(game.state, GameState::INPROGRESS);
    }

    #[test]
    fn it_should_set_the_game_to_lost_state_when_clicking_a_bomb() {
        let mut game = Game {
            grid: Grid {
                cells: [[Cell::new(); GRID_SIZE]; GRID_SIZE],
            },
            state: GameState::INPROGRESS,
        };
        game.grid.cells[0][0].set_bombed();
        game.grid.expose_cell(0, 0);
        game.update_game_state();
        matches!(game.state, GameState::LOST);
    }

    #[test]
    fn it_should_set_the_game_to_won_state_when_all_other_cells_are_exposed() {
        let mut game = Game {
            grid: Grid {
                cells: [[Cell::new(); GRID_SIZE]; GRID_SIZE],
            },
            state: GameState::INPROGRESS,
        };
        game.grid.cells[0][0].set_bombed();
        game.grid.expose_cell(0, 1); // right
        game.update_game_state();
        matches!(game.state, GameState::INPROGRESS);
        game.grid.expose_cell(1, 1); // diagonal
        game.update_game_state();
        matches!(game.state, GameState::INPROGRESS);
        game.grid.expose_cell(1, 0); // below
        game.update_game_state();
        matches!(game.state, GameState::INPROGRESS);
        game.grid.expose_cell(2, 2); // any random empty cell
        game.update_game_state();
        matches!(game.state, GameState::WON);
    }
}
