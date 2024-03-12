use std::{
    env,
    fs
};
use custom_dag::{
    Node,
    topological::Topology
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut enumerated_lines = contents.lines().enumerate();
    let dag_size: usize = enumerated_lines.next().expect("Wrong file format.").1.parse().expect("Wrong file format.");
    let mut nodes_vector: Vec<Node<u32>> = Vec::with_capacity(dag_size);

    nodes_vector.push(Node::new(1, None, None));
    for (i,node_data) in enumerated_lines {
        let Some((left, right)) = node_data.split_once(" ") else { panic!("Wrong file format.") };
        nodes_vector.push(Node::new((i + 1) as u32, Some(left.parse().expect("Wrong file format.")), Some(right.parse().expect("Wrong file format."))));
    }

    println!("Nodes vector : {nodes_vector:?}");
    let Ok(Some(sorted)) = Topology::sort(&nodes_vector) else { panic!("Wrong topological assumptions for this test data.") };
    let Ok(Some(shortest_and_longest)) = Topology::shortest_and_longest_paths(&sorted) else { panic!("Wrong topological assumptions for this test data.") };
    let printable: Vec<(&u32, &(Option<usize>, Option<usize>))> = shortest_and_longest.iter().collect();
    println!("Shortes and longest path sizes : {:?}", printable);
}