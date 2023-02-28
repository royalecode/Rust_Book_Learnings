pub use crate::vectors::vectors_code;

fn main() {
    // vectors::vectors_code();
    strings::strings_code();
}

pub mod vectors {
    pub fn vectors_code() {
        // let v: Vec<i32> = Vec::new();
        let v: Vec<i32> = vec![1, 2, 3, 4, 5];

        // let mut v = Vec::new();
        // v.push(5);
        // v.push(6);
        // v.push(7);
        // v.push(8);

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }

        let v = vec![1, 2, 3, 4, 5];
        // let does_not_exist = &v[100];
        let does_not_exist = v.get(100);

        // let mut v = vec![1, 2, 3, 4, 5];

        // let mut first = &v[0];

        // v.push(6);

        // println!("The first element is: {}", first);

        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }

        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
            println!("{}", i);
        }

        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}

pub mod strings {
    pub fn strings_code() {
        let s = String::new();

        let data = "initial contents";
        let s = data.to_string();

        // the method also works on a literal directly
        let s = "initial contents".to_string();
        let s = String::from("initial contents");

        let mut s = String::from("foo");
        s.push_str("bar");

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);

        let mut s = String::from("lo");
        s.push('l');

        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}"); // doesn't take ownership of any of the parameters

        let s1 = String::from("hello");
        // let h = s1[0]; // this will cause a panic

        for c in "नमस्ते".chars() {
            println!("{}", c);
        }

        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }

        let hello = "Здравствуйте";
    }
}
