#[macro_export]
macro_rules! chmin {
    ($x: expr, $y: expr) => {
        if $x > $y {
            $x = $y;
            true
        } else {
            false
        }
    };
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        let mut a = 314;
        assert_eq!(chmin!(a, 159), true);
        assert_eq!(chmin!(a, 265), false);
        assert_eq!(chmin!(a, 358), false);
        assert_eq!(a, 159);
    }
}