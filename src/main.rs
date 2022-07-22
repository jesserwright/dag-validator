use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::process::{ExitCode, Termination};

fn main() -> Exit {
    let mut edges: Vec<Edge> = Vec::new();

    for input_line in io::stdin().lines() {
        match input_line {
            Ok(line) => {
                let mut chars = line.chars();
                if let (Some(source), Some(sink)) = (&chars.next(), &chars.next()) {
                    if source == sink {
                        println!("{}", source);
                        return Exit::Cycle;
                    } else {
                        edges.push(Edge {
                            source: Node(*source),
                            sink: Node(*sink),
                        });
                    }
                } else {
                    // Input must be two characters
                    return Exit::InvalidInput;
                }
            }
            Err(_input_error) => {
                // Input must be valid UTF-8
                return Exit::InvalidInput;
            }
        }
    }

    // g for graph, as an adjacency list
    let mut g: HashMap<Node, Vec<Node>> = HashMap::new();

    // build the list of roots here; they are never listed in as a 'sink'

    for edge in &edges {
        if let Some(out_degrees) = g.get_mut(&edge.source) {
            out_degrees.push(edge.sink.to_owned());
        } else {
            g.insert(edge.source.to_owned(), vec![edge.sink.to_owned()]);
        }
    }

    for node in g.keys() {
        // separating 'state' should solve the extra ownership problem.
        let mut set: Vec<Node> = Vec::new();
        set.push(node.to_owned());
        let mut path: Vec<State> = Vec::new();
        path.push(State {
            node: node.to_owned(),
            index: 0,
        });
        // if path.iter().any(|s| s.node == node.to_owned()) { return Exit::Cycle; }

        loop {
            println!("{:?}", &path);
            if let Some(State { node, index }) = path.last_mut() {
                if let Some(descendants) = g.get(node) {
                    // find roots and iterate them instead.
                    if let Some(next) = &descendants.get(*index) {
                        // how to get a ref to path to check??? snapshot?
                        if set.contains(next) {
                            // find the slice of the cycle by finding the index of 'next'
                            // and slicing starting from there
                            println!("{:?}", &set);
                            return Exit::Cycle;
                        }
                        *index += 1;
                        if g.get(next).is_some() {
                            set.push(next.to_owned().to_owned());
                            path.push(State {
                                node: next.to_owned().to_owned(),
                                index: 0,
                            });
                        }
                        continue; // go to next loop
                    }
                    // will pop if the index is invalid; the next index will be checked
                }
                set.pop();
                path.pop();
            } else {
                break;
            }
        }
    }

    Exit::NoCycle
}

#[derive(Debug)]
struct State {
    node: Node,
    index: usize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.node.0, self.index)
    }
}

#[repr(u8)]
enum Exit {
    NoCycle = 0,
    Cycle = 1,
    InvalidInput = 2,
}

#[derive(Debug)]
struct Edge {
    source: Node,
    sink: Node,
}

#[derive(Eq, Hash, Clone, Debug)]
struct Node(char);

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Termination for Exit {
    fn report(self) -> std::process::ExitCode {
        ExitCode::from(self as u8)
    }
}
