use std::fs::File;
use std::io::{self, Write};
mod img;

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

pub fn modu(left: usize, right: usize) -> usize {
    left % right
}

pub fn save_add_image(file_path: &str) -> io::Result<()> {
    let image = img::ADD_IMAGE;
    let mut file = File::create(file_path)?;
    file.write_all(image)?;
    Ok(())
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

    #[test]
    fn test_modu() {
        let result = modu(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_save_add_image() {
        let result = save_add_image("add.png");
        assert!(result.is_ok());
    }
}
