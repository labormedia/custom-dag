use std::{
    env,
    fs,
    error::Error,
};
use custom_dag::{
    Node,
    topological::Topology
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Error reading file.");

    let mut enumerated_lines = contents.lines().enumerate();
    let dag_size: usize = enumerated_lines.next().expect("Invalid file format.").1.parse()?;
    let mut nodes_list: Vec<Node<u32>> = Vec::with_capacity(dag_size);
    let root = Node::new(1, None, None);

    nodes_list.push(root); // Adds the first node to the nodes_list
    for (i,node_data) in enumerated_lines {
        match node_data.split_once(" ") {
            Some((left, right)) => {
                nodes_list.push(Node::new((i + 1) as u32, Some(left.parse().expect("Invalid file format.")), Some(right.parse()?)));
            },
            None => {}
        };
        
    }
    let Some(sorted) = Topology::sort(&nodes_list)? else { panic!("Invalid topological assumptions for this test data.") };
    let Ok(Some(mut shortest_and_longest)) = Topology::shortest_and_longest_paths(&sorted) else { panic!("Invalid topological assumptions for this test data.") };
    let Some(topology) = Topology::from_slice(&nodes_list) else { panic!("Invalid topological assumptions for this test data.") };
    let Some(bfs_all_paths) = Topology::bfs_all_paths(&topology, root.id) else { panic!("Invalid topological assumptions for this test data.") };
    let all_paths_size_sum: usize = bfs_all_paths.iter().map(|path| { path.len() }).sum();
    let average_node_size = all_paths_size_sum as f32/bfs_all_paths.len() as f32;

    println!("Shortest and longest path sizes : {:?}", shortest_and_longest);

    let aggregation = shortest_and_longest
        .iter_mut()
        .map(|(id, (left, right))| {
            (left, right)
        })
        .fold((
            (0_usize), // Number of nodes with shortest depth 0.
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

    let average_shortest_depth = aggregation.2.0 as f32 / aggregation.1.0 as f32;
    let average_longest_depth = aggregation.2.1 as f32 / aggregation.1.1 as f32;
    println!("AVG SHORT DAG DEPTH: {average_shortest_depth}");
    println!("AVG LONG DAG DEPTH: {average_longest_depth}");
    println!("NUMBER OF ALL PATHS {}", bfs_all_paths.len());
    println!("AVG NODES PER PATH {average_node_size}");
    Ok(())
}