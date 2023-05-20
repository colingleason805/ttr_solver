use node::Node;
use player::Player;

pub mod edge;
pub mod node;
pub mod player;
pub mod board;
pub mod game;

fn main() {

    let nodes: Vec<Node> = create_nodes();
    let mut players: Vec<Player> = create_players();
    let mut routes: Vec<(&Node,&Node)> = create_routes(nodes);


}

fn create_nodes() -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    let node1 = Node{
        name:String::from("Los Angeles"),
    };
    nodes.push(node1);

    let node1 = Node{
        name:String::from("Seattle"),
    };
    nodes.push(node1);

    let node1 = Node{
        name:String::from("Kansas City"),
    };
    nodes.push(node1);

    let node1 = Node{
        name:String::from("New York"),
    };
    nodes.push(node1);

    let node1 = Node{
        name:String::from("Miami"),
    };
    nodes.push(node1);

    let node1 = Node{
        name:String::from("Dallas"),
    };
    nodes.push(node1);

    nodes
}

fn create_players() -> Vec<Player>{
    let mut players: Vec<Player> = Vec::new();

    let player1 = Player{
        name: String::from("Player1"),
        routes: Vec::new(),
        points: 0,
    };
    players.push(player1);

    let player1 = Player{
        name: String::from("Player2"),
        routes: Vec::new(),
        points: 0,
    };
    players.push(player1);

    let player1 = Player{
        name: String::from("Player3"),
        routes: Vec::new(),
        points: 0,
    };
    players.push(player1);

    let player1 = Player{
        name: String::from("Player4"),
        routes: Vec::new(),
        points: 0,
    };
    players.push(player1);

    players
}

fn create_routes(nodes:Vec<Node>) -> Vec<(&Node, &Node)>{
    let mut routes = Vec::new();

    routes.push((&nodes[0], &nodes[4]));
    routes.push((&nodes[1], &nodes[5]));

    routes

}