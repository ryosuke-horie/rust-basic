enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// コインをセントの価値に変換する
fn value_in_cents(coin: Coin) -> u8 {
    // Note: ifと違って条件式をBoolではなく、Enumで判定できる
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}
