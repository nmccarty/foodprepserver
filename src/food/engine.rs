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
        let mut available = Vec::new();

        for _ in 0..7 {
            //available.push(HashMap::new());
            prep.push(Vec::new());
            available.push(0);
        }

        let library: Vec<Food> = FOOD_LIB
            .clone()
            .into_iter()
            .filter(|x| x.on_own())
            .collect();

        SuggestionEngine {
            library: library,
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
        let mut subfoods = food.breakdown();
        // Remvoe the food we are adding
        subfoods.remove(0);
        let foods: Vec<Food> = subfoods.into_iter().filter(|x| x.is_recipe()).collect();

        let mut avail = self.available.clone();
        let mut max_x = day + 1;

        if avail[day] < food.get_prep_time() {
            return false;
        } else {
            avail[day] -= food.get_prep_time();
            for f in foods {
                let mut placed = false;
                let mut found = false;
                for x in 0..max_x {
                    if self.prep[x].contains(&f) {
                        found = true;
                        placed = true;
                        break;
                    }
                }

                if !found {
                    // Try to place the food on the eairlieset day
                    for x in 0..max_x {
                        if placed {
                            break;
                        } else {
                        }
                        if avail[x] >= f.get_prep_time() {
                            placed = true;
                            avail[x] -= f.get_prep_time();
                        } else {
                        }
                    }
                } else {
                }

                if !placed {
                    return false;
                } else {
                }
            }
        }

        return true;
    }

    fn add_food(&mut self, food: &Food, day: i32) {
        let day = day as usize;
        let mut subfoods = food.breakdown();
        // Remvoe the food we are adding
        subfoods.remove(0);
        let foods: Vec<Food> = subfoods.into_iter().filter(|x| x.is_recipe()).collect();

        let avail = &mut self.available;
        let prep = &mut self.prep;
        let mut max_x = day + 1;

        avail[day] -= food.get_prep_time();
        prep[day].push(food.clone());
        for f in foods {
            let mut found = false;
            for x in 0..max_x {
                if prep[x].contains(&f) {
                    found = true;
                    break;
                }
            }

            if !found {
                let mut placed = false;
                // Try to place the food on the eairlieset day
                for x in 0..max_x {
                    if placed {
                        break;
                    } else {
                    }
                    if avail[x] >= f.get_prep_time() {
                        avail[x] -= f.get_prep_time();
                        prep[x].push(f.clone());
                        placed = true;
                    } else {
                    }
                }
            } else {
            }
        }
    }

    pub fn fill_with_food(&mut self) {
        let mut rand = rand::thread_rng();
        let mut my_lib = self.library.clone();
        for x in (0..7).rev() {
            let mut fooded = false;
            for _ in (0..10) {
                let trial_food = rand.choose(&my_lib).unwrap();
                if trial_food.get_name() == "Starve" {
                    continue;
                } else {
                }
                if self.can_add_food(trial_food, x) {
                    self.add_food(trial_food, x);
                    fooded = true;
                    break;
                } else {
                }
            }
            if !fooded {
                self.add_food(&FOOD_LIB[21], x);
            } else {
            }
        }
    }

    pub fn dishify(&self) -> Vec<Vec<Dish>> {
        let mut vec = Vec::new();
        for x in (0..7) {
            let mut ivec = Vec::new();
            let mut i = 0;
            for y in &self.prep[x] {
                ivec.push(y.to_dish(i));
                i += 1;
            }
            vec.push(ivec);
        }

        vec
    }
}
