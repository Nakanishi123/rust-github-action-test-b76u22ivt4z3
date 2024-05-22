pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn mul(left: usize, right: usize) -> usize {
    left * right
}

pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

pub fn div(left: usize, right: usize) -> usize {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_mul() {
        let result = mul(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sub() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_div() {
        let result = div(2, 2);
        assert_eq!(result, 1);
    }
}
