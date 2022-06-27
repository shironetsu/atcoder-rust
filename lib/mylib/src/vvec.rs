use cargo_snippet::snippet;

#[snippet("vvec")]
#[macro_export]
macro_rules! vvec {
    ($val: expr; $a:expr, $b:expr) => {
        vec![vec![$val; $b]; $a]
    };
}

#[snippet("vvvec")]
#[macro_export]
macro_rules! vvvec {
    ($val: expr; $a:expr, $b:expr, $c:expr) => {
        vec![vec![vec![$val; $c]; $b]; $a]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vvec![0;3,4], vec![vec![0; 4]; 3]);
    }
}
