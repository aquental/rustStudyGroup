pub fn egg_count(display_value: u32) -> usize {
    (0..32).fold(0, |count, i| count + ((display_value >> i) & 1) as usize)
}
