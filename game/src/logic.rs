use core::iter::Flatten;

const BOARD_SIZE: usize = 30;
type Board = Box<[[Tile; BOARD_SIZE]; BOARD_SIZE]>;
pub struct Logic {
    pub tiles: Board,
}

fn is_border_tile(row: usize, column: usize) -> bool {
    row == 0 || row == BOARD_SIZE - 1 || column == 0 || column == BOARD_SIZE - 1
}

fn populate_tile(row: usize, column: usize) -> Tile {
    todo!()
}

impl Logic {
    pub fn new() -> Self {
        let mut tiles: Board = Board::default();

        // Set initial tiles
        for (row, items) in tiles.iter_mut().enumerate() {
            for (column, tile) in items.iter_mut().enumerate() {
                if is_border_tile(row, column) {
                    *tile = populate_tile(row, column);
                }
            }
        }

        Self { tiles }
    }

    pub fn handle_input(&mut self, input: Input) {
        self.execute_input_tiles();
        self.execute_circuit_tiles();
        self.execute_output_tiles();
    }

    fn execute_input_tiles(&mut self) {
        self.tiles_mut()
            .filter(|t| t.is_input())
            .for_each(|t| t.execute(NeighborTiles::new()));
    }

    fn execute_circuit_tiles(&mut self) {
        self.tiles_mut()
            .filter(|t| t.is_circuit())
            .for_each(|t| t.execute(NeighborTiles::new()));
    }

    fn execute_output_tiles(&mut self) {
        self.tiles_mut()
            .filter(|t| t.is_output())
            .for_each(|t| t.execute(NeighborTiles::new()));
    }

    fn tiles_mut(&mut self) -> Flatten<std::slice::IterMut<'_, [Tile; BOARD_SIZE]>> {
        self.tiles.iter_mut().flatten()
    }
}

pub struct NeighborTiles<'a> {
    pub e: Option<&'a mut Box<Tile>>,
    pub n: Option<&'a mut Box<Tile>>,
    pub ne: Option<&'a mut Box<Tile>>,
    pub nw: Option<&'a mut Box<Tile>>,
    pub s: Option<&'a mut Box<Tile>>,
    pub se: Option<&'a mut Box<Tile>>,
    pub sw: Option<&'a mut Box<Tile>>,
    pub w: Option<&'a mut Box<Tile>>,
}
impl<'a> NeighborTiles<'a> {
    pub fn new() -> Self {
        todo!()
    }
}

pub enum Input {}

pub enum Tile {
    Circuit(Circuit),
    Empty,
    Input(InputType), // TODO: add an input queue? Or should that not be modelled here?
    Output(OutputType),
}
impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}
impl Tile {
    pub fn execute<'a>(&mut self, neighbor_tiles: NeighborTiles<'a>) {
        match self {
            Tile::Circuit(circuit) => {
                circuit.execute(neighbor_tiles);
            }
            Tile::Empty => {}
            Tile::Input(input) => {
                input.execute(neighbor_tiles);
            }
            Tile::Output(output) => {
                output.execute(neighbor_tiles);
            }
        }
    }
    pub fn is_circuit(&self) -> bool {
        todo!()
    }
    pub fn is_empty(&self) -> bool {
        todo!()
    }
    pub fn is_input(&self) -> bool {
        todo!()
    }
    pub fn is_output(&self) -> bool {
        todo!()
    }
}

pub enum InputType {
    Enemy(Enemy),
    None,
    Resource(Resource),
}
impl InputType {
    pub fn execute<'a>(&mut self, neighbor_tiles: NeighborTiles<'a>) {
        match self {
            _ => {
                todo!()
            }
        }
    }
}

pub enum OutputType {
    None,
    Enemy(Enemy),
    Resource(Resource),
}
impl OutputType {
    pub fn execute<'a>(&mut self, neighbor_tiles: NeighborTiles<'a>) {
        match self {
            _ => {
                todo!()
            }
        }
    }
}

pub enum Enemy {
    Small,
    Medium,
    Large,
    Boss,
}
impl Enemy {
    /// Returns the resources the enemy will target in priority.
    pub fn targets(&self) -> &'static [Resource] {
        match self {
            Self::Small => &[Resource::Coal],
            Self::Medium => &[Resource::Coal],
            Self::Large => &[Resource::Coal],
            Self::Boss => &[Resource::Coal],
        };

        todo!("Figure out targets ")
    }
}

pub enum Circuit {}

impl Circuit {
    pub fn execute<'a>(&mut self, neighbor_tiles: NeighborTiles<'a>) {
        match self {
            _ => {
                todo!()
            }
        }
    }
}
pub enum Resource {
    Coal,
    Iron,
    Copper,
    Stone,
}
