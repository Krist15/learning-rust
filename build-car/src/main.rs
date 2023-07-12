#[derive(Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    roof: bool,
    age: (Age, u32),
}


#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}
#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_quality (miles: u32) -> (Age, u32) {
    let quality = (Age::New, miles + 0);
    quality 
}


fn car_factory(color: String, transmission: Transmission, roof: bool, age: (Age, u32)) -> Car {
    let car = Car {color, transmission, roof, age};
    car
}
    
fn main() {
    let colors: [&str; 4] = ["Blue","Green","Red","Silver"];

    let mut car: Car = Car {color: String::from(colors[0]), transmission: Transmission::Automatic, roof: false, age: car_quality(0)};
    println!("{:?}", car);

    let mut engine: Transmission = Transmission::Manual;
    car = car_factory(String::from(colors[1]), engine, false, car_quality(200));
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.transmission, car.color, car.age.1);

    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[2]), engine, false, car_quality(0));
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.transmission, car.color, car.age.1);

    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[3]), engine, true, car_quality(100));
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.transmission, car.color, car.age.1);
}