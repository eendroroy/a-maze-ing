use crate::solver::cli::formatter;
use crate::solver::matrix::cli_viz::CliViz;
use crate::solver::matrix::loader::{loader_maze_from_file, parse_node};
use crate::{FROM, PATH, TO};
use amazeing::solver::matrix::dfs;

pub fn visualize() {
    let (maze, from, to) = unsafe {
        (
            loader_maze_from_file(&*PATH),
            parse_node(&*FROM),
            parse_node(&*TO),
        )
    };

    let mut viz = CliViz::from_maze(&maze, '█', '█', '█', formatter::formatter);

    viz.merge_maze(&maze);

    println!("DFS:{}", viz.merged_path(dfs(&maze, from, to, &mut None)));
}
