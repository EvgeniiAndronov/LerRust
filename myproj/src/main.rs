// use std::io;
mod math;

macro_rules! my_print {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}

fn main() {
    let sum: i32 = math::add(10, 5);
    my_print!(sum);

    let minus = math::min(10, 5);
    my_print!(minus);
}

// fn macros() {
//     let a: i32 = 321;
//     my_print!(a);
// }

// fn funcs() {
//     let res1: i32 = add(4, 2);
//     let res2: i32 = add(3, -23);
//     println!("{}\n{}", res1, res2);

//     let user: &str = "Name";
//     greet_user(&user);

//     let mut name_to_change = String::from("Bob");
//     change_str(&mut name_to_change);
//     println!("{}", name_to_change);

//     let data = (32, 91);
//     let res = mult(&data);

//     println!("{}", res);
// }


// fn mult(data: &(i32, i32)) -> i32 {
//     data.0 * data.1
// }

// fn change_str(name: &mut String) {
//     *name = String::from("NewBob");
// }

// fn greet_user(name: &str) {
//     println!("name: {}", name);
// }

// fn add(a: i32, b:i32) -> i32 {
//     let res = a + b;
//     return res;
//     // println!("add-{}", res);
// }

// fn loops() {
//     for i in 1..4 {
//         println!("f-1-{}", i);
//     }

//     for i in (1..4).rev() {
//         println!("f-2-{}", i);
//     }

//     for i in (1..4).step_by(2) {
//         println!("f-3-{}", i);
//     }

//     let mut num = 3;

//     while num > 0 {
//         println!("w-1-{}", num);
//         num -= 1;    
//     }

//     for i in 1..20 {
//         if i % 2 == 0{continue;}
//         if i > 7 {break;}
//         println!("f-4-{}", i);
//     }

//     let mut count = 0;

//     loop {
//         count += 1;
//         println!("l-1-{}", count);

//         if count == 5 {
//             break;
//         }
//     }

//     let array = [1, 2, 3, 4, 5, 6, 7, 8];
//     for el in array {
//         println!("f-5-{}", el);
//     }

// }



// fn ifs() {
//     let a: u8 = 10;
//     let b: u8 = 11;
//     let c: u8 = 9;

//     if a < b && b < c {
//         println!("a = {}, b = {}", a, b);
//     } else if a < c || c < b {
//         println!("else if a < c, a = {}, c = {}", a, c);
//     } else {
//         println!("else");
//     }
//     let a = if a > b {0} else {1};
//     println!("{}", a);

//     match a {
//         1 => l(),
//         0 => println!("a = 0"),
//         _ =>println!("not 1, not 0"),
//     }
// }

// fn l() {
//     let mut s = String::from("Hi bitch!");
//     chacnge_str(&mut s);
//     let count_s = calculate_len(&s);
//     println!("{}\ncount symbols is {}", s, count_s);
// }

// fn chacnge_str(s: &mut String) {
//     s.push_str(" Some add to back side string");
// }

// fn test_links() {
//     let s1 = String::from("Sup");
//     let lens = calculate_len(&s1);

//     println!("len of {}, is {}", s1, lens);
// }

// fn calculate_len(s: &String) -> usize {
//     s.len()
// }


// fn test_nums() {
//     let mut inp_val_1 = String::new();
//     let mut inp_val_2: String = String::new();

//     println!("Enter num1: ");
//     io::stdin().read_line(&mut inp_val_1).expect("val 1");

//     println!("Enter num2: ");
//     io::stdin().read_line(&mut inp_val_2).expect("val 2");

//     let data1: i16 = inp_val_1.trim().parse().expect("not valid number");
//     let data2: i16 = inp_val_2.trim().parse().expect("not correct val");

//     let result = data1 + data2;

//     println!("View:\nn1 = {}, n2 = {};\nn1 + n2 = {}", data1, data2, result);
// }
