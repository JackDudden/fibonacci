use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n: i32 = match &args[1].trim().parse() {
        Ok(v) => *v,
        Err(_) => {
            println!("First argument must be an integer");
            return;
        }
    };

    let result = fibonnaci(n);

    println!("Fibonnaci number for {n} is {result}");
}

fn fibonnaci(n: i32) -> i32 {
    if [0, 1].contains(&n) {
        n
    } else {
        fibonnaci(n - 1) + fibonnaci(n - 2)
    }
}
