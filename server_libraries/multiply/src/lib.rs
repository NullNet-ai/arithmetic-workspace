#[must_use]
pub fn multiply(left: f32, right: f32) -> f32 {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = multiply(3.5, 2.0);
        assert_eq!(result, 7.0);
    }
}
