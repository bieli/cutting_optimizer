use cutting_optimizer::*;

use crate::dp::Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_no_cuts() {
        let result = Solution::optimal_cut_order(100, vec![]);

        assert!(result.is_empty());
    }

    #[test]
    fn test_single_cut() {
        let result = Solution::optimal_cut_order(100, vec![30]);

        assert_eq!(result, vec![30]);
    }

    #[test]
    fn test_two_cuts_unsorted() {
        let result = Solution::optimal_cut_order(100, vec![70, 10]);

        assert_eq!(result, vec![70, 10]);
    }

    #[test]
    fn test_three_cuts_unbalanced() {
        let result = Solution::optimal_cut_order(100, vec![10, 90, 95]);

        assert_eq!(result, vec![10, 90, 95]);
    }

    #[test]
    fn test_four_cuts() {
        let result = Solution::optimal_cut_order(100, vec![20, 40, 60, 80]);

        assert_eq!(result, vec![40, 20, 60, 80]);
    }

    #[test]
    fn test_duplicate_cuts() {
        let result = Solution::optimal_cut_order(100, vec![30, 30, 30]);
        assert_eq!(result, vec![30, 30, 30]);
    }

    #[test]
    fn test_cut_at_zero_and_n_ignored() {
        let result = Solution::optimal_cut_order(100, vec![0, 100, 30, 70]);

        assert_eq!(result, vec![30, 0, 70, 100]);
    }

    #[test]
    fn test_large_gaps() {
        let result = Solution::optimal_cut_order(1000, vec![10, 900]);

        assert_eq!(result, vec![900, 10]);
    }

    #[test]
    fn test_many_cuts() {
        let cuts = vec![5, 20, 35, 50, 65, 80, 95];
        let result = Solution::optimal_cut_order(100, cuts.clone());

        assert_eq!(result[0], 50);
        assert_eq!(result.len(), cuts.len());
    }

    #[test]
    fn test_deterministic_output() {
        let cuts = vec![13, 57, 22, 88, 41];
        let r1 = Solution::optimal_cut_order(100, cuts.clone());
        let r2 = Solution::optimal_cut_order(100, cuts.clone());

        assert_eq!(r1, r2);
    }
}
