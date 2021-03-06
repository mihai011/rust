use std::vec;

use std::collections::HashMap;

fn latin_converter(word: String)->String{

    let vocals = "aeiouAEIOU";
    let vocals = String::from(vocals);

    let first = word.chars().next().unwrap();

    for v in vocals.chars(){
        if first == v {
            println!("{}-hay", word);
            break;
        }
    }

    let mut flag = 0;
    for v in vocals.chars(){
        if first == v{
            flag = 1;
        }
    }
    if flag == 0{
        println!("{}-fay", &word[1..]);
    }
    word
}

fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    for _ in tqdm_rs::Tqdm::new(0..100) {
        v.push(24);
    }

    let read_vector = vec![1, 2, 3, 4, 5];

    let third: &i32 = &read_vector[2];

    println!("the third element is {}", third);

    match read_vector.get(2) {
        Some(third) => println!("the third element is {}", third),
        None => println!("There is no third element."),
    }

    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);
    // let does_no_exist = &v[100];

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 23, 24];

    for i in &mut v {
        *i += 5;
    }

    println!("{:?}", v);

    enum SpradSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpradSheetCell::Int(3),
        SpradSheetCell::Text(String::from("blue")),
        SpradSheetCell::Float(10.34),
    ];

    let hello = String::from("􏰃􏰄􏰅􏰆􏰇 􏰉􏰊􏰋􏰌􏰍");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("􏰎􏰏􏰐􏰒􏰑􏰓");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s1 = String::from("foo");
    s1.push_str("bar");
    println!("{}", s1);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World");

    let s3 = s1 + &s2;

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");


    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    let field_name = String::from("Favorite Color");
    let blue = map.get(&field_name);

    println!("{:?}", blue);

    for (k,v) in &scores{
        println!("{}:{}", k, v);
    }

    // counting words
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut v = vec![3,4,2,23,42,432,42,424,2,4,5,35,345,346,36744567];

    let mut sum:i32 = 0;
    let sum_ref = &mut sum;

    for number in &v{
        println!("{}", number);
        *sum_ref+=number;
    }

    let vector_length = v.iter().len();

    println!("Mean of the vector is {}", (sum/v.len() as i32) as f32);

    v.sort();

    let mid =(v.iter().count()/2) as usize;

    println!("The median of the vector is {:?}", v[mid]);

    let mut map = HashMap::new();
    
    for element in v{
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mode = map
            .iter()
            .max_by(|a,b| a.1.cmp(&b.1))
            .map(|(k, _v)| k);

    
    println!("the mode for vector is {:?}", mode);

    let apple = String::from("apple");
    let first = String::from("first");

    let latin = latin_converter(apple);
    let latin = latin_converter(first);    
}
