#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        N: usize,
        A: [i64;N],
    }

    let mut AA = A.iter().copied().enumerate().collect_vec();
    AA.sort_by_key(|&(i, a)|(a, i));
    //println!("{:?}", AA);
    let mut nth = vec![0;N];
    for (i, &(j, a)) in AA.iter().enumerate(){
        nth[j] = i;
    }

    //println!("{:?}", nth);

    let mut B = vec![0;N+1];
    for i in 0..N{
        B[i+1] = B[i] + A[i];
    }

    let mut st = SegmentTree::<usize, OpAdd>::from(vec![0;N]);
    let mut inv = vec![0;N];

    for i in 0..N{
        st.set(nth[i], 1);
        inv[i] = st.prod(nth[i]+1, N);
    }

    
    let mut p = 0i64;
    let mut ans = vec![0;N];
    let mut st2 = SegmentTree::<i64, OpAddi64>::from(vec![0;N]);
    for i in 0..N{
        let k = i - inv[i];
        st2.set(nth[i], A[i]);
        let b = st2.prod(nth[i]+1, N);
        //p += 2 * (B[i]-B[k]) + (2*k as i64+1) * A[i];
        p += 2 * b + (2*k as i64+1) * A[i];
        //println!("{} {}", i, p);
        let e = ModInt::from(p) * (ModInt::from(i as i64+1).inv().pow(2));
        ans[i] = e.val;
    }
    
    ans.ansl();
    //println!("{:?}", inv);
}
//______________________________________________________________________________
//
use std::marker::PhantomData;

struct OpAddi64 {}

impl Op<i64> for OpAddi64 {
    fn op(lhs: &i64, rhs: &i64) -> i64 {
        *lhs + *rhs
    }

    fn e() -> i64 {
        0
    }
}
//______________________________________________________________________________
//
struct OpAdd {}

impl Op<usize> for OpAdd {
    fn op(lhs: &usize, rhs: &usize) -> usize {
        *lhs + *rhs
    }

    fn e() -> usize {
        0
    }
}

pub trait Op<S> {
    fn op(lhs: &S, rhs: &S) -> S;
    fn e() -> S;
}
impl<S: Clone, F: Op<S>> SegmentTree<S, F> {
    pub fn from(v: Vec<S>) -> SegmentTree<S, F> {
        let n = v.len();
        let log = Self::ceil_pow2(n);
        let size = 1 << log;
        let mut d = (0..2 * size).map(|_| F::e()).collect::<Vec<_>>();
        for (i, x) in v.into_iter().enumerate() {
            d[size + i] = x
        }
        let mut st = SegmentTree {
            n,
            log,
            size,
            d,
            _marker: PhantomData,
        };
        for i in (1..=(size - 1)).rev() {
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
    pub fn get(&self, p: usize) -> &S {
        &self.d[p + self.size]
    }
    pub fn prod(&mut self, l: usize, r: usize) -> S {
        let mut sml = F::e();
        let mut smr = F::e();
        let mut l = l + self.size;
        let mut r = r + self.size;
        while l < r {
            if l & 1 == 1 {
                sml = F::op(&sml, &self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = F::op(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        F::op(&sml, &smr)
    }
    pub fn all_prod(&self) -> S {
        self.d[1].clone()
    }
    fn update(&mut self, k: usize) {
        self.d[k] = F::op(&self.d[2 * k], &self.d[2 * k + 1]);
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
pub struct SegmentTree<S: Clone, F: Op<S>> {
    n: usize,
    log: usize,
    size: usize,
    d: Vec<S>,
    _marker: PhantomData<F>,
}
//______________________________________________________________________________
//
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
impl Sub<i64> for ModInt {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self {
        let val = (self.val - rhs).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
impl SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        self.val = (self.val - rhs.val).rem_euclid(Self::MODULO);
    }
}
impl Sub for ModInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let val = (self.val - rhs.val).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
impl Mul<i64> for ModInt {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        let val = (self.val * rhs).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
impl MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.val = (self.val * rhs.val).rem_euclid(Self::MODULO);
    }
}
impl Mul for ModInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let val = (self.val * rhs.val).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
impl Inv for ModInt {
    fn inv(self) -> Self {
        let mut x = 0;
        let mut y = 0;
        gcd(self.val, Self::MODULO, &mut x, &mut y);
        ModInt {
            val: x.rem_euclid(Self::MODULO),
        }
    }
}
pub trait Inv {
    fn inv(self) -> Self;
}
pub fn gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if a != 0 {
        let mut d = gcd(b % a, a, x, y);
        *y -= (b / a) * *x;
        std::mem::swap(x, y);
        if d < 0 {
            d = -d;
            *x = -*x;
            *y = -*y;
        }
        d
    } else {
        *x = 0;
        *y = 1;
        b
    }
}
impl ModInt {
    pub const MODULO: i64 = 998_244_353;
    pub fn new() -> ModInt {
        ModInt { val: 0 }
    }
    pub fn from(x: i64) -> ModInt {
        let val = x.rem_euclid(Self::MODULO);
        ModInt { val }
    }
    pub fn binom(n: i64, k: i64) -> ModInt {
        let mut val = ModInt::from(1);
        for i in 1..=k {
            val *= ModInt::from(n - i + 1) * ModInt::from(i).inv()
        }
        val
    }
    pub fn pow(&self, n: i64) -> ModInt {
        if n == 0 {
            return ModInt::from(1);
        }
        let (a, n) = if n > 0 {
            (self.clone(), n)
        } else {
            (self.inv(), -n)
        };
        let mut k = 0;
        while n >= 1 << k {
            k += 1;
        }
        let mut x = ModInt::from(1);
        for i in (0..k).rev() {
            x = if (n >> i) & 1 == 1 { x * x * a } else { x * x };
        }
        x
    }
}
impl Add<i64> for ModInt {
    type Output = Self;
    fn add(self, rhs: i64) -> Self {
        let val = (self.val + rhs).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
impl AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val).rem_euclid(Self::MODULO);
    }
}
impl Add for ModInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let val = (self.val + rhs.val).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModInt {
    val: i64,
}
//______________________________________________________________________________
//
pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        self.iter().collect::<String>()
    }
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self)->String;
    fn fmtl(&self)->String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn ansl(&self) {
        println!("{}", self.fmtl());
    }
}
//______________________________________________________________________________
//
#[macro_export]
macro_rules! input_edges {
    ($n: expr, $m: expr, $edges: tt, $ad: tt) => {
        input! {
            $edges: [(Usize1, Usize1); $m],
        }

        let mut $ad = vec![vec![]; $n];
        for &(a, b) in $edges.iter() {
            $ad[a].push(b);
            $ad[b].push(a);
        }
        let $ad = $ad;
    };
}

