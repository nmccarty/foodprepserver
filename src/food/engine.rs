use food::*;
use food::food_lib::FOOD_LIB;

use std::collections::HashMap;

#[derive(Clone)]
pub struct SuggestionEngine {
    library: Vec<Food>,
    available: Vec<HashMap<i32, i32>>,
    prep: Vec<Vec<Food>>,
}

impl SuggestionEngine {
    pub fn new() -> SuggestionEngine {
        let mut available = Vec::new();
        let mut prep = Vec::new();

        for _ in 0..7 {
            available.push(HashMap::new());
            prep.push(Vec::new());
        }

        SuggestionEngine {
            library: FOOD_LIB
                .clone()
                .into_iter()
                .filter(|x| x.on_own())
                .collect(),
            available: available,
            prep: prep,
        }
    }

    pub fn add_avail(&mut self, day: i32, start: i32, time: i32) {
        let day = day as usize;
        let avail = &mut self.available[day];
        avail.insert(start, time);
    }
}
