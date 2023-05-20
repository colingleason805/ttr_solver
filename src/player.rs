use crate::node::Node;

pub struct Player{
    pub name: String,
    pub routes: Vec<(&Node,&Node)>,
    pub points: u32,
}
