use std::ops::Mul;
use cosmwasm_std::{Decimal, Uint128};

fn get_share(amount: Uint128, share_percent: u64) -> Uint128 {
    let share = Decimal::percent(share_percent);

    // treat amout like integer, i.e. a decimal with 0 decimal places
    let amount_decimal = Decimal::from_atomics(amount, 0).unwrap();
    println!("amount_decimal: {} ({})", amount_decimal, amount_decimal.atomics());

    // Calculate retained fees
    let fee = amount_decimal.mul(share);
    println!("fee: {} ({})", fee, fee.atomics());

    // return amount converted back from decimal to regular uint128,
    fee.to_uint_floor()
}

fn main() {
    // let's say, payment_token's decimals is 9, so this swap price is actually 1 of that cw20
    assert_eq!(
        get_share(Uint128::new(1_000_000_000), 1),
        Uint128::new(10_000_000),
    );

    assert_eq!(
        get_share(Uint128::new(1_000_000_000), 50),
        Uint128::new(500_000_000),
    );

    assert_eq!(
        get_share(Uint128::new(3_330), 10),
        Uint128::new(333),
    );

    // this one is tricky, as 10% of 333 is 33.3, which is not acceptable for denominated transfer amounts
    // so we expect get_share method to return the int floor, which is 33 in this case
    assert_eq!(
        get_share(Uint128::new(333), 10),
        Uint128::new(33),
    );
}
