

mod burrow;
mod boost_farm;
mod utils;
// mod big_decimal;

pub use burrow::*;
pub use boost_farm::*;
pub use utils::*;
// pub use big_decimal::*;

fn main() {
    // burrow
    println!("{}", compute_x_booster_multiplier_at_maximum_staking_duration(31104000, 2592000, to_nano(2592000 * 2), 100, 150));
    println!("{}", compute_x_booster_amount(31104000, 2592000, 65000, 100, to_nano(2592000 * 2)));

    println!("{}", calc_log_base(10u128.pow(20), 1000, 1200));
    println!("{}", calc_extra_shares(9999999999999999583119736832, 10u128.pow(20), 1000));
}