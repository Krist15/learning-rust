// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}


// Declare enum for Car transmission type
#[derive(PartialEq, Debug)]
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}


fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car = Car {color, transmission, convertible, mileage: 0};
    car
}
    
fn main() {

        let mut car = car_factory(String::from("Black"), Transmission::Automatic, true);
    println!("Car 1 = {}, {:?} transmission, convertible {:?}, mileage {:?}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("White"), Transmission::SemiAuto, false);
    println!("Car 2 = {}, {:?} transmission, convertible {:?}, mileage {:?}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Blue"), Transmission::Manual, false);
    println!("Car 3 = {}, {:?} transmission, convertible {:?}, mileage {:?}", car.color, car.transmission, car.convertible, car.mileage)
}