// derive is a macro that can be applied to enums and structs (adds additional functionality)
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Copy, Clone)] // all fields in struct also should derive
struct Employee {
    position: Position,
    work_hours: u64,
}

fn main() {
    let me = Employee {
        position: Position::Supervisor,
        work_hours: 30,
    };

    // no move occurs
    print_employee(me);
    // :? - debug print token
    println!("{:?}", me.position);
    println!("{:?}", me);
}

fn print_employee(employee: Employee) {
    println!("{:?}", employee);
}
// ----------------------------
// #[derive(Debug)]
// struct Person {
//     age: u8,
//     name: String,
//     fav_color: String,
// }

// impl Person {
//     fn print_fav_color(&self) {
//         println!("{}", self.fav_color)
//     }

//     fn print_name(&self) {
//         println!("{}", self.name)
//     }
// }

// fn main() {
//     let people = vec![
//         Person {
//             name: "Bob".to_owned(),
//             age: 20,
//             fav_color: String::from("Red"),
//         },
//         Person {
//             name: "Alice".to_owned(),
//             age: 40,
//             fav_color: String::from("Green"),
//         },
//         Person {
//             name: "Eve".to_owned(),
//             age: 33,
//             fav_color: String::from("Blue"),
//         },
//     ];

//     for peep in &people {
//         match peep.age {
//             x if x <= 33 => {
//                 peep.print_name();
//                 peep.print_fav_color()
//             }
//             _ => {}
//         }
//     }
// }

// --------------------------------------
// fn main() {
//     let v = vec![10, 20, 30, 40];

//     for el in &v {
//         match el {
//             30 => println!("thrity"),
//             _ => println!("{el}"),
//         }
//     }

//     println!("{}", v.len());
// }
// #[derive(Debug)]
// enum Colors {
//     Red,
//     Green,
//     Blue,
// }

// struct Dimensions {
//     width: f64,
//     height: f64,
// }

// impl Dimensions {
//     fn print(&self) {
//         println!("{} height", self.height);
//         println!("{} width", self.width);
//     }
// }

// struct Color {
//     color: Colors,
// }

// impl Color {
//     fn print(&self) {
//         match self.color {
//             Colors::Blue => println!("blue color"),
//             Colors::Red => println!("red color"),
//             Colors::Green => println!("green color"),
//         }
//     }
// }

// struct ShippingBox {
//     weight: f64,
//     color: Color,
//     dimensions: Dimensions,
// }

// impl ShippingBox {
//     fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
//         Self {
//             weight,
//             color,
//             dimensions,
//         }
//     }

//     fn print(&self) {
//         println!("{} weight", self.weight);
//         self.color.print();
//         self.dimensions.print();
//     }
// }

// fn main() {
//     let shipping_box = ShippingBox {
//         weight: 30.3,
//         color: Color {
//             color: Colors::Blue,
//         },
//         dimensions: Dimensions {
//             width: 30.0,
//             height: 50.1,
//         },
//     };

//     shipping_box.print();

//     let new_box = ShippingBox::new(
//         100.0,
//         Color {
//             color: Colors::Green,
//         },
//         Dimensions {
//             width: 50.0,
//             height: 70.0,
//         },
//     );

//     new_box.print();
// }
// ---------------------------------
// #[derive(Debug)]
// enum Flavors {
//     Apple,
//     Orange,
//     Banana,
// }
// struct Drink {
//     id: u32,
//     flavor: Flavors,
//     ounce: u32,
// }

// fn display_qnt(drink: &Drink) {
//     println!("{}", drink.ounce);
// }

// fn display_id(drink: Drink) -> Drink {
//     println!("{}", drink.id);

//     return drink;
// }

// struct Temperature {
//     degrees_f: f64,
// }

// impl Temperature {
//     // Self refers to a struct itself, we can also use struct name here instead of Self
//     fn freeze() -> Self {
//         Self { degrees_f: 32.2 }
//     }

//     // taking a reference to self
//     fn show_temp(temp: &Temperature) {
//         println!("{}", temp.degrees_f);
//     }

//     fn show_temp_self(&self) {
//         println!("{}", self.degrees_f);
//     }
// }

// fn main() {
//     let temp = Temperature { degrees_f: 100.1 };

//     // show_temp is kinda like static method here (::)
//     Temperature::show_temp(&temp);

//     temp.show_temp_self();

//     let cold = Temperature::freeze();
//     // self is implied when we use . notation
//     cold.show_temp_self();

// let mut drink = Drink {
//     id: 1,
//     flavor: Flavors::Banana,
//     ounce: 11,
// };

// drink.id = 2;

// drink = display_id(drink);
// display_qnt(&drink);

// println!("{:#?}", drink.flavor);

// let is_tasty = match drink.flavor {
//     Flavors::Apple => false,
//     Flavors::Banana => true,
//     Flavors::Orange => true,
// };

// println!("{is_tasty}");

// let is_enough = match drink.ounce {
//     0..=10 => false,
//     _ => true,
// };

// println!("{is_enough}");

// let tup = (1, "bob", 3);

// println!("{:?}", tup.0);

// println!("{:?}", get_drink_info(drink));

// let coord = (1, 2);

// let (x, y) = coord;

// println!("{x},{y}");

// if 1 < x || x > 2 {
//     print!("yes!");
// } else {
//     print!("no!");
// }

// let is_more_than_100 = if x > 100 { true } else { false };
// println!("{is_more_than_100}");
// }

// fn get_drink_info(drink: Drink) -> (Flavors, u32) {
//     let (flavor, ounce) = (drink.flavor, drink.ounce);

//     (flavor, ounce)
// }