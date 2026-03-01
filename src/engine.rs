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
