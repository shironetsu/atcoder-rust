use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input!{
        A: i32,
        B: i32,
        C: i32,
        D: i32,
    }

    if (A,B,0) < (C,D,1) {
        println!("Takahashi");
    }else{
        println!("Aoki");
    }
}
