use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io;
use std::process::{ExitCode, Termination};

// Two things I would improve:
// Only iterate roots
// Use a BTreeMap to store tree state for constant lookup times and ordering

fn main() -> Exit {
    let mut g: HashMap<Node, Vec<Node>> = HashMap::new();
    let mut sinks: HashSet<Node> = HashSet::new();
    let mut sources: HashSet<Node> = HashSet::new();

    for input_line in io::stdin().lines() {
        match input_line {
            Ok(line) => {
                let mut chars = line.chars();
                if let (Some(source), Some(sink)) = (&chars.next(), &chars.next()) {
                    let (source, sink) = (Node(*source), Node(*sink));

                    sinks.insert(sink.to_owned());
                    sources.insert(source.to_owned());

                    if source == sink {
                        println!("{}", source.0);
                        return Exit::Cycle;
                    } else {
                        if let Some(out_degrees) = g.get_mut(&source) {
                            out_degrees.push(sink.to_owned());
                        } else {
                            g.insert(source.to_owned(), vec![sink.to_owned()]);
                        }
                    }
                } else {
                    // Must be at least two characters; beyond 2 scalars are ignored
                    return Exit::InvalidInput;
                }
            }
            Err(_input_error) => {
                return Exit::InvalidInput; // Must be valid UTF-8
            }
        }
    }

    // All sources that are not sinks are roots
    let roots: Vec<&Node> = sources.difference(&sinks).collect();

    // let mut m: BTreeMap<Node, usize> = BTreeMap::new();

    for root in roots {
        let mut path: Vec<Node> = Vec::new();
        let mut indicies: Vec<usize> = Vec::new();

        path.push(root.to_owned());
        indicies.push(0);

        while let (Some(node), Some(index)) = (path.last_mut(), indicies.last_mut()) {
            if let Some(descendants) = g.get(node) {
                if let Some(next) = &descendants.get(*index) {
                    // 😬
                    if let Some((idx, _)) = path.iter().enumerate().find(|(_idx, n)| n == next) {
                        for n in &path[idx..] {
                            println!("{}", n.0);
                        }
                        return Exit::Cycle;
                    }

                    *index += 1;

                    if g.get(next).is_some() {

                        path.push(next.to_owned().to_owned());
                        indicies.push(0);

                    }
                    continue;
                }
            }

            path.pop();
        }
    }

    Exit::NoCycle
}

#[derive(Hash, Clone, Debug, Eq)]
struct Node(char);

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[repr(u8)]
enum Exit {
    NoCycle = 0,
    Cycle = 1,
    InvalidInput = 2,
}

impl Termination for Exit {
    fn report(self) -> std::process::ExitCode {
        ExitCode::from(self as u8)
    }
}
