// 1. "Give" owner ship to another, you loose ownership.
// fn main() {
//     let mut owner = String::from("Hello, World");

//     {
//         let mut another_owner = owner; // passing ownership
//         another_owner = String::from("Hello, New World");
//         println!("{}", owner);  // this line cannot compile
//         println!("{}", another_owner);
//     }

//     println!("{}", owner);  // this line cannot compile
// }

// 2. When shared(borrowed), mutate not allowed. Or you can share immutalbe stuff.
// fn main() {
//     let mut owner = String::from("Hello, World");

//     {
//         let mut borrower = &owner; // shared borrow
//         *borrower = String::from("Hello, New World"); // this line cannot compile
//         println!("{}", owner);
//         println!("{}", borrower);
//     }

//     println!("{}", owner);
// }

// 3. When mutable shared (mut borrow), first guy loose access!
// fn main() {
//     let mut owner = String::from("Hello, World");

//     {
//         let mut borrower = &mut owner; // mutable borrow
//         *borrower = String::from("Hello, New World");;
//         println!("{}", owner);  // this line cannot compile
//         println!("{}", borrower);
//     }

//     println!("{}", owner);
// }

//////////////////////////////////////////////////

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("'{}' has length: {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // or (*s).len() also works. s1.len() &s1.len() both works. Interesting...
}
