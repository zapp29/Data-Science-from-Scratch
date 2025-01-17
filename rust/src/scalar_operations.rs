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
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(1.0, 2.0), -1.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(1.0, 2.0), 2.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(1.0, 2.0), 0.5);
    }
}
