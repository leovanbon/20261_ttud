use std::collections::HashMap;
use std::io::{self, Read, Write};

struct Node {
    id: i32,
    children: Vec<usize>,
}
// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
// `impl Write` accepts any output type implementing the Write trait; &mut borrows it for writing.
fn traverse(index: usize, nodes: &[Node], order: &str, out: &mut impl Write) {
    let node = &nodes[index];
    if order == "PreOrder" {
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        write!(out, "{} ", node.id).unwrap();
    }
    // `Some(&x)` matches an Option containing a reference and copies its Copy value into x.
    if let Some(&first) = node.children.first() {
        traverse(first, nodes, order, out);
    }
    if order == "InOrder" {
        write!(out, "{} ", node.id).unwrap();
    }
    // `&child` in this loop pattern copies each referenced integer, leaving the collection borrowed.
    for &child in node.children.iter().skip(1) {
        traverse(child, nodes, order, out);
    }
    if order == "PostOrder" {
        write!(out, "{} ", node.id).unwrap();
    }
}

fn main() {
    // `let` bindings are immutable by default; `mut` allows changes. String owns its text.
    let mut input = String::new();
    // `&mut input` lends exclusive mutable access; unwrap() extracts Ok or panics on an I/O error.
    io::stdin().read_to_string(&mut input).unwrap();
    // The iterator yields borrowed &str tokens; input must stay alive while they are used.
    let mut it = input.split_whitespace();
    // `macro_rules!` defines a macro (called with !). `$t:ty` matches a type; parse::<T>() selects it explicitly.
    // unwrap() extracts Some/Ok or panics; next!() infers the type, while next!(usize) specifies it.
    macro_rules! next {
        ($t:ty) => {
            it.next().unwrap().parse::<$t>().unwrap()
        };
        () => {
            it.next().unwrap().parse().unwrap()
        };
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut nodes: Vec<Node> = Vec::new();
    let mut positions = HashMap::new();
    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
    while let Some(cmd) = it.next() {
        // `match` selects a pattern arm (`pattern => expression`); `_` matches any remaining value.
        match cmd {
            "*" => break,
            "MakeRoot" => {
                let id: i32 = next!();
                if nodes.is_empty() {
                    nodes.push(Node { id, children: Vec::new() });
                    positions.insert(id, 0);
                }
            }
            "Insert" => {
                let id: i32 = next!();
                let parent: i32 = next!();
                if positions.contains_key(&id) {
                    continue;
                }
                if let Some(&p) = positions.get(&parent) {
                    let index = nodes.len();
                    nodes.push(Node { id, children: Vec::new() });
                    nodes[p].children.push(index);
                    positions.insert(id, index);
                }
            }
            // `|` between patterns means either pattern matches, unlike the logical OR operator ||.
            "PreOrder" | "InOrder" | "PostOrder" => {
                if !nodes.is_empty() {
                    traverse(0, &nodes, cmd, &mut out);
                }
                writeln!(out).unwrap();
            }
            _ => {}
        }
    }
}
