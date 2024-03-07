use crate::*;

pub const MIN_BOOSTER_MULTIPLIER: u32 = 10000;
pub const BOOSTER_DECIMALS: u32 = 18;

pub fn compute_x_booster_amount(
    maximum_staking_duration_sec: u32, 
    minimum_staking_duration_sec: u32, 
    x_booster_multiplier_at_maximum_staking_duration: u32, 
    amount: u128, 
    duration_ns: u64
) -> u128 {
    amount
        + u128_ratio(
            amount,
            u128::from(
                x_booster_multiplier_at_maximum_staking_duration - MIN_BOOSTER_MULTIPLIER,
            ) * u128::from(duration_ns - to_nano(minimum_staking_duration_sec)),
            u128::from(to_nano(
                maximum_staking_duration_sec - minimum_staking_duration_sec,
            )) * u128::from(MIN_BOOSTER_MULTIPLIER),
        )
}

pub fn compute_x_booster_multiplier_at_maximum_staking_duration(
    maximum_staking_duration_sec: u32, 
    minimum_staking_duration_sec: u32,
    duration_ns: u64,
    capital: u128,
    target: u128
) -> u128 {
    let capital = capital * 10u128.pow(18);
    let target = target * 10u128.pow(18);
    u128_ratio(
        target - capital,
        u128::from(to_nano(
            maximum_staking_duration_sec - minimum_staking_duration_sec,
        )) * u128::from(MIN_BOOSTER_MULTIPLIER),
        u128::from(duration_ns - to_nano(minimum_staking_duration_sec)) * capital
    ) + u128::from(MIN_BOOSTER_MULTIPLIER)
}

pub fn calc_extra_shares(booster_log_base: u128, booster_balance: u128, shares: u128) -> u128 {
    let booster_balance = booster_balance * 10u128.pow(18);
    let booster_base = 10u128.pow(18);
    let log_base =
            (booster_log_base as f64) / (booster_base as f64);
    ((shares as f64)
        * ((booster_balance as f64) / (booster_base as f64)).log(log_base))
        as u128
}

pub fn calc_log_base(booster_balance: u128, shares: u128, target: u128) -> u128 {
    let booster_balance = booster_balance * 10u128.pow(18);
    let booster_base = 10u128.pow(18);
    let extra_shares = target - shares;
    (((booster_balance / booster_base) as f64).powf(1.0f64 / (extra_shares as f64 / shares as f64)) * 10f64.powi(18)) as u128
}
