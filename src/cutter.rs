#[derive(Debug, Clone, Copy)]
pub enum Strategy {
    ShortestFirst,
    LongestFirst,
}

#[derive(Debug, Clone)]
pub struct RodStock {
    pub length: i32,
    pub qty: usize,
}

#[derive(Debug, Clone)]
pub struct Cutter {
    pub rods: Vec<RodStock>,
    pub offcuts: Vec<i32>,
}

impl Cutter {
    pub fn new(mut rods: Vec<RodStock>) -> Self {
        rods.sort_by_key(|r| r.length);
        Self {
            rods,
            offcuts: vec![],
        }
    }
}
