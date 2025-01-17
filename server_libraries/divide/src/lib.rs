#[must_use]
pub fn divide(left: u64, right: u64) -> u64 {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(8, 2);
        assert_eq!(result, 4);
    }
}
