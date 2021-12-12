use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

type Cave = String;

#[derive(Debug)]
pub struct System {
    connections: HashMap<Cave, HashSet<Cave>>,
}

impl std::str::FromStr for System {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut connections = HashMap::new();
        let _ = s
            .lines()
            .map(|l| {
                l.split_once('-')
                    .ok_or_else(|| anyhow!("failed to split line on '-': {}", l))
                    .map(|(start, end)| {
                        connections
                            .entry(String::from(start))
                            .or_insert_with(HashSet::new)
                            .insert(String::from(end));
                        connections
                            .entry(String::from(end))
                            .or_insert_with(HashSet::new)
                            .insert(String::from(start));
                    })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(System { connections })
    }
}

impl System {
    pub fn count_paths_small_cave_once(&self, start_name: &str, end_name: &str) -> usize {
        self.distinct_path_count(start_name, end_name, visit_small_caves_once)
    }

    pub fn count_paths_one_small_cave_twice(&self, start_name: &str, end_name: &str) -> usize {
        self.distinct_path_count(start_name, end_name, visit_single_small_cave_twice)
    }

    fn distinct_path_count(
        &self,
        start_name: &str,
        end_name: &str,
        strategy: fn(&[String], &str) -> bool,
    ) -> usize {
        let mut finished_paths = Vec::new();
        let mut paths = VecDeque::new();
        let empty = HashSet::new();
        paths.push_back(vec![start_name.to_string()]);
        while !paths.is_empty() {
            let path = paths.pop_front().unwrap();
            let last_cave = &path[path.len() - 1];
            self.connections
                .get(last_cave)
                .or(Some(&empty))
                .unwrap()
                .iter()
                .filter(|cave| strategy(&path, cave))
                .for_each(|cave| {
                    let mut path = path.clone();
                    path.push(cave.to_string());
                    if cave == end_name {
                        finished_paths.push(path);
                    } else {
                        paths.push_back(path);
                    }
                });
        }
        finished_paths.len()
    }
}

fn visit_small_caves_once(path: &[String], cave: &str) -> bool {
    !path.contains(&cave.to_string()) || cave.find(char::is_lowercase).is_none()
}

#[allow(clippy::needless_bool)]
#[allow(clippy::if_same_then_else)]
fn visit_single_small_cave_twice(path: &[String], cave: &str) -> bool {
    if !path.contains(&cave.to_string()) {
        true
    } else if cave.find(char::is_lowercase).is_none() {
        true
    } else if cave == "start" {
        false
    } else if no_duplicate_small_caves(path) {
        true
    } else {
        false
    }
}

fn no_duplicate_small_caves(path: &[String]) -> bool {
    let small_caves = path
        .iter()
        .filter(|s| s.find(char::is_uppercase).is_none())
        .collect::<Vec<_>>();
    let dedup = small_caves.iter().collect::<HashSet<_>>();
    small_caves.len() == dedup.len()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_distinct_path_count_1() {
        let input = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let system = input.parse::<System>().unwrap();
        println!("System: {:?}", system);
        assert_eq!(10, system.count_paths_small_cave_once("start", "end"));
        assert_eq!(36, system.count_paths_one_small_cave_twice("start", "end"));
    }

    #[test]
    fn test_distinct_path_count_2() {
        let input = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let system = input.parse::<System>().unwrap();
        assert_eq!(19, system.count_paths_small_cave_once("start", "end"));
        assert_eq!(103, system.count_paths_one_small_cave_twice("start", "end"));
    }
    #[test]
    fn test_distinct_path_count_3() {
        let input = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let system = input.parse::<System>().unwrap();
        assert_eq!(226, system.count_paths_small_cave_once("start", "end"));
        assert_eq!(
            3509,
            system.count_paths_one_small_cave_twice("start", "end")
        );
    }
}
