use std::io::{self, Read, Write};

struct Node {
    key: i32,
    // `Option<T>` is either Some(value) or None; absence is represented explicitly, without a null value.
    left: Option<usize>,
    right: Option<usize>,
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
    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
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
            // `if let` runs this branch only when the value matches Some(...), binding its contents.
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
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        write!(out, "{} ", nodes[index].key).unwrap();
        if let Some(right) = nodes[index].right {
            stack.push(right);
        }
        if let Some(left) = nodes[index].left {
            stack.push(left);
        }
    }
}
