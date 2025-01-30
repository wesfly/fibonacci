// i rewrote it from a cpp file
fn fib(n: i128) -> i128 {
    let mut a: i128 = 0;
    let mut b: i128 = 1;
    let mut c: i128 = 0;
    for i in 2..=n {
        if i < n {
            c = a + b;
        }
        a = b;
        if i <= n {
            b = c;
        }
    }
    return b;
}

fn main() {
    //let n = 9;

    //let x = fib(n);
    for i in 0..186 {
        // NEVER let it run over 186 iterations, it'll panic
        let x: i128 = fib(i);
        println!("Iteration: {}", i + 1);
        println!("{:#?}", x);
    }
    //println!("{:#?}", x);
}
