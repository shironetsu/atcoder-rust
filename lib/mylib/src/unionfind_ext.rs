use petgraph::stable_graph::IndexType;
use petgraph::unionfind::UnionFind;
use std::collections::BTreeMap;

#[snippet("union-find-ext-def")]
pub trait UnionFindExt {
    fn into_sizes(self) -> Vec<usize>;
}

#[snippet("union-find-ext")]
#[snippet(include="union-find-ext-def")]
#[snippet(prefix="use petgraph::stable_graph::IndexType;")]
#[snippet(prefix="use petgraph::unionfind::UnionFind;")]
#[snippet(prefix="use std::collections::BTreeMap;")]
impl<K: IndexType> UnionFindExt for UnionFind<K> {
    fn into_sizes(self) -> Vec<usize> {
        let reps = self.into_labeling();
        let mut sizes = BTreeMap::<K, usize>::new();
        for r in reps {
            *sizes.entry(r).or_default() += 1;
        }
        sizes.into_iter().map(|(_, size)| size).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut uf = petgraph::unionfind::UnionFind::<usize>::new(5);
        let sizes = uf.into_sizes();
        assert_eq!(sizes, vec![1, 1, 1, 1, 1]);
    }
}
