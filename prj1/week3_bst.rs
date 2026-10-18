use std::io::{self, Read, Write};

struct Node {
    key: i32,
    left: Option<usize>,
    right: Option<usize>,
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
    while let Some(cmd) = it.next() {
        if cmd == "#" {
            break;
        }
        let key: i32 = next!();
        if nodes.is_empty() {
            nodes.push(Node { key, left: None, right: None });
            continue;
        }
        let mut index = 0;
        loop {
            if nodes[index].key == key {
                break;
            }
            let left = key < nodes[index].key;
            let next = if left { nodes[index].left } else { nodes[index].right };
            if let Some(next) = next {
                index = next;
            } else {
                let next = nodes.len();
                nodes.push(Node { key, left: None, right: None });
                if left {
                    nodes[index].left = Some(next);
                } else {
                    nodes[index].right = Some(next);
                }
                break;
            }
        }
    }
    let mut stack = Vec::new();
    if !nodes.is_empty() {
        stack.push(0);
    }
    while let Some(index) = stack.pop() {
        write!(out, "{} ", nodes[index].key).unwrap();
        if let Some(right) = nodes[index].right {
            stack.push(right);
        }
        if let Some(left) = nodes[index].left {
            stack.push(left);
        }
    }
}
