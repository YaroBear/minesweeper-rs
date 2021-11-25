pub struct Cell {
    state: CellState,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            state: CellState::HIDDEN
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
}

pub enum CellState {
    EXPOSED,
    SEALED,
    HIDDEN
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
}
