use std::collections::HashMap;
use std::io::{self, Read};


// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
fn dfs(u: &str, tree: &HashMap<String, Vec<String>>, memo: &mut HashMap<String, (usize, usize)>) -> (usize, usize){
    // `Some(&x)` matches an Option containing a reference and copies its Copy value into x.
    if let Some(&result) = memo.get(u){
        return result;
    }

    let mut desc = 0;
    let mut gen = 0;

    // `if let` runs this branch only when the value matches Some(...), binding its contents.
    if let Some(children) = tree.get(u) {
        for v in children {
            // Tuple destructuring binds each component separately; mut applies only to the binding it precedes.
            let (d,g) = dfs(v, tree, memo);
            desc += 1 + d;
            gen = gen.max(1+g);
        }
    }

    memo.insert(u.to_string(), (desc, gen));
    // No trailing semicolon: this tuple is returned as the function's value.
    (desc,gen)
}

fn main() {
    // `let` bindings are immutable by default; `mut` allows changes. String owns its text.
    let mut input = String::new();
    // `&mut input` lends exclusive mutable access; unwrap() extracts Ok or panics on an I/O error.
    io::stdin().read_to_string(&mut input).unwrap(); //use ctrl+D to send EOF
    // The iterator yields borrowed &str tokens; input must stay alive while they are used.
    let mut it = input.split_whitespace();

    let mut tree: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let child = it.next().unwrap();
        if child == "***" {
            break;
        }
        let parent = it.next().unwrap();
        // entry(...).or_default() returns a mutable reference, inserting a default value if absent; * dereferences it.
        tree.entry(parent.to_string()).or_default().push(child.to_string());
    }

    let mut memo = HashMap::new();

    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
    while let Some(cmd) = it.next() {
        if cmd == "***" { break; }

        let name = it.next().unwrap();
        let (desc, gen) = dfs(name, &tree, &mut memo);

        // `match` selects a pattern arm (`pattern => expression`); `_` matches any remaining value.
        match cmd {
            // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
            "descendants" => {println!("{desc}");}
            "generation" => {println!("{gen}");}
            _ => {}
        }
    }
}