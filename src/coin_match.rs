#[derive(Debug)]
// アメリカの州を表すEnum
pub enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter硬貨はアメリカの州のデザインを持つ
}

// コインをセントの価値に変換する
pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
