fn main() {
    // mutable variabbles
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants
    // dead code
    const MAX_POINTS: u32 = 100_000;

    // shadowing

    let x: u8 = 100;

    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    // mutable variables are static error here
    // let mut spaces = "  ";
    // spaces = spaces.len();

    // experiment
    // let guess: str = "42".parse().expect("Error");
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    let c = 'z';
    let z = 'z';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("{}", z);

    let a  = [1,2,3,4,5];
    let index = 3;

    println!(" The value of element is: {}", a[index]);

    another_function(24);
}

fn another_function(x: i32){
    println!("The value of x is: {}", x);
}
