use proconio::marker::Usize1;
use proconio::input;
use std::fmt::Write;
use std::collections::BTreeMap;
use std::collections::VecDeque;

#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        M: usize,
        Q: usize,
    }

    let mut ans = String::new();
    let mut ops = Vec::<u32>::new();
    let mut op1 = VecDeque::<(usize, usize, u64)>::new();
    let mut op2 = VecDeque::<(usize, u64)>::new();
    let mut op3 = VecDeque::<(usize, usize)>::new();
    for _ in 0..Q{
        input!{
           op : u32,
        }
        ops.push(op);
        match op {
            1 => {
                input!{
                    l: Usize1,
                    r: Usize1,
                    x: u64,
                }
                op1.push_back((l,r,x));
            },
            2 => {
                input!{
                    i: Usize1,
                    x: u64,
                }
                op2.push_back((i,x));
            },
            3 => {
                input!{
                    i: Usize1,
                    j: Usize1,
                }
                op3.push_back((i,j));
            },
            _ => (),
        }
    }
}
