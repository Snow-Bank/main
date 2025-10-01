pub fn mul_div_u128(a: u128, b: u128, denom: u128) -> Option<u128> {
    a.checked_mul(b)?.checked_div(denom)
}
