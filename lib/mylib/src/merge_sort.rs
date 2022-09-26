use std::cmp::Ord;

pub trait MergeSort {
    fn merge_sort(&mut self) -> i64;
}

impl<T: Ord + Clone> MergeSort for [T] {
    fn merge_sort(&mut self) -> i64 {
        let mut inv = 0;
        let n = self.len();
        if n > 1 {
            let m = n / 2;
            inv += self[0..m].merge_sort();
            inv += self[m..n].merge_sort();
            let mut w = Vec::with_capacity(n);
            let mut c = m;
            for i in 0..m {
                while c < n && self[c] < self[i] {
                    inv += (m - i) as i64;
                    w.push(self[c].clone());
                    c += 1;
                }
                w.push(self[i].clone());
            }
            for i in c..n {
                w.push(self[i].clone());
            }

            // https://stackoverflow.com/questions/66417950/how-to-assign-to-a-slice-range
            (*self).clone_from_slice(&w);
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
