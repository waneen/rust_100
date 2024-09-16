fn compute(a: u32, b: u32) -> u32 {
    // TODO: change the line below to fix the compiler error and make the tests pass.
    a + b * 1_00__0_00_0u32
}

fn main() {
    let value = compute(1, 2);
    println!("{value:?}")
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 2000001);
    }
}
