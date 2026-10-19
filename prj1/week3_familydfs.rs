use std::collections::HashMap;
use std::io::{self, Read};

fn dfs(
    u: &str,
    tree: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, (usize, usize)>,
) -> (usize, usize) {
    if let Some(&result) = memo.get(u) {
        return result;
    }
    let mut desc = 0;
    let mut gen = 0;
    if let Some(children) = tree.get(u) {
        for v in children {
            let (d, g) = dfs(v, tree, memo);
            desc += 1 + d;
            gen = gen.max(1 + g);
        }
    }
    memo.insert(u.to_string(), (desc, gen));
    (desc, gen)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut tree: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let child = it.next().unwrap();
        if child == "***" {
            break;
        }
        let parent = it.next().unwrap();
        tree.entry(parent.to_string()).or_default().push(child.to_string());
    }
    let mut memo = HashMap::new();

    while let Some(cmd) = it.next() {
        if cmd == "***" {
            break;
        }
        let name = it.next().unwrap();
        let (desc, gen) = dfs(name, &tree, &mut memo);
        match cmd {
            "descendants" => {
                println!("{desc}");
            }
            "generation" => {
                println!("{gen}");
            }
            _ => {}
        }
    }
}
