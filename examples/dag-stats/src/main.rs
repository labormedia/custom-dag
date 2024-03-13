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
    let Ok(Some(mut shortest_and_longest)) = Topology::shortest_and_longest_paths(&sorted) else { panic!("Wrong topological assumptions for this test data.") };
    // let printable: Vec<(&u32, &(Option<usize>, Option<usize>))> = shortest_and_longest.iter().collect();
    println!("Shortest and longest path sizes : {:?}", shortest_and_longest);

    let aggregation = shortest_and_longest
        .iter_mut()
        .map(|(id, (left, right))| {
            (left, right)
        })
        .fold((
            (0_usize), // Number of nodes without shortest depth 0.
            (0_usize, 0_usize), // sum aggregation of shortest path lengths in the the dag.
            (0_usize,0_usize) // sum aggregation of longest path lengths in the dag..
        ), |mut acc, (shortest, longest)| {
            if let Some(value) = shortest {
                    if value == &0 {
                        acc.0 += 1;
                    }
                    acc.2.0 += *value ;
                    acc.1.0 += 1;
            };
            if let Some(value) = longest { 
                    acc.2.1 += *value;
                    acc.1.1 += 1;
            };
            acc
        });
    println!("aggregation : {aggregation:?}");
    let average_shortest_depth = aggregation.2.0 as f32 / aggregation.1.0 as f32;
    let average_longest_depth = aggregation.2.1 as f32 / aggregation.1.1 as f32;
    println!("averages : {average_shortest_depth} {average_longest_depth}")
}