use std::io::{self, Read, Write};

struct Assignment {
    n: usize,
    can_teach: Vec<Vec<bool>>,
    conflict: Vec<Vec<bool>>,
    credits: Vec<i64>,
    lower_bounds: Vec<usize>,
    loads: Vec<i64>,
    counts: Vec<usize>,
    // `Option<T>` is either Some(value) or None; absence is represented explicitly, without a null value.
    assigned: Vec<Option<usize>>,
    answer: i64,
}
// `impl Type` defines associated functions and methods; &mut self is a mutable borrow of the receiver.
impl Assignment {
    // `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
    fn search(&mut self, course: usize) {
        if course == self.n {
            // `|x| ...` defines a closure; any returns true when at least one iterator item satisfies it.
            // zip pairs corresponding iterator items; |(a, b)| destructures each pair in the closure.
            if self.counts.iter().zip(&self.lower_bounds).any(|(count, bound)| count < bound) {
                return;
            }
            self.answer = self.answer.min(self.loads.iter().copied().max().unwrap_or(0));
            return;
        }
        // `a..b` excludes b; `a..=b` includes b.
        for teacher in 0..self.loads.len() {
            if !self.can_teach[teacher][course]
                || self.loads[teacher] + self.credits[course] >= self.answer
            {
                continue;
            }
            if (0..course).any(|j| self.assigned[j] == Some(teacher) && self.conflict[course][j]) {
                continue;
            }
            self.assigned[course] = Some(teacher);
            self.loads[teacher] += self.credits[course];
            self.counts[teacher] += 1;
            let remaining = self.n - course - 1;
            if self
                .counts
                .iter()
                .zip(&self.lower_bounds)
                // `|x| ...` defines a closure; all returns true when every iterator item satisfies it.
                .all(|(count, bound)| count + remaining >= *bound)
            {
                self.search(course + 1);
            }
            self.counts[teacher] -= 1;
            self.loads[teacher] -= self.credits[course];
            self.assigned[course] = None;
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
    let m: usize = next!();
    let n: usize = next!();
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    let mut can_teach = vec![vec![false; n]; m];
    // `for x in &mut collection` lends each element mutably instead of consuming the collection.
    // `for x in &collection` borrows its elements; iterating an owned Vec directly would consume it.
    for courses in &mut can_teach {
        let k: usize = next!();
        for _ in 0..k {
            courses[next!(usize) - 1] = true;
        }
    }
    // `|_| ...` is a closure ignoring its argument; map transforms items and collect builds the target collection.
    let credits = (0..n).map(|_| next!()).collect();
    let lower_bounds = (0..m).map(|_| next!()).collect();
    let mut conflict = vec![vec![false; n]; n];
    let k: usize = next!();
    for _ in 0..k {
        let u = next!(usize) - 1;
        let v = next!(usize) - 1;
        conflict[u][v] = true;
        conflict[v][u] = true;
    }
    let mut problem = Assignment {
        n,
        can_teach,
        conflict,
        credits,
        lower_bounds,
        loads: vec![0; m],
        counts: vec![0; m],
        assigned: vec![None; n],
        answer: i64::MAX,
    };
    if problem.lower_bounds.iter().sum::<usize>() <= n {
        problem.search(0);
    }
    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
    // Rust if/else is an expression; both branches must produce the same type.
    write!(out, "{}", if problem.answer == i64::MAX { -1 } else { problem.answer }).unwrap();
}
