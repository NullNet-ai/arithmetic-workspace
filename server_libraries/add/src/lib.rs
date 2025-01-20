#[must_use]
pub fn add(left: f32, right: f32) -> f32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2.5, 2.1);
        assert_eq!(result, 4.6);
    }
}
