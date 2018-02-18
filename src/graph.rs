use petgraph;
use petgraph::graph::NodeIndex;

#[derive(Debug)]
pub struct Graph<'a> {
    graph: petgraph::Graph<&'a str, ()>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Graph<'a> {
        Graph {
            graph: petgraph::Graph::new(),
        }
    }
}

pub fn add_node<'a>(graph: &mut Graph<'a>, word: &'a str) -> NodeIndex {
    graph.graph.add_node(word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let graph = Graph::new();
        assert_eq!(graph.graph.node_count(), 0);
    }

    #[test]
    fn add_node_works() {
        let mut graph = Graph::new();
        let node = add_node(&mut graph, "test");
        assert_eq!(graph.graph.node_count(), 1);
        assert_eq!(graph.graph[node], "test");
    }
}
