fn main() {
    println!("The {}th Fibonacci number is {}", 10, fibonacci(10));
}

fn fibonacci(n: u32) -> u32 {
    let mut f_n_2 = 0;
    let mut f_n_1 = 1;
    let mut f_n = f_n_1 + f_n_2;
    match n {
        0 => f_n_2,
        1 => f_n_1,
        _ => {
            for _ in 2..n {
                f_n_2 = f_n_1;
                f_n_1 = f_n;
                f_n = f_n_1 + f_n_2;
            }
            f_n
        }
    }
}
