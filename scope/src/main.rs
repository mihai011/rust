fn main() {
    let s = "hello"; // on stack, valid from this point here on

    let mut string = String::from("hello"); //on heap mutable

    string.push_str(", world!"); // adds hellow world to string 

    println!("{}", string);

    let x = 5;//integer assigned to variable
    let y = x;// antoher variable with the save value

    let s1 = String::from("hello");// memory on the heap is allocated
    let s2 = s1;//memory is not copied, only the pointer and metadata are 'moved' to s2

    // println!("{}", s1) this will make an error since s1 was moved to s2

    let s1 = String::from("hello");
    let s2 = s1.clone();// this will work but it is expensive

    println!("{} {}", s1, s2);


    let x = 5;
    let y = x; //this works these are on stack

    println!("x={}, y={}", x,y);

    let mut s1 = String::from("control");

    let len = calculate_length(&s1);//passiing a reference not the ownership

    println!("{}", len);

    // trying to change an immutable reference, won't work
    // change(&s1);
    // this will work
    change(&mut s1);
    println!("{}", s1);

    let string = String::from("some control");

    let index = first_word(&string);
    
    println!("{}", index);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

// fn change(s: &String){
//     s.push_str(", some modifications");
// }

fn change(s: &mut String){
    s.push_str(",some string");
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    s.len()
}