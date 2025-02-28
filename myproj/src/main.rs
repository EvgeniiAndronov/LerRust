use std::io;

fn main() {
    let mut inp_val_1 = String::new();
    let mut inp_val_2: String = String::new();

    println!("Enter num1: ");
    io::stdin().read_line(&mut inp_val_1).expect("val 1");

    println!("Enter num2: ");
    io::stdin().read_line(&mut inp_val_2).expect("val 2");

    let data1: i16 = inp_val_1.trim().parse().expect("not valid number");
    let data2: i16 = inp_val_2.trim().parse().expect("not correct val");

    let result = data1 + data2;

    println!("View:\nn1 = {}, n2 = {};\nn1 + n2 = {}", data1, data2, result);
} 
