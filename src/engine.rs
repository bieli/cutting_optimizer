use crate::cutter::Strategy;
use crate::cutter::{Cutter, RodStock};

#[derive(Debug)]
pub struct CutResult {
    pub used_rods: Vec<(i32, i32)>,
    pub offcuts: Vec<i32>,
    pub total_waste: i32,
    pub order: Vec<i32>,
    pub plan: Vec<Vec<i32>>,
    pub rods_flat: Vec<i32>,
}

pub struct CuttingEngine {
    cutter: Cutter,
}

impl CuttingEngine {
    pub fn new(rods: Vec<RodStock>) -> Self {
        Self {
            cutter: Cutter::new(rods),
        }
    }

    /// Backtracking: return cuts plans for all rods
    fn pack_with_plan(rods: &[i32], cuts: &[i32]) -> Option<Vec<Vec<i32>>> {
        let mut cuts_sorted = cuts.to_vec();
        cuts_sorted.sort_by(|a, b| b.cmp(a));

        let mut plan: Vec<Vec<i32>> = rods.iter().map(|_| vec![]).collect();
        let mut caps = rods.to_vec();

        fn backtrack(caps: &mut [i32], plan: &mut [Vec<i32>], cuts: &[i32], idx: usize) -> bool {
            if idx == cuts.len() {
                return true;
            }
            let cut = cuts[idx];

            for i in 0..caps.len() {
                if caps[i] >= cut {
                    caps[i] -= cut;
                    plan[i].push(cut);

                    if backtrack(caps, plan, cuts, idx + 1) {
                        return true;
                    }

                    caps[i] += cut;
                    plan[i].pop();
                }
            }
            false
        }

        if backtrack(&mut caps, &mut plan, &cuts_sorted, 0) {
            Some(plan)
        } else {
            None
        }
    }

    pub fn execute(&mut self, _n: i32, cuts: Vec<i32>) -> Result<CutResult, String> {
        // PHASE 1
        let mut cuts_sorted = cuts.clone();
        cuts_sorted.sort_by(|a, b| b.cmp(a));

        let mut cutter1 = self.cutter.clone();
        let attempt1 = cutter1.cut_all_with_strategy(cuts_sorted.clone(), Strategy::ShortestFirst);

        if attempt1.is_ok() {
            let rods_flat: Vec<i32> = self
                .cutter
                .rods
                .iter()
                .flat_map(|r| std::iter::repeat(r.length).take(r.qty))
                .collect();

            let mut plan: Vec<Vec<i32>> = rods_flat.iter().map(|_| vec![]).collect();
            let mut caps = rods_flat.clone();

            for cut in &cuts_sorted {
                for (i, cap) in caps.iter_mut().enumerate() {
                    if *cap >= *cut {
                        *cap -= *cut;
                        plan[i].push(*cut);
                        break;
                    }
                }
            }

            let mut used_rods = vec![];
            let mut offcuts = vec![];

            for (i, rod_len) in rods_flat.iter().enumerate() {
                if plan[i].is_empty() {
                    continue;
                }
                let used_sum: i32 = plan[i].iter().sum();
                let leftover = rod_len - used_sum;

                used_rods.push((*rod_len, leftover));
                if leftover > 0 {
                    offcuts.push(leftover);
                }
            }

            let total_waste: i32 = offcuts.iter().sum();

            return Ok(CutResult {
                used_rods,
                offcuts,
                total_waste,
                order: cuts_sorted,
                plan,
                rods_flat,
            });
        }

        // PHASE 2
        let mut rods_flat: Vec<i32> = self
            .cutter
            .rods
            .iter()
            .flat_map(|r| std::iter::repeat(r.length).take(r.qty))
            .collect();

        rods_flat.sort_by(|a, b| b.cmp(a));

        if let Some(plan) = Self::pack_with_plan(&rods_flat, &cuts) {
            let mut used_rods = vec![];
            let mut offcuts = vec![];

            for (i, rod_len) in rods_flat.iter().enumerate() {
                if plan[i].is_empty() {
                    continue;
                }
                let used_sum: i32 = plan[i].iter().sum();
                let leftover = rod_len - used_sum;

                used_rods.push((*rod_len, leftover));
                if leftover > 0 {
                    offcuts.push(leftover);
                }
            }

            let total_waste: i32 = offcuts.iter().sum();

            return Ok(CutResult {
                used_rods,
                offcuts,
                total_waste,
                order: cuts,
                plan,
                rods_flat,
            });
        }

        Err("No possible adjustment of cuts to bars (phase 2)".into())
    }
}

pub fn print_report(result: &CutResult) {
    println!("Optimal cut order: {:?}", result.order);
    println!("Used rods:");
    for (len, leftover) in &result.used_rods {
        println!("  Rod {} mm used (leftover {} mm)", len, leftover);
    }
    println!("Offcuts: {:?}", result.offcuts);
    println!("Total waste: {} mm", result.total_waste);
}

pub fn ascii_visualize(order: &[i32], rods: &[i32], plan: &[Vec<i32>]) {
    println!("\n=== ASCII VISUALIZATION ===");

    println!("\nCuts in order:");
    for (i, cut) in order.iter().enumerate() {
        let bar = "#".repeat((*cut / 2).max(1) as usize);
        println!("Cut {:>2}: {:>4} mm |{}", i + 1, cut, bar);
    }

    println!("\n=== RODS USED ===");

    let mut used_index = 1;

    for (rod_idx, rod_len) in rods.iter().enumerate() {
        if plan[rod_idx].is_empty() {
            continue;
        }

        println!(
            "\nRod {}: {} mm (cuts: {:?})",
            used_index, rod_len, plan[rod_idx]
        );
        used_index += 1;

        let scale = 2;
        let total_chars = (*rod_len / scale).max(1) as usize;

        let used_sum: i32 = plan[rod_idx].iter().sum();
        let leftover = rod_len - used_sum;

        let mut bar = if leftover == 0 {
            // pręt zużyty w 100% → sam '#'
            vec!['#'; total_chars]
        } else {
            // klasyczny przypadek: '#' dla cięć, '.' dla pozostałości
            let mut b = vec!['.'; total_chars];
            let mut pos = 0;
            for cut in &plan[rod_idx] {
                let width = (*cut / scale).max(1) as usize;
                for i in pos..(pos + width) {
                    if i < b.len() {
                        b[i] = '#';
                    }
                }
                pos += width;
            }
            b
        };

        let bar_str: String = bar.drain(..).collect();
        println!("|{}|", bar_str);
    }

    println!();
}
