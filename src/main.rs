fn main() {
    functions(12);
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
     
    println!("After adding 10, the value is: {add_10}");
}
