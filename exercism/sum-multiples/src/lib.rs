pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|n| factors.iter().filter(|&&y| y != 0u32).any(|x| n % x == 0))
        .sum()
}
