use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let graph = read_input();
    {
        let mut visited = HashSet::new();
        let mut path_counter = 0;
        traverse_small_cave_once(&graph, Node::Start, &mut visited, &mut path_counter);
        println!("traverse small cave once path count: {}", path_counter);
    }
    {
        let mut visited = HashSet::new();
        let mut path_counter = 0;
        traverse_one_small_cave_twice(&graph, Node::Start, &mut visited, &mut path_counter, false);
        println!(
            "traverse once small cave twice path count: {}",
            path_counter
        );
    }
}

fn read_input() -> HashMap<Node, HashSet<Node>> {
    let mut result = HashMap::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for line in string.lines() {
            let mut iter = line.split('-');
            if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
                let a = to_node(a);
                let b = to_node(b);
                add_edge(&mut result, a.clone(), b.clone());
                add_edge(&mut result, b.clone(), a.clone());
            }
        }
    }
    return result;
}

fn add_edge(graph: &mut HashMap<Node, HashSet<Node>>, a: Node, b: Node) {
    if let Some(set) = graph.get_mut(&a) {
        set.insert(b);
    } else {
        let mut set = HashSet::new();
        set.insert(b);
        graph.insert(a, set);
    }
}

fn to_node(string: &str) -> Node {
    match string {
        "start" => Node::Start,
        "end" => Node::End,
        _ => {
            if string.chars().next().unwrap().is_lowercase() {
                Node::Small(String::from(string))
            } else {
                Node::Big(String::from(string))
            }
        }
    }
}

fn traverse_small_cave_once(
    graph: &HashMap<Node, HashSet<Node>>,
    node: Node,
    visited: &mut HashSet<Node>,
    path_counter: &mut usize,
) {
    if node == Node::End {
        *path_counter += 1;
    } else {
        if let Some(other_set) = graph.get(&node) {
            for other in other_set {
                match other {
                    Node::Small(_) => {
                        if !visited.contains(other) {
                            visited.insert(other.clone());
                            traverse_small_cave_once(graph, other.clone(), visited, path_counter);
                            visited.remove(other);
                        }
                    }
                    Node::Start => {}
                    _ => traverse_small_cave_once(graph, other.clone(), visited, path_counter),
                }
            }
        }
    }
}

fn traverse_one_small_cave_twice(
    graph: &HashMap<Node, HashSet<Node>>,
    node: Node,
    visited: &mut HashSet<Node>,
    path_counter: &mut usize,
    twice: bool,
) {
    if node == Node::End {
        *path_counter += 1;
    } else {
        if let Some(other_set) = graph.get(&node) {
            for other in other_set {
                match other {
                    Node::Small(_) => {
                        if !visited.contains(other) {
                            visited.insert(other.clone());
                            traverse_one_small_cave_twice(
                                graph,
                                other.clone(),
                                visited,
                                path_counter,
                                twice,
                            );
                            visited.remove(other);
                        } else if !twice {
                            traverse_one_small_cave_twice(
                                graph,
                                other.clone(),
                                visited,
                                path_counter,
                                true,
                            );
                        }
                    }
                    Node::Start => {}
                    _ => traverse_one_small_cave_twice(
                        graph,
                        other.clone(),
                        visited,
                        path_counter,
                        twice,
                    ),
                }
            }
        }
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum Node {
    Start,
    End,
    Small(String),
    Big(String),
}
