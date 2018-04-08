extern crate rand;

use self::rand::Rng;

use food::*;
use food::food_lib::FOOD_LIB;

use std::collections::HashMap;

#[derive(Clone)]
pub struct SuggestionEngine {
    library: Vec<Food>,
    available: Vec<i32>,
    prep: Vec<Vec<Food>>,
}

impl SuggestionEngine {
    pub fn new() -> SuggestionEngine {
        //let mut available = Vec::new();
        let mut prep = Vec::new();

        for _ in 0..7 {
            //available.push(HashMap::new());
            prep.push(Vec::new());
        }

        let available = Vec::new();

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
        self.available[day] += time;
    }

    fn can_add_food(&self, food: &Food, day: i32) -> bool {
        let day = day as usize;
        let foods = food.breakdown();
        let mut avail = self.available.clone();
        let mut max_x = day;
        for f in foods {
            let time = f.get_prep_time();
            for x in (0..max_x).rev() {
                if x < max_x {
                    max_x = x
                } else {
                }
                if time <= avail[x] {
                    avail[x] -= time;
                    break;
                } else {
                    if x == 0 {
                        return false;
                    } else {
                    }
                }
            }
        }

        return true;
    }

    fn add_food(&mut self, food: &Food, day: i32) {
        let day = day as usize;
        let foods = food.breakdown();
        let avail = &mut self.available;
        let mut max_x = day;
        for f in foods {
            let time = f.get_prep_time();
            for x in (0..max_x).rev() {
                if x < max_x {
                    max_x = x
                } else {
                }
                if time <= avail[x] {
                    avail[x] -= time;
                    self.prep[x].push(f);
                    break;
                } else {
                }
            }
        }
    }

    fn fill_with_food(&mut self) {
        let mut rand = rand::thread_rng();
        let mut my_lib = self.library.clone();
        for x in (0..7).rev() {
            for _ in (0..10) {
                let trial_food = rand.choose(&my_lib).unwrap();
                if self.can_add_food(trial_food, x) {
                    self.add_food(trial_food, x);
                    break;
                } else {
                }
            }
        }
    }

    fn dishify(&self) -> Vec<Vec<Dish>> {
        let mut vec = Vec::new();
        for x in (0..7) {
            let mut ivec = Vec::new();
            for y in &self.prep[x] {
                ivec.push(y.to_dish(9));
            }
            vec.push(ivec);
        }

        vec
    }
}
