

mod burrow;
mod boost_farm;
mod utils;

pub use burrow::*;
pub use boost_farm::*;
pub use utils::*;

fn main() {
    // burrow
    {
        // println!("{}", compute_x_booster_multiplier_at_maximum_staking_duration(31104000, 2592000, to_nano(2592000 * 2), 100, 150));
        // println!("{}", compute_x_booster_amount(31104000, 2592000, 65000, 100, to_nano(2592000 * 2)));

        // println!("{}", calc_log_base(300_0000, 1000, 1300));
        // println!("{}", calc_extra_shares(340282366920938463463374607431768211455, 300_0000, 1000));

        // println!("{}", compute_x_booster_multiplier_at_maximum_staking_duration(7776000, 2592000, to_nano(2592000 * 3), 100, 300));
        // println!("{}", compute_x_booster_amount(7776000, 2592000, 30000, 100, to_nano(2592000 * 3)));
        
        // supress factor = 500, so 1K => 2
        let log_base: u128 = calc_log_base(3000, 1000, 1300);
        println!("xRhea log_base set to: {}", log_base);
        // 389407383983000719942528008192
        // math.log(1500000/500, 389407383983.00072) = math.log(3000, 389407383983.00072) = 0.3
        println!("extra shares per 1000 shares: [500 lock 3 months] {}", calc_extra_shares(log_base, 3, 1000));
        println!("extra shares per 1000 shares: [5K  lock 3 months] {}", calc_extra_shares(log_base, 30, 1000));
        println!("extra shares per 1000 shares: [50K lock 3 months] {}", calc_extra_shares(log_base, 300, 1000));
        println!("extra shares per 1000 shares: [500K lock 3 months] {}", calc_extra_shares(log_base, 3_000, 1000));
        println!("extra shares per 1000 shares: [5M  lock 3 months] {}", calc_extra_shares(log_base, 30_000, 1000));
        println!("extra shares per 1000 shares: [50M lock 3 months] {}", calc_extra_shares(log_base, 300_000, 1000));
        // 1M => 1K
        let log_base: u128 = calc_log_base(3_000, 1000, 1300);
        println!("log_base set to: {}", log_base);
        println!("extra shares per 1000 shares: [1K   booster lock 3 months] {}", calc_extra_shares(log_base, 3, 1000));
        println!("extra shares per 1000 shares: [10K  booster lock 3 months] {}", calc_extra_shares(log_base, 30, 1000));
        println!("extra shares per 1000 shares: [100K booster lock 3 months] {}", calc_extra_shares(log_base, 300, 1000));
        println!("extra shares per 1000 shares: [1M   booster lock 3 months] {}", calc_extra_shares(log_base, 3_000, 1000));
        println!("extra shares per 1000 shares: [10M  booster lock 3 months] {}", calc_extra_shares(log_base, 30_000, 1000));
        println!("extra shares per 1000 shares: [100M booster lock 3 months] {}", calc_extra_shares(log_base, 300_000, 1000));
        let log_base: u128 = u128::MAX;
        println!("[burrow]log_base set to MAX: {}", log_base);
        println!("extra shares per 1000 shares: [1K   booster lock 3 months] {}", calc_extra_shares(log_base, 3, 1000));
        println!("extra shares per 1000 shares: [10K  booster lock 3 months] {}", calc_extra_shares(log_base, 30, 1000));
        println!("extra shares per 1000 shares: [100K booster lock 3 months] {}", calc_extra_shares(log_base, 300, 1000));
        println!("extra shares per 1000 shares: [1M   booster lock 3 months] {}", calc_extra_shares(log_base, 3_000, 1000));
        println!("extra shares per 1000 shares: [10M  booster lock 3 months] {}", calc_extra_shares(log_base, 30_000, 1000));
        println!("extra shares per 1000 shares: [100M booster lock 3 months] {}", calc_extra_shares(log_base, 300_000, 1000));
    }

    // boost_farm
    {
        // println!("{}", boost_farm_compute_max_locking_multiplier(31104000, 2592000 * 2, 100, 150));
        // println!("{}", boost_farm_compute_x_amount(31104000, 40000, 100, 2592000 * 2));

        // println!("{}", boost_farm_calc_log_base(300_0000, 1000, 2000));
        // println!("{}", boost_farm_calc_extra_shares(3000000, 300_0000, 1000));

        let log_base: u32 = boost_farm_calc_log_base(3_000, 100_00, 140_00);
        println!("[boost_farm]log_base set to: {}", log_base);
        println!("[boost_farm]extra shares per 10k shares: [1K   booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 3, 10000));
        println!("[boost_farm]extra shares per 10k shares: [10K  booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 30, 10000));
        println!("[boost_farm]extra shares per 10k shares: [100K booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 300, 10000));
        println!("[boost_farm]extra shares per 10k shares: [1M   booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 3000, 10000));
        println!("[boost_farm]extra shares per 10k shares: [10M  booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 30000, 10000));
        println!("[boost_farm]extra shares per 10k shares: [100M booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 300000, 10000));

        let log_base: u32 = u32::MAX;
        println!("[boost_farm]log_base set to MAX: {}", log_base);
        println!("[boost_farm]extra shares per 10k shares: [1K   booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 3, 10000));
        println!("[boost_farm]extra shares per 10k shares: [10K  booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 30, 10000));
        println!("[boost_farm]extra shares per 10k shares: [100K booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 300, 10000));
        println!("[boost_farm]extra shares per 10k shares: [1M   booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 3000, 10000));
        println!("[boost_farm]extra shares per 10k shares: [10M  booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 30000, 10000));
        println!("[boost_farm]extra shares per 10k shares: [100M booster lock 3 months] {}", boost_farm_calc_extra_shares(log_base, 300000, 10000));

    }
}
