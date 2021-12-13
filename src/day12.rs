use std::collections::{HashMap, HashSet};

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for (a, b) in input
        .lines()
        .map(|s| s.split_once('-').expect("no separator in string"))
    {
        graph.entry(a.to_string())
            .or_default()
            .insert(b.to_string());
        graph.entry(b.to_string())
            .or_default()
            .insert(a.to_string());
    }

    let mut todo = Vec::from([Path {
        node: "start".to_string(),
        ..Default::default()
    }]);
    let mut part1 = 0;
    while !todo.is_empty() {
        let mut node = todo.remove(0);
        if node.is_end() {
            part1 += 1;
            continue
        }
        node.enter();
        todo.extend(graph
            .get(&node.node)
            .iter()
            .flat_map(|&v| v.iter())
            .filter(|s| node.can_reenter(s))
            .map(|s| node.next(s)));

    }

    println!("Part2");
    let mut part2 = 0;
    let mut todo = Vec::from([Path {
        node: "start".to_string(),
        ..Default::default()
    }]);
    while !todo.is_empty() {
        let mut node = todo.remove(0);
        if node.is_end() {
            part2 += 1;
            continue
        }
        node.enter();
        todo.extend(graph
            .get(&node.node)
            .iter()
            .flat_map(|&v| v.iter())
            .filter(|s| node.can_reenter(s) || node.maybe_one_more_time(s))
            .map(|s| node.next(s)));

    }
    Ok((part1, part2))
}

#[derive(Debug, Default, Clone)]
struct Path {
    prefix: String,
    node: String,
    visited: HashSet<String>,
    visits: usize,
}

impl Path {
    pub fn can_reenter(&self, node: &str) -> bool {
        !self.visited.contains(node)
    }
    pub fn maybe_one_more_time(&self, node: &str) -> bool {
        assert!(!self.can_reenter(node));
        self.visits == 0 && node != "start"
    }

    pub fn enter(&mut self) {
        if self.node.as_bytes()[0].is_ascii_lowercase() {
            let is_new = self.visited.insert(self.node.clone());
            if !is_new {
                self.visits += 1;
            }
        }
    }

    pub fn next(&self, node: &str) -> Self {
        let prefix = if self.prefix.is_empty() {
            self.node.clone()
        } else {
            format!("{},{}", self.prefix, self.node)
        };
        Path {
            prefix,
            node: node.to_string(),
            visited: self.visited.clone(),
            visits: self.visits,
        }
    }

    pub fn is_end(&self) -> bool {
        self.node == "end"
    }
}


#[cfg(test)]
mod test {
    use crate::day12::main;

    static DATA: &str = concat!(
        "dc-end\n",
        "HN-start\n",
        "start-kj\n",
        "dc-start\n",
        "dc-HN\n",
        "LN-dc\n",
        "HN-end\n",
        "kj-sa\n",
        "kj-HN\n",
        "kj-dc",
    );

    #[test]
    fn solution() {
        let (p1, p2) = main(DATA).expect("invalid data");
        assert_eq!(p1, 19);
        assert_eq!(p2, 103);
    }
}
