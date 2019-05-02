use {heapless::ArrayLength, heapless_bigint::BigUint};

pub fn powm<N: ArrayLength<u8>>(
    base: &BigUint<N>,
    exp: &BigUint<N>,
    modulus: &BigUint<N>,
) -> BigUint<N> {
    let zero = BigUint::from_bytes_be(&[0]);
    let one = BigUint::from_bytes_be(&[1]);
    let two = BigUint::from_bytes_be(&[2]);
    let mut exp = exp.clone();
    let mut result = one.clone();
    let mut base = base % modulus;

    while exp > zero {
        if &exp % &two == one {
            result = &(&result * &base) % modulus;
        }
        exp >>= 1;
        base = &(&base * &base) % modulus;
    }
    result
}
