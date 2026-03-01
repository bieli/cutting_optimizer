pub struct Solution;

impl Solution {
    pub fn optimal_cut_order(n: i32, mut cuts: Vec<i32>) -> Vec<i32> {
        cuts.push(0);
        cuts.push(n);
        cuts.sort_unstable();

        let m = cuts.len();
        let mut dp = vec![vec![0; m]; m];
        let mut choice = vec![vec![None; m]; m];

        for len in 2..m {
            for i in 0..(m - len) {
                let j = i + len;

                let mut best = i32::MAX;
                let mut best_k = None;

                for k in (i + 1)..j {
                    let cost = dp[i][k] + dp[k][j] + (cuts[j] - cuts[i]);
                    if cost < best {
                        best = cost;
                        best_k = Some(k);
                    }
                }

                dp[i][j] = best;
                choice[i][j] = best_k;
            }
        }

        fn build_order(
            i: usize,
            j: usize,
            cuts: &[i32],
            choice: &[Vec<Option<usize>>],
            out: &mut Vec<i32>,
        ) {
            if let Some(k) = choice[i][j] {
                out.push(cuts[k]);
                build_order(i, k, cuts, choice, out);
                build_order(k, j, cuts, choice, out);
            }
        }

        let mut order = vec![];
        build_order(0, m - 1, &cuts, &choice, &mut order);
        order
    }
}
