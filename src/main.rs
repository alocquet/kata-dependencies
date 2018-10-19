use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Graph<'a> {
    dependencies: HashMap<&'a str, Vec<&'a str>>
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph { dependencies: HashMap::new() }
    }

    fn get_deps(&self, node: &'a str) -> Vec<&str> {
		let mut visited = vec!();
		self.get_deps_recursive(node, &mut visited)
    }

    fn get_deps_recursive(&self, node: &'a str, visited: &mut Vec<&'a str>) -> Vec<&str> {
        if visited.contains(&node) {
            return vec!();
        }
        visited.push(node);
        match self.dependencies.get(node) {
            Some(val) => {
                let mut result = vec!();
                for elt in val.to_vec() {
                    if !visited.contains(&elt) {
                        result.push(elt)
                    }
                    result.append(&mut self.get_deps_recursive(elt, visited));
                }
                result
            }
            None => vec!()
        }
    }

    fn add_dep(&mut self, node: &'a str, deps: Vec<&'a str>) {
        self.dependencies.insert(node, deps);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_deps() {
        let deps = Graph::new();
        let expected: Vec<&str> = vec!();
        assert_eq!(deps.get_deps("a"), expected);
    }

    #[test]
    fn a_dep_b() {
        let mut deps = Graph::new();
        deps.add_dep("a", vec!("b"));
        assert_eq!(deps.get_deps("a"), vec!("b"));
    }

    #[test]
    fn a_dep_b_c() {
        let mut deps = Graph::new();
        deps.add_dep("a", vec!("b", "c"));
        assert_eq!(deps.get_deps("a"), vec!("b", "c"));
    }

    #[test]
    fn a_dep_b_dep_c() {
        let mut deps = Graph::new();
        deps.add_dep("a", vec!("b"));
        deps.add_dep("b", vec!("c"));
        assert_eq!(deps.get_deps("a"), vec!("b", "c"));
    }

    #[test]
    fn a_dep_b_dep_a() {
        let mut deps = Graph::new();
        deps.add_dep("a", vec!("b"));
        deps.add_dep("b", vec!("a"));
        assert_eq!(deps.get_deps("a"), vec!("b"));
    }

    #[test]
    fn a_dep_b_dep_c_d() {
        let mut deps = Graph::new();
        deps.add_dep("a", vec!("b"));
        deps.add_dep("b", vec!("c", "d"));
        assert_eq!(deps.get_deps("a"), vec!("b", "c", "d"));
    }
}