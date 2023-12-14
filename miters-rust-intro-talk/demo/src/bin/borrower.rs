#[derive(Debug)]
struct Car {
    owner: String,
    color: String,
}

fn borrow_car_to_ivan(car: &mut Car) {
    car.owner = String::from("Ivan");
    println!("Car is belong to {} now.", car.owner);
}

fn paint_car(car: &mut Car) {
    car.color = String::from("blue");
    println!("Car is now in {}", car.color);
}

fn main() {
    let mut my_car = Car {
        color: String::from("red"),
        owner: String::from("Jeff"),
    };

    println!("Ca is belong to {}", my_car.owner);
    borrow_car_to_ivan(&mut my_car);
    paint_car(&mut my_car);

    println!(
        "Car is belong to {} and in color {}.",
        my_car.owner, my_car.color
    )
}
