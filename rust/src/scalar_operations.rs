pub fn add(a: f32, b: f32) -> f32 {
    a + b
}

pub fn subtract(a: f32, b: f32) -> f32 {
    a - b
}

pub fn multiply(a: f32, b: f32) -> f32 {
    a * b
}

pub fn divide(a: f32, b: f32) -> f32 {
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = 2.0;
        let b = 3.0;
        assert_eq!(add(a, b), 5.0);
        assert_eq!(add(b, a), 5.0);
    }

    #[test]
    fn test_subtract() {
        let a = 2.0;
        let b = 3.0;
        assert_eq!(subtract(a, b), -1.0);
        assert_eq!(subtract(b, a), 1.0);
    }

    #[test]
    fn test_multiply() {
        let a = 2.0;
        let b = 3.0;
        assert_eq!(multiply(a, b), 6.0);
        assert_eq!(multiply(b, a), 6.0);
    }

    #[test]
    fn test_divide() {
        let a = 2.0;
        let b = 4.0;
        assert_eq!(divide(a, b), 0.5);
        assert_eq!(divide(b, a), 2.0);
    }
}
