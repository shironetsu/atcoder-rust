macro_rules! min {
    ($a: expr, $($b:expr),*) => {
        {
            let mut x = $a;
            $(
                if x > $b {
                    x = $b;
                }
            )*
            x
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = min!(31, 4, 15, 9, 26);
        assert_eq!(x, 4);
    }

    // fn single_value() {
    //     let x = min!(42);
    //     assert_eq!(x, 42);
    // }
}
