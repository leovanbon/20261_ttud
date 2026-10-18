use std::collections::HashMap;
use std::io::{self, Read, Write};

struct Node {
    id: i32,
    children: Vec<usize>,
}
fn traverse(index: usize, nodes: &[Node], order: &str, out: &mut impl Write) {
    let node = &nodes[index];
    if order == "PreOrder" {
        write!(out, "{} ", node.id).unwrap();
    }
    if let Some(&first) = node.children.first() {
        traverse(first, nodes, order, out);
    }
    if order == "InOrder" {
        write!(out, "{} ", node.id).unwrap();
    }
    for &child in node.children.iter().skip(1) {
        traverse(child, nodes, order, out);
    }
    if order == "PostOrder" {
        write!(out, "{} ", node.id).unwrap();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
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
    while let Some(cmd) = it.next() {
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
