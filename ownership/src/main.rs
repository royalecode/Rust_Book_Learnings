// OWNERSHIP
// fn main() {
//     // let mut s: String = String::from("hello");
//     // s.push_str(", world!"); // push_str() appends a literal to a String
//     // println!("{}", s);

//     // let x = 5;
//     // let y = x;
//     // println!("x = {}, y = {}", x, y);

//     // let s1 = String::from("hello");
//     // let s2 = s1.clone();

//     // println!("s1 = {}, s2 = {}", s1, s2);

//     let s = String::from("hello"); // s comes into scope

//     takes_ownership(s); // s's value moves into the function...
//                         // ... and so is no longer valid here

//     let x = 5; // x comes into scope

//     makes_copy(x); // x would move into the function,
//                    // but i32 is Copy, so it's okay to still
//                    // use x afterward

//     // Generate error
//     // x = 20;
//     // println!("{}", x);

//     // println!("{}", s);
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1

//     let s2 = String::from("hello");     // s2 comes into scope

//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

// REFERENCES AND BORROWING
//fn main() {
// without reference
// let s1 = String::from("hello");

// let (s2, len) = calculate_length(s1);

// println!("The length of '{}' is {}.", s2, len);

// with reference
// let mut s1 = String::from("hello");

// // let len = calculate_length(&s1);

// // println!("The length of '{}' is {}.", s1, len);
// change(&mut s1);
// println!("{}", s1);

// let mut s = String::from("hello");

// // let r1 = &mut s;
// {
//     let r1 = &mut s;
// } // r1 goes out of scope here, so we can make a new reference with no problems.
// let r2 = &mut s;

// println!("{}, {}", r1, r2);
//}

// fn calculate_length(s: &String) -> usize {
//     // let length = s.len(); // len() returns the length of a String
//     // (s, length)
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn main() {
    // let mut s = String::from("hello world");
    // let index_word = first_word(&s);
    // println!("The first word is: {}", &s[0..index_word]);

    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // let word = first_word(&s);
    // s.clear(); // error!
    // println!("The first word is: {}", word)

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("{}", word);

    let word = first_word(&my_string[..]);
    println!("{}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("{}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("{}", word);
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
