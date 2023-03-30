mod builders;
mod cars;
mod components;
mod director;

use builders::{Builder, CarBuilder, CarManualBuilder};
use cars::{Car, Manual};
use director::Director;

fn main() {
    let mut car_builder = CarBuilder::default();

    // Director gets the concrete builder object from client
    Director::construct_sports_car(&mut car_builder);

    let car: Car = car_builder.build();
    println!("Car built: {:?}\n", car.car_type());

    let mut manual_builder = CarManualBuilder::default();

    Director::construct_city_car(&mut manual_builder);

    let manual: Manual = manual_builder.build();
    println!("Car manual built: \n{}", manual);
}
