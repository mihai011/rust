use std::io;

fn main() {

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");

    let number: u32 = number.trim().parse()
        .expect("Not a number");
   
   let x = fibo(number);

    println!("{}", x);
}

fn fibo(n : u32) -> i128{

    let mut a = 1;
    let mut b = 1;
    let mut c = 1;

    for i in (0..n){
        c = a+b;
        b = a;
        a = c;
    }
    c
}