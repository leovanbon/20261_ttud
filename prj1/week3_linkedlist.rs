use std::collections::HashMap;
use std::io::{self, Read, Write};

struct Node {
    key: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Default)]
struct List {
    nodes: Vec<Node>,
    positions: HashMap<i32, usize>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl List {
    fn insert(&mut self, key: i32, prev: Option<usize>, next: Option<usize>) {
        if self.positions.contains_key(&key) {
            return;
        }
        let index = self.nodes.len();
        self.nodes.push(Node { key, prev, next });
        self.positions.insert(key, index);
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut list = List::default();

    for _ in 0..n {
        list.insert(it.next().unwrap().parse().unwrap(), list.tail, None);
    }

    while let Some(cmd) = it.next() {
        match cmd {
            "#" => break,
            "addlast" => {
                let k = it.next().unwrap().parse().unwrap();
                list.insert(k, list.tail, None);
            }
            "addfirst" => {
                let k = it.next().unwrap().parse().unwrap();
                list.insert(k, None, list.head);
            }
            "addafter" | "addbefore" => {
                let u = it.next().unwrap().parse().unwrap();
                let v: i32 = it.next().unwrap().parse().unwrap();
                if let Some(&index) = list.positions.get(&v) {
                    if cmd == "addafter" {
                        list.insert(u, Some(index), list.nodes[index].next);
                    } else {
                        list.insert(u, list.nodes[index].prev, Some(index));
                    }
                }
            }
            "remove" => {
                let k: i32 = it.next().unwrap().parse().unwrap();
                if let Some(index) = list.positions.remove(&k) {
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
        write!(out, "{}", list.nodes[index].key).unwrap();
        current = list.nodes[index].next;
        if current.is_some() {
            write!(out, " ").unwrap();
        }
    }

    writeln!(out).unwrap();
}
