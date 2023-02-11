#[derive(Debug)] // so we can inspect the state in a minute
enum UseState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UseState),
}

fn main() {
    let coin = Coin::Quarter(UseState::Alaska);
    let cent_value = value_in_cents(coin);
    println!("The value of the coin is {} cents", cent_value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
