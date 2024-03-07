use crate::*;


uint::construct_uint!(
    pub struct U256(4);
);

pub(crate) fn u128_ratio(a: u128, num: u128, denom: u128) -> u128 {
    (U256::from(a) * U256::from(num) / U256::from(denom)).as_u128()
}

pub fn to_nano(ts: u32) -> u64 {
    u64::from(ts) * 10u64.pow(9)
}
