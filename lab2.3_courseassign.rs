use std::io::{self, Read, Write};

struct Assignment {
    n: usize,
    can_teach: Vec<Vec<bool>>,
    conflict: Vec<Vec<bool>>,
    credits: Vec<i64>,
    lower_bounds: Vec<usize>,
    loads: Vec<i64>,
    counts: Vec<usize>,
    assigned: Vec<Option<usize>>,
    answer: i64,
}
impl Assignment {
    fn search(&mut self, course: usize) {
        if course == self.n {
            if self.counts.iter().zip(&self.lower_bounds).any(|(count, bound)| count < bound) {
                return;
            }
            self.answer = self.answer.min(self.loads.iter().copied().max().unwrap_or(0));
            return;
        }
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
    let m: usize = next!();
    let n: usize = next!();
    let mut can_teach = vec![vec![false; n]; m];
    for courses in &mut can_teach {
        let k: usize = next!();
        for _ in 0..k {
            courses[next!(usize) - 1] = true;
        }
    }
    let credits = vec![1; n];
    let lower_bounds = vec![0; m];
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
    write!(out, "{}", if problem.answer == i64::MAX { -1 } else { problem.answer }).unwrap();
}
