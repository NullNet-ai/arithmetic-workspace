#[must_use]
pub fn divide(left: f32, right: f32) -> f32 {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(5.0, 2.0);
        assert_eq!(result, 2.5);
    }
}
