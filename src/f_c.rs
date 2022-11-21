use std::io::stdin;

fn main() {
    let mut x = String::new();

    stdin().read_line(&mut x).expect("001");

    let x: i32 = x.trim().parse().expect("002");

    println!("{} -> {}", x, fib(x));
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn c_to_f(c: f64) -> f64 {
    1.8 * c + 32.0
}

fn fib(i: i32) -> i32 {
    if i == 0 {
        0
    } else if i == 1 {
        1
    } else {
        fib(i - 2) + fib(i - 1)
    }
}
