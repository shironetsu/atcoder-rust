use cargo_snippet::snippet;
use std::marker::PhantomData;

#[snippet("lazy_segtre-op")]
pub trait Op<S> {
    fn op(lhs: &S, rhs: &S) -> S;
    fn e() -> S;
}

#[snippet("lazy_segtre-action")]
pub trait Action<S> {
    fn apply(&self, x: &S) -> S;
    fn composition(&self, rhs: &Self) -> Self;
    fn id() -> Self;
}

#[snippet("lazy_segtree")]
#[snippet(include="lazy_segtre-op")]
#[snippet(include="lazy_segtre-action")]
#[snippet(include="lazy_segtree-impl")]
#[snippet(prefix="use std::marker::PhantomData;")]
struct LazySegmentTree<S, T: Op<S>, U: Action<S>> {
    n: usize,
    log: usize,
    size: usize,
    d: Vec<S>,
    _marker: PhantomData<T>,
    lz: Vec<U>,
}

#[snippet("lazy_segtree-impl")]
impl<S: Clone, T: Op<S>, U: Action<S> + Clone> LazySegmentTree<S, T, U> {
    pub fn from(v: Vec<S>) -> LazySegmentTree<S, T, U> {
        let n = v.len();
        let log = Self::ceil_pow2(n);
        let size = 1 << log;
        let mut d = (0..2 * size).map(|_| T::e()).collect::<Vec<_>>();
        let lz = (0..size).map(|_| U::id()).collect::<Vec<_>>();
        for (i, x) in v.into_iter().enumerate() {
            d[size + i] = x;
        }
        let mut lst = LazySegmentTree {
            n,
            log,
            size,
            d,
            _marker: PhantomData,
            lz,
        };
        for i in (1..=size - 1).rev() {
            lst.update(i);
        }
        lst
    }

    pub fn set(&mut self, p: usize, x: S) {
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&mut self, p: usize) -> &S {
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        &self.d[p]
    }

    pub fn prod(&mut self, l: usize, r: usize) -> S {
        if l == r {
            return T::e();
        }

        let mut l = l + self.size;
        let mut r = r + self.size;

        for i in (1..=self.log).rev() {
            if (l >> i) << i != l {
                self.push(l >> i);
            }
            if ((r >> i) << i != r) {
                self.push((r - 1) >> i);
            }
        }

        let mut sml = T::e();
        let mut smr = T::e();
        while l < r {
            if l & 1 == 1 {
                sml = T::op(&sml, &self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = T::op(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }

        T::op(&sml, &smr)
    }

    pub fn all_prod(&self) -> S {
        self.d[1].clone()
    }

    pub fn apply_at(&mut self, p: usize, f: U) {
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = f.apply(&self.d[p]);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn apply_range(&mut self, l: usize, r: usize, f: &U) {
        if l == r {
            return;
        }

        let l = l + self.size;
        let r = r + self.size;

        for i in (1..=self.log).rev() {
            if (l >> i) << i != l {
                self.push(l >> i);
            }
            if (r >> i) << i != r {
                self.push((r - 1) >> i);
            }
        }

        {
            let mut l = l;
            let mut r = r;
            while l < r {
                if l & 1 == 1 {
                    self.all_apply(l, &f);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    self.all_apply(r, &f);
                }
                l >>= 1;
                r >>= 1;
            }
        }

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    fn update(&mut self, k: usize) {
        self.d[k] = T::op(&self.d[2 * k], &self.d[2 * k + 1]);
    }

    fn all_apply(&mut self, k: usize, f: &U) {
        self.d[k] = f.apply(&self.d[k]);
        if k < self.size {
            self.lz[k] = f.composition(&self.lz[k]);
        }
    }

    fn push(&mut self, k: usize) {
        let f = self.lz[k].clone();
        self.all_apply(2 * k, &f);
        self.all_apply(2 * k + 1, &f);
        self.lz[k] = U::id();
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
}
