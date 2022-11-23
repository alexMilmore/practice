struct Solution;

use std::collections::HashSet;
type Grid = Vec<Vec<i32>>;
type VisitedSet = HashSet<(usize, usize)>;
// type GridPoint = (usize, usize); // (x, y, cost, steps)
// type Candidate = (GridPoint, f32, usize); // (x, y, cost, steps)

#[derive(Hash, PartialEq, Eq)]
struct GridPoint {
    x: usize,
    y: usize,
}
struct Candidate {
    point: GridPoint,
    steps: usize,
    cost: f32,
}

fn cost(x: usize, y: usize, end_x: usize, end_y: usize) -> f32 {
    (((end_x-x).pow(2) + (end_y-y).pow(2)) as f32).sqrt()
}

fn get_neighbour_points(point: &GridPoint, end_x: usize, end_y: usize) -> Vec<GridPoint> {
    let mut out = vec!();
    if point.x > 0 { out.push(GridPoint{ x: point.x-1, y: point.y}); }
    if point.y > 0 { out.push(GridPoint{ x: point.x, y: point.y-1}); }
    if point.x < end_x { out.push(GridPoint{ x: point.x+1, y: point.y}); }
    if point.y < end_y { out.push(GridPoint{ x: point.x, y: point.y+1}); }

    out
}

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut visited = std::collections::HashSet::<GridPoint>::new();
        let mut candidates = Vec::<Candidate>::new();
        let end_x = grid.len()-1;
        let end_y = grid[0].len()-1;
        let candidate = Candidate{point: GridPoint{x: 0,y: 0}, steps: 0, cost: cost(0, 0, end_x, end_y)};
        loop {
            // if point is end, exit
            if (&candidate).point.x == end_x && (&candidate).point.y == end_y
            { return candidate.steps as i32; }

            // add neighbours to candidates
            for point in get_neighbour_points(&candidate.point, end_x, end_y) {
                match visited.get(&point) {
                    Some(_) => (),
                    None => {
                        if grid[point.x][point.y] == 0 {
                            visited.insert(point);
                        }
                    },
                }
            }




            // get new point
            candidates.sort_by(|a, b| {a.cost.partial_cmp(&b.cost).expect("NaN")});
            let point = candidates.pop();

        }
    }
}

fn main() {
    println!("Hello, world!");
}
