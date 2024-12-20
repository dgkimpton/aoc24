use crate::{maze::Maze, maze_graph::MazeGraph};

pub fn run_on_string(input: &str, part: u8, is_real: bool) -> Result<i64, String> {
    let maze = Maze::new_from_string(input);
    if !is_real {
        //maze.print();
    }

    let mut graph = MazeGraph::new(&maze);
    graph.identify_shortest_connections(maze.start());
    let result = graph.mark_shortest_path(maze.start(), maze.end());
    if !is_real {
        //graph.print(maze.start(), maze.end());
        graph.print_tight(maze.start(), maze.end());
    }

    Ok(match part {
        1 => result,
        _ => graph.count_seats(),
    })
}
