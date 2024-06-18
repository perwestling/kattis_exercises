use super::*;

#[cfg(test)]
mod tests {
    // Took test datasets from https://people.sc.fsu.edu/~jburkardt/datasets/bin_packing/bin_packing.html
    use super::*;

    #[test]
    fn test_p02() {
        assert_eq!(
            (51, 26, vec![2, 3, 4]),
            knapsack(26, vec![12, 7, 11, 8, 9], vec![24, 13, 23, 15, 16])
        );
    }

    #[test]
    fn test_p04() {
        assert_eq!(
            (150, 190, vec![1, 2, 5]),
            knapsack(
                190,
                vec![56, 59, 80, 64, 75, 17],
                vec![50, 50, 64, 46, 50, 5]
            )
        );
    }

    #[test]
    fn test_p01() {
        assert_eq!(
            (309, 165, vec![1, 2, 3, 4, 6]),
            knapsack(
                165,
                vec![23, 31, 29, 44, 53, 38, 63, 85, 89, 82],
                vec![92, 57, 49, 68, 60, 43, 67, 84, 87, 72]
            )
        );
    }

    #[test]
    fn test_p06() {
        assert_eq!(
            (1735, 169, vec![2, 4, 7]),
            knapsack(
                170,
                vec![41, 50, 49, 59, 55, 57, 60],
                vec![442, 525, 511, 593, 546, 564, 617]
            )
        );
    }

    #[test]
    fn test_p07() {
        assert_eq!(
            (1458, 749, vec![1, 3, 5, 7, 8, 9, 14, 15]),
            knapsack(
                750,
                vec![70, 73, 77, 80, 82, 87, 90, 94, 98, 106, 110, 113, 115, 118, 120],
                vec![135, 139, 149, 150, 156, 163, 173, 184, 192, 201, 210, 214, 221, 229, 240]
            )
        );
    }
}
