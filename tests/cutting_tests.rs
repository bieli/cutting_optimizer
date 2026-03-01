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

    #[test]
    fn test_cutting_strategy_shortest_first_basic() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 1,
            },
            RodStock { length: 50, qty: 2 },
        ];

        let mut cutter = Cutter::new(rods);

        cutter
            .cut_all_with_strategy(vec![30, 40, 10], Strategy::ShortestFirst)
            .unwrap();

        assert!(cutter.offcuts.contains(&20));
    }

    #[test]
    fn test_uses_shortest_rods_first() {
        let rods = vec![
            RodStock {
                length: 200,
                qty: 1,
            },
            RodStock { length: 80, qty: 1 },
            RodStock { length: 60, qty: 1 },
        ];

        let mut cutter = Cutter::new(rods);

        cutter
            .cut_piece_with_strategy(50, Strategy::ShortestFirst)
            .unwrap();

        assert!(cutter.offcuts.contains(&10));
    }

    #[test]
    fn test_uses_longtest_rods_first() {
        let rods = vec![
            RodStock {
                length: 200,
                qty: 1,
            },
            RodStock { length: 80, qty: 1 },
            RodStock { length: 60, qty: 1 },
        ];

        let mut cutter = Cutter::new(rods);

        cutter
            .cut_piece_with_strategy(50, Strategy::LongestFirst)
            .unwrap();

        assert!(cutter.offcuts.contains(&150));
    }

    #[test]
    fn test_offcut_preferred_over_rod() {
        let rods = vec![RodStock {
            length: 100,
            qty: 1,
        }];

        let mut cutter = Cutter::new(rods);

        cutter
            .cut_piece_with_strategy(70, Strategy::ShortestFirst)
            .unwrap();
        cutter
            .cut_piece_with_strategy(20, Strategy::ShortestFirst)
            .unwrap();

        assert!(cutter.offcuts.contains(&10));
        assert_eq!(cutter.rods[0].qty, 0);
    }

    #[test]
    fn test_no_available_rod() {
        let rods = vec![RodStock { length: 40, qty: 1 }];

        let mut cutter = Cutter::new(rods);

        let result = cutter.cut_piece_with_strategy(50, Strategy::ShortestFirst);
        assert!(result.is_err());
    }

    #[test]
    fn test_phase2_backtracking() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 1,
            },
            RodStock { length: 40, qty: 1 },
            RodStock { length: 10, qty: 3 },
        ];

        let cuts = vec![50, 40, 30, 20, 10];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.total_waste, 0);
        assert_eq!(result.used_rods.len(), 3);
        assert_eq!(result.rods_flat.len(), 5);
        assert_eq!(result.offcuts.len(), 0);
    }

    #[test]
    fn test_plan_structure() {
        let rods = vec![RodStock {
            length: 100,
            qty: 2,
        }];

        let cuts = vec![70, 20, 10];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.plan.len(), result.rods_flat.len());

        let sum_cuts: i32 = result.plan.iter().flatten().sum();
        assert_eq!(sum_cuts, 70 + 20 + 10);
    }

    #[test]
    fn test_simple_cutting001() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 1,
            },
            RodStock { length: 40, qty: 1 },
            RodStock { length: 10, qty: 3 },
        ];

        let cuts = vec![50, 40, 30, 20, 10];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.total_waste, 0);
        assert_eq!(result.used_rods.len(), 3);
        assert_eq!(result.rods_flat.len(), 5);
        assert_eq!(result.offcuts.len(), 0);
    }

    #[test]
    fn test_simple_cutting002() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 2,
            },
            RodStock { length: 40, qty: 1 },
            RodStock { length: 10, qty: 3 },
        ];

        let cuts = vec![50, 40, 30, 20, 10];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.total_waste, 0);
        assert_eq!(result.used_rods.len(), 3);
        assert_eq!(result.rods_flat.len(), 6);
        assert_eq!(result.offcuts.len(), 0);
    }

    #[test]
    fn test_simple_cutting003() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 2,
            },
            RodStock { length: 40, qty: 1 },
            RodStock { length: 30, qty: 1 },
            RodStock { length: 10, qty: 3 },
        ];

        let cuts = vec![50, 40, 30, 20, 10, 10, 10];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.total_waste, 30);
        assert_eq!(result.used_rods.len(), 6);
        assert_eq!(result.rods_flat.len(), 7);
        assert_eq!(result.offcuts, vec![30]);
    }

    #[test]
    fn test_simple_cutting004() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 5,
            },
            RodStock { length: 50, qty: 2 },
        ];

        let cuts = vec![90, 60, 45, 5, 5, 5, 10];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.total_waste, 80);
        assert_eq!(result.rods_flat.len(), 7);
        assert_eq!(result.offcuts, vec![30, 10, 40]);
    }

    #[test]
    fn test_simple_cutting_005() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 5,
            },
            RodStock { length: 50, qty: 2 },
        ];

        let cuts = vec![15, 35, 5, 5, 5, 25, 45];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.total_waste, 65);
        assert_eq!(result.used_rods.len(), 3);
        assert_eq!(result.rods_flat.len(), 7);
        assert_eq!(result.offcuts, vec![65]);
    }

    #[test]
    fn test_simple_cutting_006() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 1,
            },
            RodStock { length: 50, qty: 1 },
        ];

        let cuts = vec![95, 1, 4, 5];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.total_waste, 45);
        assert_eq!(result.used_rods.len(), 2);
        assert_eq!(result.rods_flat.len(), 2);
        assert_eq!(result.offcuts, vec![40, 5]);
    }

    #[test]
    fn test_simple_cutting_007_error() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 1,
            },
            RodStock { length: 50, qty: 0 },
        ];

        let cuts = vec![95, 1, 4, 5];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts);

        assert!(result.is_err());
    }

    #[test]
    fn test_simple_cutting_008() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 3,
            },
            RodStock { length: 50, qty: 2 },
        ];

        let cuts = vec![10, 60, 45, 10, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 90, 5];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.total_waste, 30);
        assert_eq!(result.used_rods.len(), 4);
        assert_eq!(result.rods_flat.len(), 5);
        assert_eq!(result.offcuts, vec![30]);
    }

    #[test]
    fn test_simple_cutting_009() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 2,
            },
            RodStock { length: 40, qty: 2 },
            RodStock { length: 10, qty: 3 },
        ];

        let cuts = vec![50, 40, 30, 20, 10, 15];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.total_waste, 25);
        assert_eq!(result.used_rods.len(), 4);
        assert_eq!(result.rods_flat.len(), 7);
        assert_eq!(result.offcuts, vec![10, 15]);
    }
}
