use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_if_same_column_or_row() {
        assert_eq!(true, is_collision((&1, &2), (&3, &2)));
        assert_eq!(true, is_collision((&2, &1), (&2, &3)));
        assert_eq!(true, is_collision((&2, &1), (&2, &1)));
        assert_eq!(false, is_collision((&2, &1), (&3, &7)));
    }

    #[test]
    fn test_collision_if_lr_diagonal() {
        assert_eq!(true, is_collision((&1, &1), (&7, &7)));
        assert_eq!(true, is_collision((&2, &3), (&4, &5)));
        assert_eq!(true, is_collision((&6, &6), (&2, &2)));
        assert_eq!(true, is_collision((&6, &4), (&4, &2)));
        assert_eq!(true, is_collision((&6, &1), (&2, &5)));
        assert_eq!(true, is_collision((&3, &4), (&4, &3)));
        assert_eq!(true, is_collision((&2, &4), (&5, &7)));
        assert_eq!(false, is_collision((&3, &4), (&0, &0)));
    }
        
}

