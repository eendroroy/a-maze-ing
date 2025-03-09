mod maze;
mod maze_index;
mod neighbour;

pub(crate) use neighbour::neighbours;
pub(crate) use neighbour::D;
pub(crate) use neighbour::L;
pub(crate) use neighbour::R;
pub(crate) use neighbour::U;

pub use maze::Maze;
