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

    pub fn cut_piece_with_strategy(&mut self, size: i32, strategy: Strategy) -> Result<(), String> {
        if let Some(idx) = self
            .offcuts
            .iter()
            .enumerate()
            .filter(|(_, &len)| len >= size)
            .min_by_key(|(_, &len)| len)
            .map(|(i, _)| i)
        {
            let leftover = self.offcuts[idx] - size;
            self.offcuts.remove(idx);
            if leftover > 0 {
                self.offcuts.push(leftover);
            }
            return Ok(());
        }

        let mut rods_sorted: Vec<_> = self.rods.iter_mut().collect();

        match strategy {
            Strategy::ShortestFirst => {
                rods_sorted.sort_by_key(|r| r.length);
            }
            Strategy::LongestFirst => {
                rods_sorted.sort_by_key(|r| -r.length);
            }
        }

        for rod in rods_sorted {
            if rod.qty > 0 && rod.length >= size {
                rod.qty -= 1;
                let leftover = rod.length - size;
                if leftover > 0 {
                    self.offcuts.push(leftover);
                }
                return Ok(());
            }
        }
        /*
            println!("Trying to cut {} mm", size);
            println!("Offcuts: {:?}", self.offcuts);
            println!("Rods:");
            for r in &self.rods {
                println!("  {} mm x {}", r.length, r.qty);
            }
        */
        Err(format!("No rod or offcut can fit size {}", size))
    }

    pub fn cut_all_with_strategy(
        &mut self,
        mut pieces: Vec<i32>,
        strategy: Strategy,
    ) -> Result<(), String> {
        pieces.sort_by(|a, b| b.cmp(a));
        for p in pieces {
            self.cut_piece_with_strategy(p, strategy)?;
        }
        Ok(())
    }
}
