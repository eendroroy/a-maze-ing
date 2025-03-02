use crate::solver::cli::formatter;
use crate::solver::matrix::cli_viz::CliViz;
use crate::solver::matrix::loader::loader;
use crate::PATH;
use amazeing::solver::matrix::dfs;

pub fn visualize() {
    unsafe {
        let maze = loader(&**&raw const PATH);
        let (from, to) = ((0, 0), (29, 30));

        let mut viz = CliViz::from_maze(&maze, '█', '█', '█', formatter::formatter);

        viz.merge_maze(&maze);

        println!("DFS:{}", viz.merged_path(dfs(&maze, from, to, &mut None)));
    }
}
