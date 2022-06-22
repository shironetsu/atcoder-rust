#![allow(unused_imports)]
#![allow(non_snake_case)]
use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::*;
use std::fmt::Write;

fn main() {
    input! {
        N: usize,
        mut P: [Usize1;N],
        M: usize,
        ab: [(Usize1, Usize1);M],
    }

    let mut uf = UnionFind::<usize>::new(N);
    let mut ad = (0..N).map(|_| BTreeSet::<usize>::new()).collect::<Vec<_>>();
    let mut dict = BTreeMap::<(usize, usize), usize>::new();
    for (i, &(a, b)) in ab.iter().enumerate() {
        if !uf.equiv(a, b) {
            uf.union(a, b);
            ad[a].insert(b);
            ad[b].insert(a);
            dict.insert((a, b), i);
            dict.insert((b, a), i);
        }
    }
    //println!("{:?}", dict);

    for i in 0..N {
        if !uf.equiv(i, P[i]) {
            println!("-1");
            return;
        }
    }

    let mut Pinv = vec![0; N];
    for i in 0..N {
        Pinv[P[i]] = i;
    }

    let reps = (0..N).filter(|&x| uf.find(x) == x).collect::<Vec<_>>();

    let mut ops = vec![];

    //root
    for &root in reps.iter() {
        let mut todo = VecDeque::<usize>::new();
        todo.push_back(root);
        let mut seen = BTreeSet::<usize>::new();
        seen.insert(root);
        let mut paths = BTreeMap::<usize, Vec<usize>>::new();
        paths.insert(root, vec![]);
        loop {
            if let Some(u) = todo.pop_front() {
                // if visited.contains(&u) {
                //     continue;
                // }
                // visited.insert(u);
                //let mut is_leaf = true;
                for &v in ad[u].iter() {
                    if seen.contains(&v) {
                        continue;
                    }
                    //is_leaf = false;
                    seen.insert(v);
                    todo.push_back(v);
                    let mut path = paths[&u].clone();
                    let edge = dict[&(u, v)];
                    path.push(edge);
                    paths.insert(v, path);
                }
            } else {
                break;
            }
        }
        //println!("{:?}", paths);
        let mut path_lengths = paths
            .iter()
            .map(|(&v, path)| (path.len(), v))
            .collect::<Vec<_>>();
        path_lengths.sort();
        for &(_, u) in path_lengths.iter().rev() {
            let v = Pinv[u];
            if v == u {
                continue;
            }
            let pu = paths.get(&u).unwrap();
            let pv = paths.get(&v).unwrap();

            let mut i = 0;
            while i < pu.len().min(pv.len()) && pu[i] == pv[i] {
                i += 1;
            }

            for &i in pv.iter().skip(i).rev().chain(pu.iter().skip(i)) {
                //write!(ans, " {}", i + 1);
                ops.push(i + 1);
                let (a, b) = ab[i];
                let Pa = P[a];
                let Pb = P[b];
                P[a] = Pb;
                P[b] = Pa;
                Pinv[P[a]] = a;
                Pinv[P[b]] = b;
            }
        }
    }

    let len = ops.len();
    println!("{}", len);
    if len > 0 {
        let ops = ops
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ops);
    }
}
