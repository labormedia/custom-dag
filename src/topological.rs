use core::{
    hash::Hash,
};
use std::{
    collections::{
        HashSet,
        HashMap,
    },
    error::Error,
    fmt,
};
use crate::{
    Node,
    collitions::CollidingNode,
    Dag,
};

#[derive(Debug)]
enum TopologicalError {
    Custom,
    RepeatedNodes,
    WrongTopologicalAssumptions,
    NotADag,
    FirstNodeHasIncomingEdges,
}

impl Error for TopologicalError {}

impl fmt::Display for TopologicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom => write!(f, "Custom error"),
            Self::RepeatedNodes => write!(f, "The list has repeated nodes."),
            Self::WrongTopologicalAssumptions => write!(f, "Wrong topological assumptions."),
            Self::NotADag => write!(f, "Provided list does not conform to a DAG."),
            Self::FirstNodeHasIncomingEdges => write!(f, "List assumptions are not met, i.e. first node should not have incoming edges.")
        }
    }
}

/// Topology struct layout for analysis.
#[derive(Debug, Clone)]
pub struct Topology<T: Eq + Hash + PartialEq + Copy + std::fmt::Debug> {
    all_nodes: HashSet<CollidingNode<T>>, // collection of all nodes compared by all fields
    unique_nodes: HashMap<T, Node<T>>,
    collitions: HashSet<CollidingNode<T>>,
    repeated_nodes: HashMap<T, HashSet<CollidingNode<T>>>,
    outgoing_edges: HashMap<T, Vec<T>>,
}

/// Implements
impl<T: Eq + Hash + PartialEq + Copy + std::fmt::Debug> Topology<T> {
    /// New Topology layout.
    fn new() -> Topology<T> {
        Topology {
            all_nodes: HashSet::new(),
            unique_nodes: HashMap::new(),
            collitions: HashSet::new(),
            repeated_nodes: HashMap::new(),
            outgoing_edges: HashMap::new(),
        }
    }
    /// Inserts the node into the topology analysis.
    /// If it finds a collition, returns an Option<CollidingNode<T>> with the value of the node.
    /// otherwise it returns None if there is no collision.
    fn insert(&mut self, node: Node<T>) -> Option<CollidingNode<T>>{
        if self.all_nodes.insert(node.into()) { // if the node didn't existed in the collection of all nodes (compared by all fields)
            if self.get_unique_node_by_id(node.id) == None { // if the node didn't existed in the collection of unique nodes (compared by id)
                assert_eq!(self.unique_nodes.insert(node.id, node), None); // Inserts node to the unique nodes collection.
                self.collect_edges(node); // collect the node's edges
                None
            } else { // else if the node was already inserted in the collection of unique nodes (compared by id)
                if self.collitions.insert(node.into()) { // insert the node to the collection of collitions; if the node was not listed there
                    // done
                } else {
                    self.collect_repeated_node(node); // also collect the node as a repeated_node
                };
                Some(node.into())  // return the colliding node.
            }
        } else { // else if the node existed in the collection of all nodes (compared by all fields)
            assert_eq!(self.get_unique_node_by_id(node.id), Some(node));  // It should have been already added to the collection of unique nodes (compared by id).
            self.collect_repeated_node(node); // and collects it to the collection of repeated nodes.
            Some(node.into()) // return the colliding node
        }
    }
    /// Constructs a topology from a slice of nodes.
    /// If a consistent DAG topology can be constructed, 
    /// it returns and Option with the topology,
    /// otherwise it returns None.
    fn from_slice(node_list:&[Node<T>]) -> Option<Self> {
        let mut topology = Topology::new();
        for node in node_list {
            topology.insert(*node);
        };
        if topology.is_consistent() {
            Some(topology)
        } else {
            None
        }
    }
    fn collect_repeated_node(&mut self, node: Node<T>) {
        if let Some(repeated_nodes_set) = self.repeated_nodes.get_mut(&node.id) { // if the set of repeated nodes is already created for this node.id
            repeated_nodes_set.insert(node.into());  // insert the node into the collection of repeated nodes
        } else {  // else, if the set of repeated nodes have not been created yet for this node.id
            let mut new_set: HashSet<CollidingNode<T>> = HashSet::new(); // creates a new set
            new_set.insert(node.into());  // inserts the node into the set
            assert_eq!(self.repeated_nodes.insert(node.id, new_set), None); // and inserts the set into the collection of repeated nodes for this node.id
        };
    }
    fn collect_edges(&mut self, node: Node<T>) {
        match node.left {
            None => {},
            Some(ancestor) => {
                if let Some(edges_for_ancestor) = self.outgoing_edges.get_mut(&ancestor) { // pushes the node.id to the list of its ancestor's directed edges (directed to it)
                    edges_for_ancestor.push(node.id);  // Inserts the edge 
                } else {
                    assert_eq!(self.outgoing_edges.insert(ancestor, vec!(node.id)), None);  // Inserts the edge and asserts that the list didn't existed before.
                }
            },
        };
        match node.right {
            None => {},
            Some(ancestor) => {
                if let Some(edges_for_ancestor) = self.outgoing_edges.get_mut(&ancestor) { // pushes the node.id to the list of its ancestor's directed edges (directed to it
                    edges_for_ancestor.push(node.id); // Inserts the edge
                } else {
                    assert_eq!(self.outgoing_edges.insert(ancestor, vec!(node.id)), None); // Inserts the edge and asserts that the list didn't existed before.
                }
            },
        };
    }
    fn get_outgoing_edges_by_id(&self, id: T) -> Option<&Vec<T>> {
        self.outgoing_edges.get(&id)
    }
    fn edge_sum(&self) -> usize {
        self
            .outgoing_edges
            .iter()
            .map(|(from, list)| { list.len() })
            .sum()
    }
    fn get_unique_node_by_id(&self, id:T) -> Option<Node<T>> {
        self.unique_nodes.get(&id).copied()
    }
    /// Checks the consistency of nodes with its references, 
    /// i.e. checks: 
    /// * if all nodes references have been defined -> true
    /// * if there are no repeated nodes -> true
    /// * if there are no collitions -> true
    /// otherwise returns false
    /// References to itself are not considered.
    fn is_consistent(&self) -> bool {
        self.repeated_nodes.len() == 0
        && self.collitions.len() == 0
        && self.unique_nodes
            .iter()
            .fold(true, |acc, node| {
                acc &&
                ( node.1.left == None || self.get_unique_node_by_id(node.1.left.expect("Wrong value assumption")) != None ) &&
                ( node.1.right == None || self.get_unique_node_by_id(node.1.right.expect("Wrong value assumption")) != None )
            })
    }
    /// Tries to build a topological sort from a list of nodes.
    /// Returns a sequence of nodes that follows a topological order if it exists, otherwise it returns None.
    pub fn sort(nodes:&[Node<T>]) -> Result<Option<Vec<Node<T>>>, Box<dyn Error> > {
        let mut topology: Topology<T> = Topology::new();
        for node in nodes.iter() {
            topology.insert(*node);
        };
        let checked_topology = if topology.collitions.len() == 0
            && topology.repeated_nodes.len() == 0
            && topology.is_consistent()
            { Some(&topology) } else { None };
        match checked_topology {
            Some(topology_for_sorting) => { // sorting algorithm
                let mut in_degree_map: HashMap<&T,usize> =
                    topology_for_sorting
                        .unique_nodes
                        .iter()
                        .map( |node| {
                            (node.0, node.1.in_degree())
                        })
                        .collect();
                let mut ordering: Vec<Node<T>> = Vec::with_capacity(in_degree_map.len());
                let ordering = loop {
                    let next_ordering: Vec<T> = in_degree_map
                        .clone()
                        .into_iter()
                        .filter( |(id,in_degree)| { 
                            in_degree == &0_usize 
                        })
                        .map( |(id, degree)| {
                            *id
                        })
                        .collect();
                    if next_ordering.len() > 0 {
                        for id in next_ordering.iter() {
                            ordering.push(topology_for_sorting.get_unique_node_by_id(*id).ok_or(TopologicalError::WrongTopologicalAssumptions)?);
                            match topology_for_sorting.outgoing_edges.get(id) {
                                Some(edges) => {
                                    assert!(edges.len() > 0); // if it was inserted, should have values.
                                    for outgoing_node_id in edges {
                                        let in_degree = in_degree_map.get_mut(outgoing_node_id).ok_or(TopologicalError::WrongTopologicalAssumptions)?;
                                        assert!( in_degree > &mut 0 );
                                        *in_degree -= 1;
                                    };
                                    Some(edges)
                                },
                                None => None
                            };
                            assert_eq!(in_degree_map.remove(id), Some(0_usize));
                        }
                    } else { break ordering; };
                };
                if ordering.len() == topology_for_sorting.unique_nodes.len() { Ok(Some(ordering)) } else { Ok(None) }
            },
            None => Ok(None)
        }
    }
    /// Calculates the shortest and longest paths from a list of nodes from the *first* node of the list.
    /// Because the algorithm assumes the first node is the starting node from which to calculate distances,
    /// it should not have incoming edges, i.e. left and right reference are None, otherwise a FirstNodeHasIncomingEdges error is returned.
    /// This methods relies on Single Source Shortest and Longest (negated) Path algorithm.
    pub fn shortest_and_longest_paths(nodes:&[Node<T>]) -> Result<Option<HashMap<T, (Option<usize>, Option<usize>)>>, Box<dyn Error> > {
        if nodes.len() > 0  {
            if nodes[0].left != None || nodes[0].right != None
            {
                return Err(Box::new(TopologicalError::FirstNodeHasIncomingEdges));
            }
        } else 
        {
            return Ok(None); // list is empty.
        };
        let mut lengths_map: HashMap<T, (Option<usize>, Option<usize>)> = HashMap::new(); // HashMap for accumulating shortest and longest paths for each node in the list.
        if let Some(topological_order) = Self::sort(nodes)? {  // This algorithm assumes that the list nodes conforms to a topological sort
            assert!(topological_order.len() == nodes.len(), "Wrong value assumptions.");  // If there exists a topological sort, it includes all unique nodes.
            let mut topology: Topology<T> = Topology::new();
            for node in nodes.iter() {
                topology.insert(*node);
                lengths_map.insert(node.id, (None, None));  // initiates lengths as None for all nodes in the list.
            };
            let mut outgoing_edges = // instantiates a variable with the outgoing edges of all nodes.
                if nodes[0] == topological_order[0] // This assumption relies on the sorting algorithm.
                && topology.collitions.len() == 0
                && topology.repeated_nodes.len() == 0
                && topology.unique_nodes.len() == nodes.len()
                && topology.is_consistent()
                {
                    topology.outgoing_edges
                } else {
                    return Err(Box::new(TopologicalError::WrongTopologicalAssumptions));
                };
            if topological_order[0] == nodes[0] {
                assert_eq!(lengths_map.insert(nodes[0].id, (Some(0),Some(0))), Some((None, None))); // all nodes have been initiated in lengths_map previously with value (None, None)
            };

            let mut reverse_topological_order: Vec<&Node<T>> = topological_order.iter().rev().collect(); // The topological order is reversed to iterate over the last element in the memory layout of the vector.
            let nodes_length = loop {
                if let Some(last_node_from_reverse_topological_order) = reverse_topological_order.pop() {
                    let node_distance = &lengths_map.get(&last_node_from_reverse_topological_order.id).expect("Wrong topological assumptions.").clone(); // all nodes have been inserted to lengths_map previously at this stage.
                    match outgoing_edges.remove(&last_node_from_reverse_topological_order.id) {
                        Some(edges) => {
                            for node_id in edges {
                                if let Some(outgoing_node_path_lengths) = lengths_map.get_mut(&node_id) {
                                    if let Some(shortest_distance) = node_distance.0 {
                                        let weight = shortest_distance + 1;
                                        match outgoing_node_path_lengths.0 {
                                            Some(outgoing_node_path_shortest_distance) => {
                                                if outgoing_node_path_shortest_distance > weight {
                                                    outgoing_node_path_lengths.0 = Some(weight);
                                                };
                                            },
                                            None => {
                                                outgoing_node_path_lengths.0 = Some(weight);
                                            }
                                        };
                                    };
                                    if let Some(longest_distance) = node_distance.1 {
                                        let weight = longest_distance + 1;
                                        match outgoing_node_path_lengths.1 {
                                            Some(outgoing_node_path_longest_distance) => {
                                                if outgoing_node_path_longest_distance < weight {
                                                    outgoing_node_path_lengths.1 = Some(weight);
                                                };
                                            },
                                            None => {
                                                outgoing_node_path_lengths.1 = Some(weight);
                                            }
                                        }
                                    };
                                } else {
                                    return Err(Box::new(TopologicalError::WrongTopologicalAssumptions));
                                }
                            }
                        },
                        None => {}
                    }
                } else {
                    break lengths_map;
                }

            };
            Ok(Some(nodes_length))
        } else {
            Ok(None)
        }
    }
    /// Breath-First Search returns threads upto all nodes starting from the origin 
    /// marked as the first node id from which the iteration of this algorithm started from, i.e. first call arguments.
    fn bfs_visit<'a>(topology: &Self, id: T, backtrace: &mut Vec<T>, paths_collection: &mut Vec<Vec<T>>) {
        match topology.get_outgoing_edges_by_id(id) {
            Some(edges) => {
                if backtrace.len() == 0 {
                    backtrace.push(id);
                };
                for node_id in edges {
                    let mut new_thread = backtrace.clone();
                    new_thread.push(*node_id);
                    paths_collection.push(new_thread.clone());
                    Self::bfs_visit(topology, *node_id, &mut new_thread, paths_collection);
                };
            },
            None => { // If there is no listed edges for this node id, then it is supposed to be a last node in the thread.

            }
        };
    }
    pub fn bfs_all_paths(topology: &Self, id: T) -> Option<Vec<Vec<T>>> {
        let mut collection: Vec<Vec<T>> = Vec::new();
        let empty_vector = &mut Vec::new();
        Topology::bfs_visit(&topology, id, empty_vector, &mut collection);
        Some(collection)
    }
}

#[test]
fn insert_nodes_in_topology_analysis() {
    let node_a = Node::new(0,None,None);
    let node_b = Node::new(1,Some(0),None);
    let node_c = Node::new(2,None,Some(0));
    let node_d = Node::new(3,Some(0), Some(1));
    let node_e = Node::new(4,Some(2), Some(1));
    let node_f = Node::new(5,Some(3), Some(4));
    let mut topology = Topology::new();
    let node_list = [node_a, node_b, node_c, node_d, node_e, node_f];
    for node in node_list.into_iter() {
        assert_eq!(topology.insert(node), None); // All nodes inserted are new nodes to the topology.
    };
    assert_eq!(topology.all_nodes.get(&Node::new(23, Some(42), Some(50)).into()), None);  // tries to take an inexistent node
    for node in node_list.into_iter() {
        assert_eq!(topology.all_nodes.get(&node.into()).expect("Wrong value assumption."), &CollidingNode(node));
    };
    for node in node_list.into_iter() {
        assert_eq!(topology.get_unique_node_by_id(node.id).expect("Wrong value assumption."), node);
    };
    assert_eq!(topology.collitions.len(), 0);
    assert_eq!(topology.repeated_nodes.len(), 0);
    assert_eq!(topology.edge_sum(), 8);
    assert_eq!(topology.unique_nodes.len(), 6);
    assert_eq!(topology.get_unique_node_by_id(4), Some(node_e)); // Check existence for node_e
    for (from, to_list) in topology.outgoing_edges.iter() {
        for to in to_list.iter() {
            let compare_node = topology.get_unique_node_by_id(*to).expect("Wrong value assumptions");
            assert!(compare_node.left == Some(*from) || compare_node.right == Some(*from)); // Checks that all edges corresponds to a node in the original node's list.
        }
    };
    assert!(topology.is_consistent()); // Upto this stage, the topology is consistent.
    let colliding_node = Node::new(4, None, None);
    assert_eq!(topology.insert(colliding_node), Some(node_e.into())); // Tries to insert a node with the same id of an already indexed node in the topology analysis.
    assert_eq!(topology.collitions.len(), 1); // There's one collition.
    assert_eq!(topology.get_unique_node_by_id(4), Some(node_e)); // Checks that the Node in the collection of unique nodes is still the first one added by the same id.
    assert_eq!(topology.collitions.get(&node_e.into()), None); // The first added node_e will not be present in the collitions set.
    assert_eq!(topology.collitions.get(&colliding_node.into()), Some(&colliding_node.into()));  // The colliding node will be present in the collitions set.
    assert_eq!(topology.repeated_nodes.len(), 0);  // There is no repeated nodes yet.
    assert_eq!(topology.insert(colliding_node), Some(node_e.into()));
    assert_eq!(topology.repeated_nodes.len(), 1); // There is one repeated node i.e. the colliding node.
    assert_eq!(topology.insert(node_b), Some(node_b.into())); // Tries to insert node_b again.
    assert_eq!(topology.collitions.len(), 1); // The number of collitions is still 1.
    assert_eq!(topology.repeated_nodes.len(), 2); // The number of repeated nodes is now 2.
    assert_eq!(topology.edge_sum(), 8); // The number of edges has not changed.
    assert_eq!(topology.unique_nodes.len(), 6);  // The number of unique nodes has not changed.
    for node in node_list.into_iter() {
        assert_eq!(topology.get_unique_node_by_id(node.id).expect("Wrong value assumption."), node); // The map of unique_nodes is still the original list.
    };
    for (from, to_list) in topology.outgoing_edges.iter() {
        for to in to_list.iter() {
            let compare_node = topology.get_unique_node_by_id(*to).expect("Wrong value assumptions");
            assert!(compare_node.left == Some(*from) || compare_node.right == Some(*from)); // All edges still corresponds to a node in the original node's list.
        }
    };
    assert!(!topology.is_consistent()); // At this stage, the topology is *NOT* consistent.
}

fn insert_node_with_inexistent_references_is_inconsistent() {
    let mut topology: Topology<u32> = Topology::new();
    let node = Node::new(1, None, Some(2));
    topology.insert(node);
    assert_eq!(topology.collitions.len(), 0);
    assert_eq!(topology.repeated_nodes.len(), 0);
    assert!(!topology.is_consistent());
}

#[test]
fn topological_order() {
    let node_a = Node::new(0,None,None);
    let node_b = Node::new(1,Some(0),None);
    let node_c = Node::new(2,None,Some(0));
    let node_d = Node::new(3,Some(0), Some(1));
    let node_e = Node::new(4,Some(2), Some(1));
    let node_f = Node::new(5,Some(3), Some(4));
    let node_list = [node_a, node_b, node_c, node_d, node_e, node_f];
    let ordering = Topology::sort(&node_list).expect("Wrong value assumptions.").expect("Wrong value assumptions.");
    assert!(ordering.len() > 0);
    let mut dag = Dag::new();
    for node in ordering {
        dag.insert(node);
    };
    assert!(dag.is_safe());
}

#[test]
fn another_topological_order() {
    let node_a = Node::new(35,None,None);
    let node_b = Node::new(42,Some(35),None);
    let node_c = Node::new(32,None,Some(35));
    let node_d = Node::new(51,Some(42), None);
    let node_e = Node::new(101,Some(32), Some(51));
    let node_f = Node::new(52,Some(51), Some(32));
    let node_g = Node::new(50,Some(42), None);
    let node_h = Node::new(1,Some(50), Some(52));
    let node_i = Node::new(0,Some(50), Some(1));
    let node_list = [node_a, node_b, node_c, node_d, node_e, node_f, node_g, node_h, node_i];
    let ordering = Topology::sort(&node_list).expect("Wrong value assumptions.").expect("Wrong value assumptions.");
    // println!("Ordering : {:?}",ordering);
    assert!(ordering.len() == node_list.len());
    assert!(ordering.len() > 0);
    let mut dag = Dag::new();
    for node in ordering {
        dag.insert(node);
    };
    assert!(dag.is_safe());
}

#[test]
fn non_dag_topological_order() {
    let node_a = Node::new(35,None,None);
    let node_b = Node::new(42,Some(35),None);
    let node_c = Node::new(32,None,Some(35));
    let node_d = Node::new(51,Some(42), Some(0));
    let node_e = Node::new(101,Some(32), Some(51));
    let node_f = Node::new(52,Some(51), Some(32));
    let node_g = Node::new(50,Some(42), None);
    let node_h = Node::new(1,Some(50), Some(52));
    let node_i = Node::new(0,Some(50), Some(1));
    let node_list = [node_a, node_b, node_c, node_d, node_e, node_f, node_g, node_h, node_i];
    let ordering = Topology::sort(&node_list).expect("Wrong value assumptions.");
    assert_eq!(ordering, None);
}

#[test]
fn shortest_and_longest_paths() {
    let node_prime = Node::new(1, None, None);
    let node_a = Node::new(2, Some(1), Some(1));
    let node_b = Node::new(3, Some(1), Some(2));
    let node_c = Node::new(4, Some(2), Some(2));
    let node_d = Node::new(5, Some(3), Some(6));
    let node_e = Node::new(6, Some(3), Some(3));
    let Ok(Some(sorted)) = Topology::sort(&[node_prime, node_a, node_b, node_c, node_d, node_e]) else { panic!("Wrong topological assumptions for this test data.") };
    let Ok(Some(shortest_and_longest)) = Topology::shortest_and_longest_paths(&sorted) else { panic!("Wrong topological assumptions for this test data.") };
    let printable: Vec<(&u32, &(Option<usize>, Option<usize>))> = shortest_and_longest.iter().collect();
    // println!("shortest and longest : {:?}", printable);
}

#[test]
fn another_shortest_and_longest_paths() {
    let node_a = Node::new(35,None,None);
    let node_b = Node::new(42,Some(35),None);
    let node_c = Node::new(32,None,Some(35));
    let node_d = Node::new(51,Some(42), None);
    let node_e = Node::new(101,Some(32), Some(51));
    let node_f = Node::new(52,Some(51), Some(101));
    let node_g = Node::new(50,Some(42), None);
    let node_h = Node::new(1,Some(72), Some(52));
    let node_i = Node::new(0,Some(50), Some(1));
    let node_j = Node::new(333,Some(50), Some(101));
    let node_k = Node::new(72, Some(35), Some(50));
    let node_list = [node_a, node_b, node_c, node_d, node_e, node_f, node_g, node_h, node_i, node_j, node_k];
    let Ok(Some(sorted)) = Topology::sort(&node_list) else { panic!("Wrong topological assumptions for this test data.") };
    let Ok(Some(shortest_and_longest)) = Topology::shortest_and_longest_paths(&sorted) else { panic!("Wrong topological assumptions for this test data.") };
    let printable: Vec<(&u32, &(Option<usize>, Option<usize>))> = shortest_and_longest.iter().collect();
    // println!("another shortest and longest : {:?}", printable);
}

#[test]
fn bfs_threads() {
    let node_prime = Node::new(1, None, None);
    let node_a = Node::new(2, Some(1), Some(1));
    let node_b = Node::new(3, Some(1), Some(2));
    let node_c = Node::new(4, Some(2), Some(2));
    let node_d = Node::new(5, Some(3), Some(6));
    let node_e = Node::new(6, Some(3), Some(3));
    let node_list = &[node_prime, node_a, node_b, node_c, node_d, node_e];
    let Some(topology) = Topology::from_slice(node_list) else { panic!("Wrong topological assumptions for this test data.") };

    let Some(bfs_all_paths) = Topology::bfs_all_paths(&topology, node_prime.id) else { panic!("Wrong topological assumptions for this test data.") };;
    let all_paths_size_sum: usize = bfs_all_paths.iter().map(|path| { path.len() }).sum();
    let average_node_size = all_paths_size_sum as f32/bfs_all_paths.len() as f32;
    assert_eq!(bfs_all_paths.len(), 24);
    assert_eq!(all_paths_size_sum, 85);
    assert_eq!((average_node_size*10_000.0).round(), 35417.0); // compares truncated significant
}

#[test]
fn bfs_threads_no_double_edges() {
    let node_prime = Node::new(1, None, None);
    let node_a = Node::new(2, Some(1), None);
    let node_b = Node::new(3, Some(1), Some(2));
    let node_c = Node::new(4, Some(2), None);
    let node_d = Node::new(5, Some(3), Some(6));
    let node_e = Node::new(6, Some(3), None);
    let node_list = &[node_prime, node_a, node_b, node_c, node_d, node_e];
    let Some(topology) = Topology::from_slice(node_list) else { panic!("Wrong topological assumptions for this test data.") };

    let Some(bfs_all_paths) = Topology::bfs_all_paths(&topology, node_prime.id) else { panic!("Wrong topological assumptions for this test data.") };;
    let all_paths_size_sum: usize = bfs_all_paths.iter().map(|path| { path.len() }).sum();
    let average_node_size = all_paths_size_sum as f32/bfs_all_paths.len() as f32;
    assert_eq!(bfs_all_paths.len(), 10);
    assert_eq!(all_paths_size_sum, 33);
    assert_eq!((average_node_size*10_000.0).round(), 33000.0); // compares truncated significant
}