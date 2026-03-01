use cutting_optimizer::cutter::RodStock;
use cutting_optimizer::engine::CuttingEngine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_new_does_not_panic() {
        let rods = vec![
            RodStock {
                length: 100,
                qty: 2,
            },
            RodStock { length: 50, qty: 1 },
        ];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_phase1_simple_case() {
        let rods = vec![RodStock {
            length: 100,
            qty: 2,
        }];

        let cuts = vec![60, 40];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.total_waste, 0);
        assert_eq!(result.used_rods.len(), 1);
        assert_eq!(result.used_rods[0], (100, 0));
        assert_eq!(result.plan[0], vec![60, 40]);
    }

    #[test]
    fn test_phase1_multiple_rods() {
        let rods = vec![RodStock {
            length: 100,
            qty: 3,
        }];

        let cuts = vec![70, 20, 10, 50, 40];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert!(result.used_rods.len() >= 2);
        assert_eq!(result.total_waste, result.offcuts.iter().sum());
    }

    #[test]
    fn test_phase2_backtracking_activated() {
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
    }

    #[test]
    fn test_execute_error_when_impossible() {
        let rods = vec![RodStock { length: 30, qty: 1 }];

        let cuts = vec![40];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts);

        assert!(result.is_err());
    }

    #[test]
    fn test_plan_and_rods_flat_consistency() {
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
    fn test_cut_order_sorted_desc() {
        let cuts = vec![10, 50, 20, 30];

        let mut sorted = cuts.clone();
        sorted.sort_by(|a, b| b.cmp(a));

        assert_eq!(sorted, vec![50, 30, 20, 10]);
    }

    #[test]
    fn test_example_case_from_user() {
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
        assert_eq!(result.used_rods.len(), 4);
    }

    #[test]
    fn test_deterministic_output() {
        let rods = vec![RodStock {
            length: 100,
            qty: 3,
        }];

        let cuts = vec![33, 33, 33];

        let mut e1 = CuttingEngine::new(rods.clone());
        let mut e2 = CuttingEngine::new(rods.clone());

        let r1 = e1.execute(100, cuts.clone()).unwrap();
        let r2 = e2.execute(100, cuts.clone()).unwrap();

        assert_eq!(r1.used_rods, r2.used_rods);
        assert_eq!(r1.plan, r2.plan);
        assert_eq!(r1.total_waste, r2.total_waste);
    }

    #[test]
    fn test_perfect_fit_single_rod() {
        let rods = vec![RodStock {
            length: 100,
            qty: 1,
        }];

        let cuts = vec![30, 30, 40];

        let mut engine = CuttingEngine::new(rods);
        let result = engine.execute(100, cuts).unwrap();

        assert_eq!(result.used_rods.len(), 1);
        assert_eq!(result.used_rods[0], (100, 0));
    }
}
