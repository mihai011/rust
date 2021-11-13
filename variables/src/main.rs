fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let x = 5;

    let x = x + 1;
    let x = x * x;

    println!("the value of x is: {}", x);

    let mut spaces = "   ";
    // spaces = spaces.len();

    let c = 'z';
    let z = 'z';
    let heart_eyed_cat = "ㇲ";
    println!("{}", heart_eyed_cat);

    let mut tup: (i32, f64, u8) = (23, 23.4, 8);
    println!("{:?}", tup);
    tup.0 = 34;
    println!("{:?}", tup);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{:?}", a);
    let x = 12;
    another_function(x);
    println!("{}", x);

    let y = {
        let x = 3;
        x + 1
    };

    println!("Value of y {}", y);

    println!("{}", five());

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn another_function(mut x: i32) {
    println!("Another function : {}", x);
    x = x + 1;
}

fn five() -> i32 {
    5
}
