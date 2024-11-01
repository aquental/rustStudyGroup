pub fn factors(n: u64) -> Vec<u64> {
    (2..=n)
        .find(|i| n % i == 0)
        .map_or_else(Vec::new, |i| [vec![i], factors(n / i)].concat())
}
