fn main() {
    tupleTypes();
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
