pub trait Monoid {
    fn op(&self, rhs: &Self) -> Self;
    fn e() -> Self;
}

impl Monoid for i32 {
    fn op(&self, rhs: &Self) -> Self {
        *self.min(rhs)
    }

    fn e() -> Self {
        i32::MAX
    }
}

#[derive(Debug)]
pub struct SegTree<T: Monoid> {
    log: usize,
    size: usize,
    d: Vec<T>,
}

fn ceil_pow2(n: usize) -> usize {
    let mut m = 1;
    let mut log = 0;
    while n > m {
        m <<= 1;
        log += 1;
    }
    log
}

impl<S: Monoid> SegTree<S> {
    pub fn from(v: Vec<S>) -> SegTree<S> {
        let n = v.len();
        let log = ceil_pow2(n);
        let size = 1 << log;
        let mut d = vec![];
        for _ in 0..2 * size {
            d.push(S::e());
        }
        for (i, x) in v.into_iter().enumerate() {
            d[size + i] = x;
        }

        let mut st = SegTree { d, log, size };
        for i in (1..=size - 1).rev() {
            st.update(i);
        }

        st
    }

    pub fn set(&mut self, p: usize, x: S) {
        let p = p + self.size;
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&self, p: usize) -> Option<&S> {
        self.d.get(p + self.size)
    }

    pub fn prod(&self, l: usize, r: usize) -> S {
        let mut sml = S::e();
        let mut smr = S::e();
        let mut l = l + self.size;
        let mut r = r + self.size;

        while l < r {
            if l & 1 == 1 {
                sml = sml.op(&self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = self.d[r].op(&smr);
            }

            l >>= 1;
            r >>= 1;
        }
        sml.op(&smr)
    }

    pub fn all_prod(&self) -> &S {
        self.d.get(1).unwrap()
    }

    fn update(&mut self, k: usize) {
        self.d[k] = self.d[2 * k].op(&self.d[2 * k + 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ceil_pow2_works() {
        assert_eq!(ceil_pow2(1), 0);
        assert_eq!(ceil_pow2(2), 1);
        assert_eq!(ceil_pow2(3), 2);
        assert_eq!(ceil_pow2(4), 2);
        assert_eq!(ceil_pow2(15), 4);
        assert_eq!(ceil_pow2(16), 4);
        assert_eq!(ceil_pow2(17), 5);
    }

    #[test]
    fn all_prod_works() {
        let mut st = SegTree::from(vec![3, 1, 4, 1, 5, 9]);
        assert_eq!(*st.all_prod(), 1);
        st.set(1, 6);
        st.set(3, 3);
        assert_eq!(*st.all_prod(), 3);
    }

    
    #[test]
    fn prod_works() {
        let mut st = SegTree::from(vec![3, 1, 4, 1, 5, 9]);
        assert_eq!(st.prod(0, 1), 3);
        assert_eq!(st.prod(0, 2), 1);
        assert_eq!(st.prod(4, 6), 5);
    }
}
