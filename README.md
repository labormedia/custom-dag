# custom-dag
Provides methods for topological analysis of directed acyclic graphs, i.e. DAG analysis.

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
N # integer, number of nodes
L R # integers describing a node, L and R = Left and Right parent
L R # integers describing a node, L and R = Left and Right parent, etc...
L R # integers describing a node, L and R = Left and Right parent, etc...

It can be run with the command:
```
cargo run --release -- examples/dag-stats/data/dag.txt

AVG SHORT DAG DEPTH: 1.3333334
AVG LONG DAG DEPTH: 2
NUMBER OF ALL PATHS 24
AVG NODES PER PATH 3.5416667
```
