use crate::*;
pub const BOOSTER_DECIMALS: u32 = 18;
pub const MIN_LOCKING_REWARD_RATIO: u32 = 10000; 

// https://github.com/ref-finance/boost-farm/blob/a6568e2f69e3c34541ac2fa7e48132a4c87a63b0/contracts/boost-farming/src/booster.rs#L67
pub fn boost_farm_calc_extra_shares(booster_log_base: u32, booster_balance: u128, shares: u128) -> u128 {
    let booster_balance = booster_balance * 10u128.pow(BOOSTER_DECIMALS);
    let booster_base = 10u128.pow(BOOSTER_DECIMALS);
    let log_base = booster_log_base as f64;
    ((shares as f64)
        * ((booster_balance as f64) / (booster_base as f64)).log(log_base))
        as u128
}

pub fn boost_farm_calc_log_base(booster_balance: u128, shares: u128, target: u128) -> u32 {
    let booster_balance = booster_balance * 10u128.pow(BOOSTER_DECIMALS);
    let booster_base = 10u128.pow(BOOSTER_DECIMALS);
    let extra_shares = target - shares;
    let log = ((booster_balance / booster_base) as f64).powf(1.0f64 / (extra_shares as f64 / shares as f64));
    if log <= u32::MAX as f64 {
        log as u32
    } else {
        panic!("log > u32::MAX: {}", log as u128);
    }
}

// https://github.com/ref-finance/boost-farm/blob/44f78c156f2819a8c9657c2d2f31e080f2eb998a/contracts/boost-farming/src/farmer_seed.rs#L190
pub fn boost_farm_compute_x_amount(
    maximum_locking_duration_sec: u32, 
    max_locking_multiplier: u32, 
    amount: u128, 
    duration_sec: u32
) -> u128 {
    let amount = amount * 10u128.pow(BOOSTER_DECIMALS);
    amount
        + u128_ratio(
            amount,
            u128::from(max_locking_multiplier - MIN_LOCKING_REWARD_RATIO) * u128::from(to_nano(duration_sec)),
            u128::from(to_nano(maximum_locking_duration_sec)) * MIN_LOCKING_REWARD_RATIO as u128,
        )
}

pub fn boost_farm_compute_max_locking_multiplier(
    maximum_locking_duration_sec: u32, 
    duration_sec: u32,
    capital: u128,
    target: u128
) -> u128 {
    let capital = capital * 10u128.pow(BOOSTER_DECIMALS);
    let target = target * 10u128.pow(BOOSTER_DECIMALS);
    u128_ratio(
        target - capital,
        u128::from(to_nano(maximum_locking_duration_sec)) * MIN_LOCKING_REWARD_RATIO as u128,
        u128::from(to_nano(duration_sec)) * capital
    ) + u128::from(MIN_LOCKING_REWARD_RATIO)
}