use cargo_snippet::snippet;
//use std::mem;
//ll gcd(ll a,ll b,ll& x,ll& y)
//{if(a){ll d=gcd(b%a,a,x,y);y-=(b/a)*x;swap(x,y);if(d<0LL){d=-d;x=-x;y=-y;}return d;}else{x=0;y=1;return b;}}

#[snippet("gcd")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = 0;
        let mut y = 0;
        let a = 120;
        let b = 156;
        let g = gcd(a, b, &mut x, &mut y);
        assert_eq!(g, 12);
        assert_eq!(a*x+b*y, g);
    }
}