#[macro_export]
macro_rules! chmax {
    ($x: expr, $y: expr) => {
        if $x < $y {
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
        let mut a = 31;
        assert_eq!(chmax!(a, 41), true);
        assert_eq!(chmax!(a, 59), true);
        assert_eq!(chmax!(a, 26), false);
        assert_eq!(chmax!(a, 53), false);
        assert_eq!(a, 59);
    }
}
