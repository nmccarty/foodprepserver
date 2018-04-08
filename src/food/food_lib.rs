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

    // 14
    let soy_sauce = Ingredient::new("Soy Sauce", LITERS).to_food(Unit::Liters(0.025), false);
    lib.push(soy_sauce.clone());

    // 15
    let oil = Ingredient::new("Oil", LITERS).to_food(Unit::Liters(0.025), false);
    lib.push(oil.clone());

    // 16
    let honey = Ingredient::new("Honey", LITERS).to_food(Unit::Liters(0.025), false);
    lib.push(honey.clone());

    // 17
    let garlic = Ingredient::new("Garlic", AMMOUNT).to_food(Unit::Ammount(1.0), false);
    lib.push(garlic.clone());

    // 18
    let ginger = Ingredient::new("Ginger", GRAMS).to_food(Unit::Grams(0.025), false);
    lib.push(ginger.clone());

    // 19
    let chicken = Ingredient::new("Chicken", GRAMS).to_food(Unit::Grams(0.025), false);
    lib.push(chicken.clone());

    // 20
    let teriyaki_chicken = Recipe::new("Teriyaki Chicken")
        .add_component(&soy_sauce)
        .add_component(&oil)
        .add_component(&honey)
        .add_component(&garlic)
        .add_component(&ginger)
        .add_component(&chicken)
        .add_step("Mix soy sauch, vinegar, oil, honey, garlic, ginger, and cornstarch")
        .add_step("Season chicken and cook until almost cooked through")
        .add_step("Add chicken to sauce")
        .add_step("Simmer until sauce has thickened")
        .add_step("Serve")
        .set_takes(20)
        .make_recipe();
    lib.push(teriyaki_chicken.clone());

    lib
}

lazy_static! {
    pub static ref FOOD_LIB: Vec<Food> = gen_library();
}
