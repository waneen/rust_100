// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

#[derive(Debug, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into() {
        let wrapping: WrappingU32 = 42.into();
        assert_eq!(WrappingU32 { value: 42 }, wrapping);
    }

    fn test_from() {
        let wrapping = WrappingU32::from(42);
        assert_eq!(WrappingU32 { value: 42 }, wrapping);
    }
}
