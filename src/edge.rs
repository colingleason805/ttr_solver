use crate::node::Node;
use crate::player::Player;
pub (crate) struct Edge{
    node1: &Node,
    node2: &Node,
    length: u32,
    max_owners: u32,
    owners: Vec<&Player>
}