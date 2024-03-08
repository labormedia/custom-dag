use custom_dag::{
    Dag,
    Node,
};

fn main() {
    let mut dag = Dag::new();
    for i in 1..1000 {
        let node_id: u32 = i;
        dag.insert(Node::new(node_id, None, None));
    }
}