extern crate lazy_static;

use self::lazy_static::*;

use food::*;

pub fn gen_library() -> Vec<Food> {
    let mut lib = Vec::new();

    // 0
    let spag_noodles =
        Ingredient::new("Spaghetti Noodles", GRAMS).to_food(Unit::Grams(75.0), false);
    lib.push(spag_noodles.clone());

    // 1
    let spaghetti = Recipe::new("Spaghetti")
        .add_component(&spag_noodles)
        .add_step("Boil noodles in water for 15 miniutes")
        .set_produces(Unit::Grams(75.0))
        .set_takes(15)
        .make_recipe();
    lib.push(spaghetti.clone());

    // 2
    let fett_noodles =
        Ingredient::new("Fettuccine Noodles", GRAMS).to_food(Unit::Grams(75.0), false);
    lib.push(fett_noodles.clone());

    // 3
    let fettuccine = Recipe::new("Fettuccine")
        .add_component(&fett_noodles)
        .add_step("Boil noodles in water for 15 miniutes")
        .set_produces(Unit::Grams(75.0))
        .set_takes(15)
        .make_recipe();
    lib.push(fettuccine.clone());

    // 4
    let spa_sauce = Ingredient::new("Spaghetti Sauce", LITERS).to_food(Unit::Liters(0.25), false);
    lib.push(spa_sauce.clone());

    // 5
    let spaghetti_sauced = Recipe::new("Spaghetti with sauce")
        .add_component(&spaghetti)
        .add_component(&spa_sauce)
        .add_step("Simmer spaghetti in sauce for 10 miniutes")
        .set_produces(Unit::Grams(100.0))
        .set_takes(10)
        .make_recipe();
    lib.push(spaghetti_sauced.clone());

    // 6
    let alf_sauce = Ingredient::new("Alfredo Sauce", LITERS).to_food(Unit::Liters(0.25), false);
    lib.push(alf_sauce.clone());

    // 7
    let alfredo = Recipe::new("Fettuccine Alfredo")
        .add_component(&fettuccine)
        .add_component(&alf_sauce)
        .add_step("Simmer noodles in alfredo sauce for 5 miniutes")
        .set_produces(Unit::Grams(100.0))
        .set_takes(5)
        .make_recipe();
    lib.push(alfredo.clone());

    // 8
    let tomato = Ingredient::new("Tomato", AMMOUNT).to_food(Unit::Ammount(1.0), true);
    lib.push(tomato.clone());

    // 9
    let ground_beef = Ingredient::new("Ground Beef", GRAMS).to_food(Unit::Grams(100.0), false);
    lib.push(ground_beef.clone());

    // 10
    let burger_bun = Ingredient::new("Burger Bun", AMMOUNT).to_food(Unit::Ammount(1.0), true);
    lib.push(burger_bun.clone());

    // 11
    let cheese_slice =
        Ingredient::new("Single Cheese Slice", AMMOUNT).to_food(Unit::Ammount(1.0), true);
    lib.push(cheese_slice.clone());

    // 12
    let burger = Recipe::new("Cheese Burger")
        .add_component(&ground_beef)
        .add_component(&burger_bun)
        .add_component(&cheese_slice)
        .add_component(&tomato)
        .add_step("Form ground beef into patty")
        .add_step("Butter pan")
        .add_step("Fry patty for 10 miniutes")
        .add_step("Clean and rebutter pan")
        .add_step("Apply cheese slice to patty")
        .add_step("Place cheesed patty on bun")
        .add_step("Slice tomato")
        .add_step("Apply tomato to burger")
        .set_takes(40)
        .make_recipe();
    lib.push(burger.clone());

    // 13
    let cheese_sandwich = Recipe::new("Cheese Sandwich")
        .add_component(&burger_bun)
        .add_component(&cheese_slice)
        .add_step("Add cheese to bun")
        .set_takes(1)
        .make_recipe();
    lib.push(cheese_sandwich.clone());

    lib
}

lazy_static! {
    pub static ref FOOD_LIB: Vec<Food> = gen_library();
}
