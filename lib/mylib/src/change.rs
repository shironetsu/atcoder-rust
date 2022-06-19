use cargo_snippet::snippet;

// trait Change<T:PartialOrd>{
//     chmax(x:&mut T, y:T)->bool
//     chmin(x:&mut T, y:T)->bool
// }

// impl<T> Change<T:PartialOrd>{
//     fn chmax(x: &mut T, y: T)->bool{

//     }
// }

// pub fn chmin<T: PartialOrd>(x: &mut T, y: T)->bool{
//     if *x > y{
//         *x = y;
//         true
//     }else{
//         false
//     }
// }

// pub fn chmax<T: PartialOrd>(x: &mut T, y: T)->bool{
//     if *x < y{
//         *x = y;
//         true
//     }else{
//         false
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let mut a = 31;
//         assert_eq!(chmax(&mut a, 41), true);
//         assert_eq!(chmax(&mut a, 59), true);
//         assert_eq!(chmax(&mut a, 26), false);
//         assert_eq!(chmax(&mut a, 53), false);
//         assert_eq!(a, 59);
//     }
// }

#[snippet("change-trait")]
pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}

#[snippet("change")]
#[snippet(include="change-trait")]
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }

    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = 31;
        assert_eq!(a.chmax(41), true);
        assert_eq!(a.chmax(59), true);
        assert_eq!(a.chmax(26), false);
        assert_eq!(a.chmax(53), false);
        assert_eq!(a, 59);
    }
}
