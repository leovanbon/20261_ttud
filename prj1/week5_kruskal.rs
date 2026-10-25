
use std::io::{self, Read};

macro_rules! read {
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().unwrap()
    };
}

struct Edge {
    u: usize,
    v: usize,
    w: i64,
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            size: vec![1; n + 1],
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.parent[u] != u {
            self.parent[u] = self.find(self.parent[u]);
        }
        self.parent[u]
    }

    fn unite(&mut self, u: usize, v: usize) -> bool {
        let mut u = self.find(u);
        let mut v = self.find(v);

        if u == v {
            return false;
        }

        if self.size[u] < self.size[v] {
            std::mem::swap(&mut u, &mut v);
        }

        self.parent[v] = u;
        self.size[u] += self.size[v];

        true
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = read!(iter, usize);
    let m: usize = read!(iter, usize);

    let mut edges = Vec::with_capacity(m);

    for _ in 0..m {
        let u = read!(iter, usize);
        let v = read!(iter, usize);
        let w = read!(iter, usize);

        edges.push(Edge { u, v, w });
    }

    edges.sort_unstable_by_key(|e| e.w);

    let mut dsu = DSU::new(n);
    let mut ans: i64 = 0;
    let mut count = 0;

    for Edge { u, v, w } in edges {
        if dsu.unite(u, v) {
            ans += w;
            count += 1;

            if count == n - 1 {
                break;
            }
        }
    }

    println!("{ans}");
}
