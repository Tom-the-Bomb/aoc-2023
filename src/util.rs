
#[must_use]
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[must_use]
pub fn lcm<I>(nums: I) -> usize
where
    I: Iterator<Item = usize>,
{
    nums.fold(
        1,
        |num, ans| num * ans / gcd(num, ans),
    )
}