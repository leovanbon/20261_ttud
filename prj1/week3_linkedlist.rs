use std::collections::HashMap;
use std::io::{self, Read, Write};

struct Node {
    key: i32,
    // `Option<T>` is either Some(value) or None; absence is represented explicitly, without a null value.
    prev: Option<usize>,
    next: Option<usize>,
}
// `derive(Default)` generates default initialization: empty collections and None for Option fields.
#[derive(Default)]
struct List {
    nodes: Vec<Node>,
    positions: HashMap<i32, usize>,
    head: Option<usize>,
    tail: Option<usize>,
}
// `impl Type` defines associated functions and methods; &mut self is a mutable borrow of the receiver.
impl List {
    // `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
    fn insert(&mut self, key: i32, prev: Option<usize>, next: Option<usize>) {
        if self.positions.contains_key(&key) {
            return;
        }
        let index = self.nodes.len();
        self.nodes.push(Node { key, prev, next });
        self.positions.insert(key, index);
        // `if let` runs this branch only when the value matches Some(...), binding its contents.
        if let Some(p) = prev {
            self.nodes[p].next = Some(index);
        } else {
            self.head = Some(index);
        }
        if let Some(p) = next {
            self.nodes[p].prev = Some(index);
        } else {
            self.tail = Some(index);
        }
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
    // `usize` is the unsigned type used for collection lengths and indices.
    let n: usize = next!();
    let mut list = List::default();
    // `a..b` excludes b; `a..=b` includes b.
    for _ in 0..n {
        list.insert(next!(), list.tail, None);
    }
    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
    while let Some(cmd) = it.next() {
        // `match` selects a pattern arm (`pattern => expression`); `_` matches any remaining value.
        match cmd {
            "#" => break,
            "addlast" => {
                let k = next!();
                list.insert(k, list.tail, None);
            }
            "addfirst" => {
                let k = next!();
                list.insert(k, None, list.head);
            }
            // `|` between patterns means either pattern matches, unlike the logical OR operator ||.
            "addafter" | "addbefore" => {
                let u = next!();
                let v: i32 = next!();
                // `Some(&x)` matches an Option containing a reference and copies its Copy value into x.
                if let Some(&index) = list.positions.get(&v) {
                    if cmd == "addafter" {
                        list.insert(u, Some(index), list.nodes[index].next);
                    } else {
                        list.insert(u, list.nodes[index].prev, Some(index));
                    }
                }
            }
            "remove" => {
                let k: i32 = next!();
                if let Some(index) = list.positions.remove(&k) {
                    // Tuple destructuring binds each component separately; mut applies only to the binding it precedes.
                    let (prev, next) = (list.nodes[index].prev, list.nodes[index].next);
                    if let Some(p) = prev {
                        list.nodes[p].next = next;
                    } else {
                        list.head = next;
                    }
                    if let Some(p) = next {
                        list.nodes[p].prev = prev;
                    } else {
                        list.tail = prev;
                    }
                }
            }
            "reverse" => {
                let mut current = list.head;
                while let Some(index) = current {
                    let node = &mut list.nodes[index];
                    std::mem::swap(&mut node.prev, &mut node.next);
                    current = node.prev;
                }
                std::mem::swap(&mut list.head, &mut list.tail);
            }
            _ => {}
        }
    }
    let mut current = list.head;
    while let Some(index) = current {
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        write!(out, "{}", list.nodes[index].key).unwrap();
        current = list.nodes[index].next;
        if current.is_some() {
            write!(out, " ").unwrap();
        }
    }
    writeln!(out).unwrap();
}
