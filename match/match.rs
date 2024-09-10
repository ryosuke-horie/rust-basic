#[derive(Debug)]
// アメリカの州を表すEnum
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quater硬貨はアメリカの州のデザインを持つ
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
        // value_in_cents(Coin::Quarter(UsState::Alaska))で呼び出すと
        // State quarter from Alaska! と表示され、25が返される
        Coin::Quarter(state) =< {
            println!("State quarter from {state:?}!"); // 
            25
        }
    }
}
