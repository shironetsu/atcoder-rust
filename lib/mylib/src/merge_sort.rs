use std::cmp::Ord;

// https://stackoverflow.com/questions/53204327/how-to-have-a-private-part-of-a-trait
mod private {
    pub trait MergeSortInternal {
        fn merge_sort_internal(&mut self, a: usize, b: usize) -> i64;
    }
}

pub trait MergeSort {
    fn merge_sort(&mut self) -> i64;
}

use private::MergeSortInternal;
impl<T: Ord + Clone> MergeSort for Vec<T> {
    fn merge_sort(&mut self) -> i64 {
        let n = self.len();
        self.merge_sort_internal(0, n)
    }
}

impl<T: Ord + Clone> MergeSortInternal for Vec<T> {
    fn merge_sort_internal(&mut self, a: usize, b: usize) -> i64 {
        let mut inv = 0;
        if b - a > 1 {
            let m = (a + b) / 2;
            inv += self.merge_sort_internal(a, m);
            inv += self.merge_sort_internal(m, b);
            let mut w = vec![];
            let mut c = m;
            for i in a..m {
                while c < b && self[c] < self[i] {
                    inv += (m - i) as i64;
                    w.push(self[c].clone());
                    c += 1;
                }
                w.push(self[i].clone());
            }
            for i in c..b {
                w.push(self[i].clone());
            }
            &self[a..b].clone_from_slice(&w);
        }
        inv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_2413() {
        let mut v = vec![2, 4, 1, 3];
        let inv = v.merge_sort();
        assert_eq!(inv, 3);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn it_works_1234() {
        let mut v = vec![1, 2, 3, 4];
        let inv = v.merge_sort();
        assert_eq!(inv, 0);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn it_works_1() {
        let mut v = vec![1];
        let inv = v.merge_sort();
        assert_eq!(inv, 0);
        assert_eq!(v, vec![1]);
    }

    #[test]
    fn it_works_54321() {
        let mut v = vec![5, 4, 3, 2, 1];
        let inv = v.merge_sort();
        assert_eq!(inv, 10);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
}
