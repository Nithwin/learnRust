
use std::io::{self, Write};


fn main(){
    let mut age: String = String::new();
    let mut name: String = String::new();
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Failed to get input");
    print!("Enter your age: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age).expect("Failed to get input");
    let name = name.trim();
    let age: i32 = age.trim().parse().expect("please enter the number");
    println!("Your name is {} and your age is {}", name, age);

}


// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed...");
//     let num:i32 = input.trim().parse().expect("Ithu Number illa");
//     print!("{}", num);
//     // print!("{}", input);
//     // print!("{}", input);
// }


// fn main(){
//     // let name = 55;
//     // let name = name + 4;
//     // //name += " V M ";
//     // println!("{}",name);
//     // for i in 'a'..='z' {
//     //     print!("{} ", i);
//     // }
// }

// fn main() {
//     let mut x = 127;
//     let _f = false;
//     x += 5;
//     assert_eq!(x, 132);
//     let x = 3;
//     println!("{}",x);

//     let (..,x) = (5,5,6,7,8);

//     println!("{}", x);
//     //print_hello();


// }

// fn print_hello(){
//     print!("Hello");
// }