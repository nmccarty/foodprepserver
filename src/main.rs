pub mod food;
extern crate hyper;
#[macro_use]
extern crate rustful;
extern crate serde;
extern crate serde_json;
use rustful::{Context, Handler, Response, Server, TreeRouter};
use rustful::context::body::BodyReader;
use std::io::Read;
use hyper::header::{AccessControlAllowOrigin, Headers};

use serde_json::{Error, Value};

use food::*;
use food::food_lib::FOOD_LIB;
use food::engine::*;

struct FoodPlan;

impl Handler for FoodPlan {
    fn handle_request(&self, mut context: Context, response: Response) {
        let mut string = String::new();
        context.body.read_to_string(&mut string);

        let mut day_times = Vec::new();
        let mut day_avails = Vec::new();

        for i in (0..7) {
            day_times.push(9);
            day_avails.push(0);
        }

        let json_value: Value = serde_json::from_str(&string).ok().unwrap();
        let values = json_value.as_array().unwrap();

        for value in values {
            let day = value["day"].as_u64().unwrap() as usize;
            let time = value["time"].as_i64().unwrap() as i32;
            let start = value["start"].as_i64().unwrap() as i32;

            day_times[day] = start;
            day_avails[day] = time;
            println!("day: {} start: {} time: {}", day, start, time);
        }

        let mut engine = SuggestionEngine::new();

        for x in (0..7) {
            engine.add_avail(x, day_times[x as usize], day_avails[x as usize]);
        }

        engine.fill_with_food();
        let mut data = engine.dishify();

        for day in 0..7 {
            let mut day_time = day_times[day];
            let day_foods = &mut data[day];
            for f in day_foods {
                f.set_time(day_time);
               // day_time += 1;
            }
        }

        let mut response = response;

        let resp = serde_json::to_string_pretty(&data).ok().unwrap();

        addACA(&mut response);
        response.send(resp);
    }
}

fn addACA(response: &mut Response) {
    response.headers_mut().set(AccessControlAllowOrigin::Any);
}

fn main() {
    let my_router = insert_routes!{
        //Create a new TreeRouter
        TreeRouter::new() => {
            "foodplan" => {
                Get: FoodPlan,
                Post: FoodPlan,
            }
        }
    };

    Server {
        //Use a closure to handle requests.
        handlers: my_router,
        //Set the listening port to `8080`.
        host: 8080.into(),
        //Fill out everything else with default values.
        ..Server::default()
    }.run();
}
