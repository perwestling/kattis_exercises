use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_usize_numbers() {
        assert_eq!(vec!(3, 1, 2), get_usize_numbers("3 1 2"));
    }

}

