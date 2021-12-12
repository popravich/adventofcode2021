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
    println!("{:?}", graph);
    println!("size: {}", graph.len());
    println!("Permutations: {}", (1..=graph.len()).map(|i| i as u128).product::<u128>());

    let mut todo = Vec::from([Path {
        node: "start".to_string(),
        // visited: HashSet::from(["start".to_string()]),
        ..Default::default()
    }]);
    let mut x = 0;
    while !todo.is_empty() {
        let mut node = todo.remove(0);
        if node.is_end() {
            println!("Found one: {:?}", node);
            x += 1;
            continue
        } 
        // println!("{:?}", node);
        if !node.can_reenter() {
            continue
        }
        node.enter();
        todo.extend(graph
            .get(&node.node)
            .iter()
            .flat_map(|&v| v.iter())
            .map(|s| node.next(s)));
        
    }
    Ok((x, 0))
}

#[derive(Debug, Default, Clone)]
struct Path {
    prefix: String,
    node: String,
    visited: HashSet<String>,
}

impl Path {
    pub fn can_reenter(&self) -> bool {
        !self.visited.contains(&self.node)
    }

    pub fn enter(&mut self) {
        if self.node.as_bytes()[0].is_ascii_lowercase() {
            self.visited.insert(self.node.clone());
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
        assert_eq!(p2, 0);
    }
}
