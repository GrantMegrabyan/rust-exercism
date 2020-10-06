pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            use super::node::Node;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                pub from: Node,
                pub to: Node,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from_val: &str, to_val: &str) -> Self {
                    Edge {
                        from: Node::new(from_val),
                        to: Node::new(to_val),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|el| (el.0.to_string(), el.1.to_string())).collect();
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub val: String,
                pub attrs: HashMap<String, String>
            }

            impl Node {
                pub fn new(val: &str) -> Self {
                    Node {
                        val: val.to_owned(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|el| (el.0.to_string(), el.1.to_string())).collect();
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|val| val.as_str())
                }
            }
        }
    }

    use std::collections::HashMap;
    use graph_items::{node::Node, edge::Edge};

    #[derive(Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_owned();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_owned();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().map(|el| (el.0.to_string(), el.1.to_string())).collect();
            self
        }

        pub fn get_node(self, node_val: &str) -> Option<Node> {
            for node in self.nodes {
                if node.val == node_val {
                    return Some(node.clone())
                }
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::graph::graph_items::edge::Edge;
    use super::graph::graph_items::node::Node;
    use super::graph::Graph;
    use maplit::hashmap;

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new();

        assert!(graph.nodes.is_empty());

        assert!(graph.edges.is_empty());

        assert!(graph.attrs.is_empty());
    }

    #[test]
    fn test_graph_with_one_node() {
        let nodes = vec![Node::new("a")];

        let graph = Graph::new().with_nodes(&nodes);

        assert!(graph.edges.is_empty());

        assert!(graph.attrs.is_empty());

        assert_eq!(graph.nodes, vec![Node::new("a")]);
    }

    #[test]
    fn test_graph_with_one_node_with_keywords() {
        let nodes = vec![Node::new("a").with_attrs(&[("color", "green")])];

        let graph = Graph::new().with_nodes(&nodes);

        assert!(graph.edges.is_empty());

        assert!(graph.attrs.is_empty());

        assert_eq!(
            graph.nodes,
            vec![Node::new("a").with_attrs(&[("color", "green")])]
        );
    }

    #[test]
    fn test_graph_with_one_edge() {
        let edges = vec![Edge::new("a", "b")];

        let graph = Graph::new().with_edges(&edges);

        assert!(graph.nodes.is_empty());

        assert!(graph.attrs.is_empty());

        assert_eq!(graph.edges, vec![Edge::new("a", "b")]);
    }

    #[test]
    fn test_graph_with_one_attribute() {
        let graph = Graph::new().with_attrs(&[("foo", "1")]);

        let expected_attrs = hashmap! {
            "foo".to_string() => "1".to_string(),
        };

        assert!(graph.nodes.is_empty());

        assert!(graph.edges.is_empty());

        assert_eq!(graph.attrs, expected_attrs);
    }

    #[test]
    fn test_graph_with_attributes() {
        let nodes = vec![
            Node::new("a").with_attrs(&[("color", "green")]),
            Node::new("c"),
            Node::new("b").with_attrs(&[("label", "Beta!")]),
        ];

        let edges = vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue")]),
        ];

        let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

        let expected_attrs = hashmap! {
            "foo".to_string() => "1".to_string(),
            "title".to_string() => "Testing Attrs".to_string(),
            "bar".to_string() => "true".to_string(),
        };

        let graph = Graph::new()
            .with_nodes(&nodes)
            .with_edges(&edges)
            .with_attrs(&attrs);

        assert_eq!(
            graph.nodes,
            vec![
                Node::new("a").with_attrs(&[("color", "green")]),
                Node::new("c"),
                Node::new("b").with_attrs(&[("label", "Beta!")]),
            ]
        );

        assert_eq!(
            graph.edges,
            vec![
                Edge::new("b", "c"),
                Edge::new("a", "b").with_attrs(&[("color", "blue")]),
            ]
        );

        assert_eq!(graph.attrs, expected_attrs);
    }

    #[test]
    fn test_graph_stores_attributes() {
        let attributes = [("foo", "bar"), ("bat", "baz"), ("bim", "bef")];
        let graph = Graph::new().with_nodes(
            &["a", "b", "c"]
                .iter()
                .zip(attributes.iter())
                .map(|(name, &attr)| Node::new(&name).with_attrs(&[attr]))
                .collect::<Vec<_>>(),
        );

        assert_eq!(
            graph
                .get_node("c")
                .expect("node must be stored")
                .get_attr("bim"),
            Some("bef")
        );
    }
}