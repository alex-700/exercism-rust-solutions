pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Default, Debug, Eq, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.into(),
                        to: to.into(),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        attrs: attrs.iter().map(|&(a, b)| (a.into(), b.into())).collect(),
                        ..self
                    }
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(String::as_str)
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Eq, PartialEq, Debug, Default)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.into(),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        attrs: attrs.iter().map(|&(a, b)| (a.into(), b.into())).collect(),
                        ..self
                    }
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(String::as_str)
                }
            }
        }
    }

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Graph {
                nodes: nodes.into(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Graph {
                edges: edges.into(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: attrs.iter().map(|&(a, b)| (a.into(), b.into())).collect(),
                ..self
            }
        }

        pub fn get_node(&self, s: &str) -> Option<&Node> {
            self.nodes.iter().find(|&node| node.name == s)
        }
    }
}
