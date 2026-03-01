use cutting_optimizer::*;

use crate::cutter::Cutter;
use crate::cutter::RodStock;
use crate::cutter::Strategy;
use cutting_optimizer::engine::CuttingEngine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cutting_strategy_longest_first_basic() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 1,
            },
            RodStock { length: 50, qty: 2 },
        ];

        let mut cutter = Cutter::new(rods);

        cutter
            .cut_all_with_strategy(vec![30, 40, 20], Strategy::LongestFirst)
            .unwrap();

        assert!(cutter.offcuts.contains(&10));
    }
}
