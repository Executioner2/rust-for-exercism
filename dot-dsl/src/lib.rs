pub mod graph {
    use crate::graph::graph_items::{edge, node};
    use std::collections::HashMap;

    fn attrs_to_map<'a, T>(attrs: T) -> HashMap<String, String>
    where
        T: AsRef<[(&'a str, &'a str)]>,
    {
        attrs
            .as_ref()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    /// 要求此宏的使用者必须有 attrs 属性，并且其类型为 HashMap<String, String>
    macro_rules! impl_attributes {
        ($struct:ty) => {
            impl $struct {
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs_to_map(attrs);
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }
            }
        };
    }

    pub struct Graph {
        pub nodes: Vec<node::Node>,
        pub edges: Vec<edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes<T>(mut self, nodes: T) -> Self
        where
            T: AsRef<[node::Node]>,
        {
            self.nodes.extend(nodes.as_ref().iter().cloned());
            self
        }

        pub fn with_edges<T>(mut self, edges: T) -> Self
        where
            T: AsRef<[edge::Edge]>,
        {
            self.edges.extend(edges.as_ref().iter().cloned());
            self
        }

        pub fn node(&self, id: &str) -> Option<&node::Node> {
            self.nodes.iter().find(|&n| n.id == id)
        }
    }

    impl_attributes!(Graph);

    pub mod graph_items {
        use super::*;
        pub mod node {
            use super::*;
            #[derive(Eq, PartialEq, Debug, Clone)]
            pub struct Node {
                pub id: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(id: &str) -> Self {
                    Self {
                        id: id.to_string(),
                        attrs: HashMap::new(),
                    }
                }
            }

            impl_attributes!(Node);
        }

        pub mod edge {
            use super::*;

            #[derive(Eq, PartialEq, Debug, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }
            }

            impl_attributes!(Edge);
        }
    }
}
