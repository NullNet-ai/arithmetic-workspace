#[must_use]
pub fn subtract(left: f32, right: f32) -> f32 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = subtract(2.0, 3.5);
        assert_eq!(result, -1.5);
    }
}
