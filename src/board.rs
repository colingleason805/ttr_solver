use crate::{edge::Edge, node::Node};

pub(crate) struct Board{
    edges: Vec<Edge>,
    nodes: Vec<Node>,
}

impl Board {
    fn check_route(&self, route:(&Node, &Node)) -> bool{
        true
    }

    fn dfs(&self)
}