
use std::io;


fn main(){
    // loop {
    //     print!("Akhila... ")
    // }
    //let mut count = 0;
    // loop {
    //     count += 1;
    //     print!("{} \n", count);
    //     if count == 100000 {
    //         break;
    //     }
    // }
    // while count <= 1000 {
    //     print!("{}\n", count);
    //     count += 1;
    // }
    // for i in 0..=100 {
    //     print!("{}\n", i);
    // }
}

// fn main(){
//     let mut input=String::new();
//     io::stdin().read_line(&mut input).expect("Failed");
//     let input: i32 = input.trim().parse().expect("Sorry you have entered string");
//     let res = match input {
//         1 => "Monday",
//         2 => "Tuesday",
//         3 => "Wednesday",
//         4 => "Thursday",
//         5 => "Friday",
//         6 => "Saturday",
//         7 => "Sunday",
//         _ => "Sunday",
//     };
//     println!("{}", res);
//     // match input {
//     //     1 => print!("Monday"),
//     //     2 => print!("Tuesday"),
//     //     3 => print!("Wednesday"),
//     //     4 => print!("Thursday"),
//     //     5 => print!("Friday"),
//     //     6 => print!("Saturday"),
//     //     7 => print!("Sunday"),
//     //     _ => print!("Wrong"),
//     // };
// }

// fn main(){
//     let mut age: String = String::new();
//     let mut name: String = String::new();
//     print!("Enter your name: ");
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut name).expect("Failed to get input");
//     print!("Enter your age: ");
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut age).expect("Failed to get input");
//     let name = name.trim();
//     let age: i32 = age.trim().parse().expect("please enter the number");
//     println!("Your name is {} and your age is {}", name, age);

// }


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