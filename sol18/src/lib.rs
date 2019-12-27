use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

use paths::{Direction, Point};

fn parse_map(map_str: &str) -> Vec<Vec<char>> {
    let mut rows = vec![];
    let mut row = vec![];
    for c in map_str.chars() {
        if c == '\n' {
            rows.push(row);
            row = vec![];
        } else {
            row.push(c);
        }
    }
    rows
}

fn print_info(map: &Vec<Vec<char>>, positions: &Vec<Point<i32>>, keys: &Vec<char>) {
    let mut chars = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let c = map[y][x].clone();
            if positions.contains(&Point::new(x as i32, y as i32)) {
                chars.push('@');
            } else if keys.contains(&c.to_ascii_lowercase()) {
                chars.push('.');
            } else {
                chars.push(c);
            }
        }
        chars.push('\n');
    }
    println!("{}", chars.iter().collect::<String>());
}

fn get_key_paths(
    map: &Vec<Vec<char>>,
    start_points: &Vec<Point<i32>>,
) -> HashMap<Point<i32>, HashMap<Point<i32>, (i32, Vec<char>)>> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut landmarks = start_points.clone();
    for y in 0..height {
        for x in 0..width {
            let c = map[y as usize][x as usize];
            if c.is_ascii_lowercase() {
                landmarks.push(Point::new(x, y));
            }
        }
    }
    let mut shortest_paths = HashMap::new();
    for src_point in landmarks.into_iter() {
        // BFS to find shortest path to each landmark
        let mut horizon = vec![(src_point.clone(), 0, vec![])];
        let mut dest_paths: HashMap<Point<i32>, (i32, Vec<char>)> = HashMap::new();
        let mut seen = HashSet::new();
        seen.insert(src_point.clone());

        while horizon.len() > 0 {
            let mut new_horizon = vec![];

            for (point, path_len, doors) in horizon.iter() {
                for dir in Direction::iter() {
                    let new_point = point.move_in_dir_y_rev(dir);
                    // Point in bounds
                    if new_point.x >= width
                        || new_point.x < 0
                        || new_point.y >= height
                        || new_point.y < 0
                    {
                        continue;
                    }
                    // Point not wall
                    let new_label = map[new_point.y as usize][new_point.x as usize];
                    if new_label == '#' {
                        continue;
                    }
                    // Point not visited
                    if seen.contains(&new_point) {
                        continue;
                    }

                    // print_info(&map, &new_point, &vec![]);
                    seen.insert(new_point.clone());

                    let new_path_len = path_len + 1;
                    let mut new_doors = doors.clone();
                    if new_label.is_ascii_lowercase() {
                        dest_paths.insert(new_point.clone(), (new_path_len, new_doors.clone()));
                    } else {
                        if new_label.is_ascii_uppercase() {
                            new_doors.push(new_label);
                        }
                        // Don't extend path if we find a key
                        new_horizon.push((new_point.clone(), new_path_len, new_doors));
                    }
                }
            }
            horizon = new_horizon;
        }
        shortest_paths.insert(src_point, dest_paths);
    }
    shortest_paths
}

pub fn get_keys(map_str: &str) -> i32 {
    let map = parse_map(map_str);
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut start_points = vec![];
    for y in 0..height {
        for x in 0..width {
            if map[y as usize][x as usize] == '@' {
                start_points.push(Point::new(x, y));
            }
        }
    }
    let num_keys = map_str.chars().filter(|c| c.is_ascii_lowercase()).count();
    println!("Map dims: {} width, {} height", width, height);
    println!("Starts: {:?}", start_points);
    print_info(&map, &start_points, &vec![]);

    // paths between each landmark
    let shortest_paths = get_key_paths(&map, &start_points);

    println!("=====================================================================");

    let mut paths: HashMap<(Vec<Point<i32>>, Vec<char>), i32> = HashMap::new();
    paths.insert((start_points.clone(), vec![]), 0);
    let mut to_visit: PriorityQueue<(Vec<Point<i32>>, Vec<char>), i32> = PriorityQueue::new();
    to_visit.push((start_points, vec![]), 0);

    let mut i = -1;
    loop {
        i += 1;
        let cur = to_visit.pop().unwrap();
        let ((points, keys), neg_path_len) = cur;
        let path_len = -1 * neg_path_len;
        paths.insert((points.clone(), keys.clone()), path_len);

        if keys.len() == num_keys {
            break;
        }

        if i % 2500 == 0 {
            println!(
                "Iteration {} state: best found {} keys",
                i,
                paths
                    .keys()
                    .max_by_key(|(_, ks)| ks.len())
                    .map(|(_, ks)| ks.len())
                    .unwrap_or(0)
            );
            print_info(&map, &points, &keys);
        }
        for (index, point) in points.iter().enumerate() {
            for (dest_point, (dest_path_len, doors)) in shortest_paths.get(&point).unwrap().iter() {
                let dest_label = map[dest_point.y as usize][dest_point.x as usize];

                // Can't go to point cause locked door is in the way
                if doors
                    .into_iter()
                    .any(|d| !keys.contains(&d.to_ascii_lowercase()))
                {
                    continue;
                }

                let mut new_keys = keys.clone();
                if !new_keys.contains(&dest_label) {
                    new_keys.push(dest_label);
                    new_keys.sort();
                }
                let new_path_len = path_len + dest_path_len;

                let mut new_points_list = points.clone();
                new_points_list[index] = dest_point.clone();

                if paths
                    .get(&(new_points_list.clone(), new_keys.clone()))
                    .is_some()
                {
                    continue;
                }

                if let Some((_, neg_prev_path_len)) =
                    to_visit.get(&(new_points_list.clone(), new_keys.clone()))
                {
                    if new_path_len < -1 * neg_prev_path_len {
                        to_visit.change_priority(
                            &(new_points_list.clone(), new_keys.clone()),
                            -new_path_len,
                        );
                    }
                } else {
                    to_visit.push((new_points_list.clone(), new_keys.clone()), -new_path_len);
                }
            }
        }
    }
    paths
        .into_iter()
        .filter(|((_, keys), _)| keys.len() == num_keys)
        .map(|(_, len)| len)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let case = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################\n";
        assert_eq!(get_keys(case), 86);
    }

    #[test]
    fn test_2() {
        let case = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################\n";
        assert_eq!(get_keys(case), 81);
    }

    #[test]
    fn test_3() {
        let case = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        assert_eq!(get_keys(case), 136);
    }
}
