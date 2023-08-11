fn main() {
    first_word_return_string_slice(&String::from("this is a test"));
}

fn variables(){
    let x = 5;
    println!("The variable is equal to: {x}");
    let x = x + 1; //shadowing
    println!("The variable is equal to: {x}");
    
}

fn tupleTypes(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let one = tup.2;
    println!("The value of y is :{y}");
    println!("The value of z is : {one}")
}

fn arrays() {
    let months = ["January", "February", "March", "April", "May", "June", 
        "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first_month = months[0];
    println!("The first month is {first_month}");
    let same_numbers = [3;5];
    let should_be_three = same_numbers[4];
    println!("An array with the same number: {should_be_three}");
}

fn functions(x: i32){
    println!("The value of x is: {x}");
    let add_10 = {
        x + 10
    };
    let returnValueFromFunction = functionWithReturn(); 
    println!("After adding 10, the value is: {add_10}");
    println!("The function returns: {returnValueFromFunction}")
}

fn functionWithReturn() -> i32 {
    12
}

fn loops(){
    let mut x :i32 = 0;
    let result = loop {
     println!("The value of x: {x}");

        if x == 5 {            
            break;
        }
        x += 1     
    };

    let mut count = 0;
    'counting_up : loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    // Iterate over an array
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {element}")
    }

    for number in (1..4).rev() {
        println!("{number}")
    }
}


fn ownership(){
    let favorite_string = String::from("Erm");
    let another_string = String::from("Another one");
    println!("My favorite string: {favorite_string}");
    // take_ownership(favorite_string);
    // println!("My favorite string: {favorite_string}, after executing take_ownership function"); //won't work because favorite_string has been disposed
    let favorite_string = take_ownership_and_giveback(favorite_string);
    println!("My favorite string: {favorite_string}, after executing take_ownership_and_giveback function");
    let mut s = String::from("hello"); // This kind of string can be mutated.
    s.push_str(", world!"); //push_str() appends a literal to a string.

    pass_by_reference(&s);
    println!("After the Pass By Reference, variable is: {another_string}");
    pass_by_reference_allowMuteable(&mut s);
    println!("After the Pass By Reference & Allow Muteable, variable is: {s}");
}

fn take_ownership(x: String){
    let s = x;
    println!("X variable is: {s}")
}

fn take_ownership_and_giveback(x : String) -> String {
    let s = x;
    s
}

//Rather then transfering ownership, this allows initial variable to maintain ownership.
fn pass_by_reference(s : &String) -> usize {
    s.len()
}

fn pass_by_reference_allowMuteable(s: &mut String){
    println!("The old value is: {s}");
    for newChar in "-modified".chars() {
        s.push(newChar);
    }

    println!("The new value is: {s}");
}

//Returns a string_slice
fn first_word_return_string_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn string_slices() {
    let s = String::from("Hello World");

    let hello = &s[0..5];
    let word = &s[6..11];
}
