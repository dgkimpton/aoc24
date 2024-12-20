use crate::direction::Direction;
use crate::grid::{Grid, GridRow};
use crate::maze::{Kind, Maze};
use crate::xy::XY;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct CellId(XY);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct RouteId {
    cell_id: CellId,
    direction: Direction,
}

#[derive(Debug)]
struct Cell {
    cell_id: CellId,
    routes: [Route; 4],
    is_on_primary_path: bool,
    is_on_secondary_path: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
enum Turn {
    Forward,
    Left,
    Right,
}

#[derive(Debug)]
struct Route {
    id: RouteId,
    destinations: [Option<WeightedConnection>; 3],
    sources: [Option<RouteId>; 3],
    node_weight: i64,
    is_on_path: bool,
}

#[derive(Debug, Clone, Copy)]
struct WeightedConnection {
    target: RouteId,
    turn: Turn,
    weight: i64,
}

type MazeCell = Option<Cell>;
type MazeMap = Grid<MazeCell>;
type MinPriorityQueue<T> = PriorityQueue<T, Reverse<i64>>;

pub struct MazeGraph {
    map: MazeMap,
    nodes: MinPriorityQueue<RouteId>,
}

impl RouteId {
    fn new(cell_id: CellId, direction: Direction) -> Self {
        Self { cell_id, direction }
    }
}

impl WeightedConnection {
    fn new(target: RouteId, weight: i64, turn: Turn) -> Self {
        Self {
            target,
            turn,
            weight,
        }
    }
}

impl Route {
    fn new(cell_id: CellId, direction: Direction) -> Self {
        Self {
            id: RouteId { cell_id, direction },
            destinations: [None; 3],
            sources: [None; 3],
            node_weight: i64::MAX,
            is_on_path: false,
        }
    }
}

impl Cell {
    fn from_position(position: XY) -> Self {
        let id = CellId(position);
        Self {
            cell_id: id,
            routes: [
                Route::new(id, Direction::North),
                Route::new(id, Direction::South),
                Route::new(id, Direction::East),
                Route::new(id, Direction::West),
            ],
            is_on_primary_path: false,
            is_on_secondary_path: false,
        }
    }
}

fn create_cells_for_maze(maze: &Maze) -> MazeMap {
    let mut map = Grid::<MazeCell>::new();

    for row in 0..maze.row_count() {
        let mut line = GridRow::<MazeCell>::new();

        for col in 0..maze.col_count() {
            let cell_xy = XY::from_rc(row, col);

            line.push(match maze.at(cell_xy) {
                Kind::Wall => None, // no cell for walls, nothing interesting
                Kind::Floor => Some(Cell::from_position(cell_xy)),
            });
        }

        map.push(line);
    }

    map
}

fn populate_connections(map: &mut MazeMap) -> MinPriorityQueue<RouteId> {
    let mut nodes = MinPriorityQueue::<RouteId>::new();

    // map target route back to it's source
    let mut forward_connections = HashMap::<RouteId, RouteId>::new();

    for row in 0..map.row_count() {
        for col in 0..map.col_count() {
            let cell_xy = XY::from_rc(row, col);

            if map.at(cell_xy).is_none() {
                continue;
            }

            let get_cell_id_to_the = |direction: Direction| -> Option<CellId> {
                if let Some(cell) = map.at(cell_xy.offset_with(direction.as_offset())).as_ref() {
                    Some(cell.cell_id)
                } else {
                    None
                }
            };

            let cardinal_targets = [
                get_cell_id_to_the(Direction::North),
                get_cell_id_to_the(Direction::South),
                get_cell_id_to_the(Direction::East),
                get_cell_id_to_the(Direction::West),
            ];

            let connect =
                |owning_cell: Option<CellId>, direction: Direction, weight: i64, turn: Turn| {
                    if let Some(target_cell_id) = owning_cell {
                        let target = RouteId::new(target_cell_id, direction);
                        Some(WeightedConnection::new(target, weight, turn))
                    } else {
                        None
                    }
                };

            if let Some(cell) = map.at_mut(cell_xy) {
                for route in &mut cell.routes {
                    let direction = route.id.direction;
                    let this_cell = Some(cell.cell_id);
                    let next_cell = cardinal_targets[direction as usize];

                    route.destinations = [
                        connect(next_cell, direction, 1, Turn::Forward),
                        connect(this_cell, direction.counter_clockwise(), 1000, Turn::Left),
                        connect(this_cell, direction.clockwise(), 1000, Turn::Right),
                    ];

                    route.sources = [
                        None,
                        Some(
                            route.destinations[Turn::Right as usize]
                                .as_ref()
                                .unwrap()
                                .target,
                        ),
                        Some(
                            route.destinations[Turn::Left as usize]
                                .as_ref()
                                .unwrap()
                                .target,
                        ),
                    ];

                    if let Some(connection) = route.destinations[Turn::Forward as usize].as_ref() {
                        forward_connections.insert(connection.target, route.id);
                    }

                    nodes.push(route.id, Reverse(i64::MAX));
                }
            }
        }
    }

    for (target, source) in forward_connections {
        map.at_mut(target.cell_id.0).as_mut().unwrap().routes[target.direction as usize].sources
            [Turn::Forward as usize] = Some(source);
    }

    nodes
}

impl MazeGraph {
    pub fn new(maze: &Maze) -> Self {
        let mut map = create_cells_for_maze(maze);
        let nodes = populate_connections(&mut map);

        Self { map, nodes }
    }

    fn route_from(&self, route_id: RouteId) -> Option<&Route> {
        if let Some(cell) = self.map.at(route_id.cell_id.0).as_ref() {
            Some(&cell.routes[route_id.direction as usize])
        } else {
            None
        }
    }

    pub fn count_seats(&self) -> i64 {
        let mut count = 0;
        for row in 0..self.map.row_count() {
            for col in 0..self.map.col_count() {
                let pos = XY::from_rc(row, col);

                if let Some(cell) = &self.map.at(pos) {
                    if cell.is_on_primary_path || cell.is_on_secondary_path {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    pub fn mark_primary_shortest_path(&mut self, start: XY, end: XY) -> i64 {
        let current_cell = CellId(end);

        let cell = self.map.at(current_cell.0).as_ref().unwrap();

        let mut best_route: Option<RouteId> = None;
        let mut best_weight = i64::MAX;

        for route_idx in 0..cell.routes.len() {
            let route = &cell.routes[route_idx];

            if route.node_weight < best_weight {
                best_route = Some(route.id);
                best_weight = route.node_weight;
            }
        }

        let mut current_route = best_route;

        while current_route.is_some() {
            if let Some(route) = current_route {
                {
                    if let Some(cell) = self.map.at_mut(route.cell_id.0).as_mut() {
                        cell.is_on_primary_path = true;
                        let route = &mut cell.routes[route.direction as usize];
                        route.is_on_path = true;
                    }
                }

                if route.cell_id.0 == start {
                    break;
                }
                {
                    let mut best_source = None;
                    let mut best_source_score = i64::MAX;

                    if let Some(route) = &self.route_from(route) {
                        for source in route.sources {
                            if let Some(source) = source {
                                if let Some(route) = self.route_from(source) {
                                    if route.node_weight < best_source_score {
                                        best_source_score = route.node_weight;
                                        best_source = Some(route.id);
                                    }
                                }
                            }
                        }
                    }
                    current_route = best_source
                }
            } else {
                current_route = None
            }
        }

        best_weight
    }

    pub fn mark_shortest_path(&mut self, start: XY, end: XY) -> i64 {
        self.mark_primary_shortest_path(start, end);

        let current_cell = CellId(end);

        let cell = self.map.at(current_cell.0).as_ref().unwrap();

        let mut best_route: Option<RouteId> = None;
        let mut best_weight = i64::MAX;

        for route_idx in 0..cell.routes.len() {
            let route = &cell.routes[route_idx];

            if route.node_weight < best_weight {
                best_route = Some(route.id);
                best_weight = route.node_weight;
            }
        }

        let mut current_routes = Vec::<RouteId>::new();
        current_routes.push(best_route.unwrap());
        let mut current_weight = best_weight;

        while let Some(route) = current_routes.pop() {
            {
                if let Some(cell) = self.map.at_mut(route.cell_id.0).as_mut() {
                    cell.is_on_secondary_path = true;
                    let route = &mut cell.routes[route.direction as usize];
                    route.is_on_path = true;
                    current_weight = route.node_weight;
                }
            }

            if route.cell_id.0 == start {
                continue;
            }
            {
                let mut best_sourcees: Vec<RouteId> = Vec::new();

                if let Some(current_node) = self.route_from(route) {
                    for opt_source_node_id in current_node.sources {
                        if let Some(source_node_id) = opt_source_node_id {
                            if let Some(source_node) = self.route_from(source_node_id) {
                                for opt_connection in &source_node.destinations {
                                    if let Some(connection) = opt_connection {
                                        if connection.target == route
                                            && source_node.node_weight + connection.weight
                                                == current_weight
                                        {
                                            // source_node is linked to current_node via connection and has a suitable weighting

                                            if connection.turn == Turn::Forward {
                                                // it's a forward link, implying that the source node was in a different cell
                                                best_sourcees.push(source_node_id);
                                            } else {
                                                // it's a turn, so it is still within the same cell.
                                                // that's fine but only if the next hop is a forward

                                                for opt_hop_node_id in source_node.sources {
                                                    if let Some(hop_node_id) = opt_hop_node_id {
                                                        if let Some(hop_node) =
                                                            self.route_from(hop_node_id)
                                                        {
                                                            for opt_hop_connection in
                                                                &hop_node.destinations
                                                            {
                                                                if let Some(hop_connection) =
                                                                    opt_hop_connection
                                                                {
                                                                    if hop_connection.target
                                                                        == source_node_id
                                                                        && hop_node.node_weight
                                                                            + hop_connection.weight
                                                                            == source_node
                                                                                .node_weight
                                                                    {
                                                                        // hop_node is linked to source_node via hop_connection

                                                                        if hop_connection.turn
                                                                            == Turn::Forward
                                                                        {
                                                                            // it's a forward link, implying that the node was in a different cell
                                                                            // add the source node as the next valid destination
                                                                            best_sourcees.push(
                                                                                source_node_id,
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                for source in best_sourcees {
                    current_routes.push(source);
                }
            }
        }

        best_weight
    }

    pub fn identify_shortest_connections(&mut self, start: XY) {
        self.reduce_node_weight(RouteId::new(CellId(start), Direction::East), 0);

        while let Some((node_pos, prio)) = self.nodes.pop() {
            let prio = prio.0;

            if prio == i64::MAX {
                break;
            }

            let mut updates = vec![];
            if let Some(cell) = self.map.at(node_pos.cell_id.0).as_ref() {
                let node = &cell.routes[node_pos.direction as usize];

                if let Some(update) = self.find_updates_via(node, Turn::Forward, prio) {
                    updates.push(update);
                }
                if let Some(update) = self.find_updates_via(node, Turn::Left, prio) {
                    updates.push(update);
                }
                if let Some(update) = self.find_updates_via(node, Turn::Right, prio) {
                    updates.push(update);
                }
            }

            for update in updates {
                self.reduce_node_weight(update.0, update.1)
            }
        }
    }

    fn find_updates_via(&self, node: &Route, turn: Turn, prio: i64) -> Option<(RouteId, i64)> {
        if let Some(connection) = node.destinations[turn as usize].as_ref() {
            if let Some(target) = self.route_from(connection.target) {
                let target_score_through_here = prio + connection.weight;

                if target_score_through_here < target.node_weight {
                    return Some((target.id, target_score_through_here));
                }
            }
        }
        None
    }

    fn reduce_node_weight(&mut self, id: RouteId, weight: i64) {
        if let Some(cell) = self.map.at_mut(id.cell_id.0) {
            let route = &mut cell.routes[id.direction as usize];
            if route.node_weight <= weight {
                return;
            }
            route.node_weight = weight;
        }

        self.nodes.change_priority(&id, Reverse(weight));
    }

    pub fn print_tight(&self, start: XY, end: XY) {
        for row in 0..self.map.row_count() {
            for col in 0..self.map.col_count() {
                match &self.map.at(XY::from_rc(row, col)) {
                    Some(cell) if cell.cell_id.0 == start => print!("\x1b[30;103mS\x1b[0m"),
                    Some(cell) if cell.cell_id.0 == end => print!("\x1b[30;103mE\x1b[0m"),
                    Some(cell) if cell.is_on_primary_path => print!("\x1b[42;30m \x1b[0m"),
                    Some(cell) if cell.is_on_secondary_path => print!("\x1b[46;30m \x1b[0m"),
                    Some(_) => print!(" "),
                    None => print!("\x1b[40;90m█\x1b[0m"),
                    //None => print!("█"),
                };
            }
            println!()
        }
        println!()
    }

    pub fn print(&self, start: XY, end: XY) {
        let print_route = |route: &Route| {
            if route.node_weight == i64::MAX {
                print!(" MAX ");
            } else if route.node_weight > 99999 {
                print!(" BIG ");
            } else if route.is_on_path {
                print!("\x1b[42;30m{:^5}\x1b[0m", format!("{}", route.node_weight));
            } else {
                print!("{:^5}", format!("{}", route.node_weight));
            }
        };

        for row in 0..self.map.row_count() {
            for col in 0..self.map.col_count() {
                let pos = XY::from_rc(row, col);

                match &self.map.at(pos) {
                    Some(cell) => {
                        print!("     ");
                        print_route(&cell.routes[Direction::North as usize]);
                        print!("     ");
                    }
                    None => print!("███████████████"),
                };
                print!("|");
            }
            println!();

            for col in 0..self.map.col_count() {
                let pos = XY::from_rc(row, col);

                match &self.map.at(pos) {
                    Some(cell) => {
                        print_route(&cell.routes[Direction::West as usize]);

                        if pos == start {
                            print!("\x1b[30;103mSTART\x1b[0m");
                        } else if pos == end {
                            print!("\x1b[30;103m END \x1b[0m");
                        } else if cell.is_on_primary_path {
                            print!("\x1b[42;30m{:^5}\x1b[0m", format!("{},{}", row, col));
                        } else if cell.is_on_secondary_path {
                            print!("\x1b[46;30m{:^5}\x1b[0m", format!("{},{}", row, col));
                        } else {
                            print!("{:^5}", format!("{},{}", row, col));
                        }

                        print_route(&cell.routes[Direction::East as usize]);
                    }
                    None => print!("███████████████"),
                };
                print!("|");
            }
            println!();

            for col in 0..self.map.col_count() {
                let pos = XY::from_rc(row, col);

                match &self.map.at(pos) {
                    Some(cell) => {
                        print!("     ");
                        print_route(&cell.routes[Direction::South as usize]);
                        print!("     ");
                    }
                    None => print!("███████████████"),
                };
                print!("|");
            }
            println!();
            for _ in 0..self.map.col_count() {
                print!("---------------");

                print!("|");
            }
            println!();
        }
        println!()
    }
}
