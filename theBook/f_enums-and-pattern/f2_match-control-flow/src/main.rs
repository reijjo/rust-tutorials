enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {
    let coin = Coin::Dime;

    println!("Value in cents: {}", value_in_cents(coin));

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // println!("Five: {}, six: {}, none: {}", five, six, none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
