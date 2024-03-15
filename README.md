# custom-dag
Provides methods for topological analysis of directed acyclic graphs, i.e. DAG analysis.

# How to use
```
 use custom_dag::{
    Node,
    collitions::CollidingNode,
    Dag,
 };
 let node_a = Node::new(0,None,None);
 let node_b = Node::new(1,Some(0),None);
 let node_c = Node::new(2,None,Some(0));
 let node_d = Node::new(3,Some(0), Some(1));
 let node_e = Node::new(4,Some(2), Some(1));
 let node_f = Node::new(5,Some(3), Some(4));
 let node_list = [node_a, node_b, node_c, node_d, node_e, node_f];
 let ordering = Topology::sort(&node_list).unwrap().unwrap();
 assert!(ordering.len() > 0);
 let mut dag = Dag::new();
 for node in ordering {
     dag.insert(node);
 };
 assert!(dag.is_safe());

```

# Build
```
cargo build --release
```

# Run tests
```
cargo test --release
```

# Build documentation
```
cargo doc
```

# Examples
Examples are provided in the examples directory.
There is also a main example that reads a text file and constructs a dag from the following format.
Database template:
```
N # integer, number of nodes
L R # integers describing a node, L and R = Left and Right parent
L R # integers describing a node, L and R = Left and Right parent, etc...
L R # integers describing a node, L and R = Left and Right parent, etc...
```

It can be run with the command:
```
cargo run --release -- examples/dag-stats/data/dag.txt

AVG SHORT DAG DEPTH: 1.3333334
AVG LONG DAG DEPTH: 2
NUMBER OF ALL PATHS 24
AVG NODES PER PATH 3.5416667
```

# wasm-binding example
There is a wasm-binding example for binding the topological analysis methods into typescript code.
check `./examples/wasm-binding`