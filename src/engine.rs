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
