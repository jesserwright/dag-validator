use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{self, BufRead};
use std::process::{ExitCode, Termination};

fn main() -> Exit {
    let stdin = io::stdin().lock();

    match validate_dag(stdin) {
        (exit, None) => exit,
        (exit, Some(cycle)) => {
            for c in cycle.chars() {
                println!("{}", c);
            }
            exit
        }
    }
}

fn validate_dag<B: BufRead>(buf: B) -> (Exit, Option<String>) {
    // The graph represented as adjacency list
    let mut g: HashMap<Node, Vec<Node>> = HashMap::new();

    // Process input and build adjacency list
    for input_line in buf.lines() {
        match input_line {
            Ok(line) => {
                let mut chars = line.chars();
                match (&chars.next(), &chars.next()) {
                    // A third character on the line will be ignored
                    (Some(source), Some(sink)) => {
                        let (source, sink) = (Node(*source), Node(*sink));

                        if source == sink {
                            return (Exit::Cycle, Some(format!("{}", source.0)));
                        } else {
                            match g.get_mut(&source) {
                                Some(out_degrees) => {
                                    out_degrees.push(sink.to_owned());
                                }
                                None => {
                                    g.insert(source.to_owned(), vec![sink.to_owned()]);
                                }
                            }
                        }
                    }
                    // Is there a case where just one character is given on a line?
                    (Some(node), None) => {
                        g.insert(Node(*node), vec![]);
                    }
                    _ => {
                        // Must be at least two characters; beyond 2 scalars are ignored
                        return (Exit::InvalidInput, None);
                    }
                }
            }
            Err(_input_error) => {
                return (Exit::InvalidInput, None); // Must be valid UTF-8
            }
        }
    }

    // Check for cycles
    for root in g.keys() {
        // State of path taken
        let mut path: Vec<Node> = Vec::new();

        // State of visitation per node indicated by index; indicies are related to the adjacency list values
        let mut sinks: Vec<usize> = Vec::new();

        // Initialize path & sinks
        path.push(root.to_owned());
        sinks.push(0);

        // Iterate as long as the path and sinks have items present
        while let (Some(node), Some(sink_index)) = (path.last_mut(), sinks.last_mut()) {
            // If there is a valid next item given the sink index...
            if let Some(descendants) = g.get(node) {
                // Using `.get(index)` returns an option instead of panicing like `[index]`;
                // if it is out of bounds then that means the node has no more sinks
                if let Some(next) = &descendants.get(*sink_index) {
                    // Check path for cycle. Not ideal time complexity to be calling `.find()` on a `Vec`
                    if let Some((cycle_start, _)) =
                        path.iter().enumerate().find(|(_idx, n)| n == next)
                    {
                        // Return the cycle never including the same node twice
                        let cycle = path[cycle_start..].iter().map(|n| n.0).collect::<String>();
                        return (Exit::Cycle, Some(cycle));
                    }

                    // Mark 'next' as visited; this may be out of bounds but it will be checked next iteration
                    *sink_index += 1;

                    // Add to path and sinks if there are sinks for 'next' according to the adjacency list
                    if g.get(next).is_some() {
                        path.push(next.to_owned().to_owned());
                        sinks.push(0);
                    }
                    continue;
                }
            }
            // Done with this node
            path.pop();
            sinks.pop();
        }
    }
    (Exit::NoCycle, None)
}

#[derive(Hash, Clone, Debug, Eq)]
struct Node(char);

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug)]
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

#[test]
fn no_cycle() {
    let input = "\
AB
BD
DF
";
    let buf = input.as_bytes();
    let result = validate_dag(buf);
    assert_eq!(result, (Exit::NoCycle, None));
}

#[test]
fn cycle_simple() {
    let input = "\
AB
BC
CA
";
    let buf = input.as_bytes();
    let result = validate_dag(buf);
    assert_eq!(result.0, Exit::Cycle);
    assert!(result.1.is_some());
    let cycle = result.1.unwrap();
    assert!(cycle == "ABC" || cycle == "BCA" || cycle == "CAB");
}

#[test]
fn cycle_small() {
    let input = "\
BA
AB
";
    let buf = input.as_bytes();
    let result = validate_dag(buf);
    assert_eq!(result.0, Exit::Cycle);
    assert!(result.1.is_some());
    let cycle = result.1.unwrap();
    assert!(cycle == "AB" || cycle == "BA");
}

#[test]
fn cycle_single_node() {
    let input = "\
AA
";
    let buf = input.as_bytes();
    let result = validate_dag(buf);
    assert_eq!(result, (Exit::Cycle, Some("A".to_string())));
}

#[test]
fn cycle_large() {
    let input = "\
AB
BD
CM
DF
EF
CE
CD
FG
FH
FI
FJ
FK
HL
IL
JL
EL
LD
JK
ZF
ZJ
BI
LK
";
    let buf = input.as_bytes();
    let result = validate_dag(buf);
    assert_eq!(result.0, Exit::Cycle);
    assert!(result.1.is_some());
    let cycle = result.1.unwrap();
    assert!(cycle == "HLDF" || cycle == "LDFH" || cycle == "DFHL" || cycle == "FHLD");
}

#[test]
fn invalid_input_short() {
    let input = "\
A
b
bA
";
    let buf = input.as_bytes();
    let result = validate_dag(buf);
    assert_eq!(result, (Exit::NoCycle, None));
}

#[test]
fn input_long() {
    let input = "\
AB
BA
";
    let buf = input.as_bytes();
    let result = validate_dag(buf);
    assert_eq!(result.0, Exit::Cycle);
    assert!(result.1.is_some());
    let cycle = result.1.unwrap();
    assert!(cycle == "AB" || cycle == "BA");
}

#[test]
fn unicode_scalars_valid_cycle() {
    let input = "\
y̆
\u{306}y
";
    let buf = input.as_bytes();
    let result = validate_dag(buf);

    assert_eq!(result.0, Exit::Cycle);
    assert!(result.1.is_some());
    let cycle = result.1.unwrap();
    assert!(cycle == "y\u{306}" || cycle == "\u{306}y");
}
