use petgraph::graph::Graph;
use petgraph::Undirected;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::fmt::Write;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
    }

    let mut ad = (0..N).map(|_| BTreeSet::new()).collect::<Vec<_>>();
    for (a, b) in edges {
        ad[a].insert(b);
        ad[b].insert(a);
    }

    // let mut g = Graph::<(), (), Undirected>::new_undirected();
    // for _ in 0..N {
    //     g.add_node(());
    // }
    // g.extend_with_edges(&edges.iter());

    input! {
        Q: usize,
        queries: [(Usize1, u32);Q],
    }

    let mut ans = String::new();

    for (x, k) in queries.into_iter() {
        let mut visited = BTreeSet::<usize>::new();
        let mut todo = BinaryHeap::from(vec![(-0i32, x)]);
        loop {
            if let Some((d_, x)) = todo.pop() {
                let d = -d_;
                if visited.contains(&x) {
                    continue;
                } else {
                    visited.insert(x);
                    if d < k as i32 {
                        for &y in ad[x].iter() {
                            todo.push((-d - 1, y));
                        }
                    }
                }
            } else {
                break;
            }
        }
        let sum: usize = visited.iter().map(|x| x + 1).sum();
        writeln!(&mut ans, "{}", sum);
    }
    print!("{}", ans);
}
