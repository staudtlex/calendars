//! Provides helper functions for calendar conversion

/// Computes the sum \Sigma_{i\ge k, p(i)} f(i), as long as the condition
/// p(i) is true.
pub fn sum<F, P>(f: F, k: i64, p: P) -> f64
where
    F: Fn(f64) -> f64,
    P: Fn(f64) -> bool,
{
    return (k..)
        .take_while(|e: &i64| p(*e as f64))
        .map(|e: i64| f(e as f64))
        .sum();
}

/// Computes the positive remainder of a mod b
pub fn modulus<T>(a: T, b: T) -> T
where
    T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + std::marker::Copy,
{
    return ((a % b) + b) % b;
}

/// Computes the (floored) quotient of two integers a and b
pub fn floor_div(a: i64, b: i64) -> i64 {
    return ((a as f64) / (b as f64)).floor() as i64;
}

/// Computes the adjusted positive remainder modulus(a-1, b) + 1
pub fn amod(a: i64, b: i64) -> i64 {
    return modulus(a - 1, b) + 1;
}
