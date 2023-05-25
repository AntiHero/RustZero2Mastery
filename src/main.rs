// lesson 77
// mod msg {
//     pub fn trim(msg: &str) -> &str {
//         msg.trim()
//     }

//     pub fn capitalize(msg: &str) -> String {
//         msg.to_uppercase()
//     }
// }
// fn main() {
// use msg::{capitalize, trim};

// let result = trim("    hello     ");
// println!("{result}");

//     let result = msg::capitalize(result);
//     println!("{result}");
// }
// lesson 76
// doesn't work in module
// use std::collections::HashMap;
// behaves like individual file, to use std we should directly define it in mod itself
// groups functionaltiy
// mod math {
//     use std::collections::HashMap;
//     pub fn add(a: i32, b: i32) -> i32 {
//         a + b
//     }
// }
// fn main() {
//     use math::*;
//     let res = add(2, 4);
//     println!("{res}");
//     let res = math::add(1, 2);
//     println!("{res}");
//     use math::add;
// }
// lesson 75
// fn main() {
//     let mut data = Some(3);

//     // suitable for working with iterators
//     while let Some(i) = data {
//         println!("looping loop {i}");
//         data = None;
//     }

//     println!("done");

//     let vector = vec![1, 2, 3, 4, 5];
//     let mut iterator = vector.iter();

//     while let Some(i) = iterator.next() {
//         println!("num {:?}", i);
//     }
// }
// lesson 74
// fn main() {
//     let maybe_user = Some("Jerry");

//     match maybe_user {
//         Some(user) => println!("{user}"),
//         _ => println!("Not a user"),
//     }

//     // is equivalent to (when we don't care about None value)

//     // helpts to handle Option type
//     if let Some(user) = maybe_user {
//         println!("{user}");
//     }
// }
// lesson 73
// fn main() {
//     let range = 1..9;

//     for i in range {
//         println!("{i}");
//     }

//     for ch in 'a'..'d' {
//         println!("{ch}");
//     }
// }
// lesson 72
// fn main() {
//     let data = vec![1, 2, 3, 4, 5];

//     let data = data
//         .iter()
//         .map(|num| num * 3)
//         .filter(|num| num > &10)
//         .collect::<Vec<i32>>();

//     for num in &data {
//         println!("{num}");
//     }
// }
// lesson 71
// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5];

//     let numbers = numbers
//         .iter()
//         .map(|num| num + 1)
//         .filter(|num| num & 1 == 0)
//         .collect::<Vec<i32>>();

//     let find_me = numbers.iter().find(|&num| num == &4);

//     match find_me {
//         Some(_) => println!("Found"),
//         None => println!("Not found"),
//     }

//     let slice = numbers.iter().take(3).collect::<Vec<&i32>>();

//     println!("{:?}", slice);
// }
// lesson 70
// enum Access {
//     Admin,
//     User,
//     Guest,
// }

// fn maybe_access(name: &str) -> Option<Access> {
//     match name {
//         "admin" => Some(Access::Admin),
//         "gary" => Some(Access::User),
//         _ => None,
//     }
// }

// fn root() -> Option<Access> {
//     Some(Access::Admin)
// }

// fn part_1() -> bool {
//     maybe_access("admin").is_some()
// }

// fn part_2() -> Option<Access> {
//     maybe_access("root").or_else(root)
// }

// fn part_3() -> Access {
//     maybe_access("Alice").unwrap_or_else(|| Access::Guest)
// }
// fn main() {}
// lesson 68
// #[derive(Debug)]
// struct User {
//     user_id: i32,
//     name: String,
// }

// fn find_user(name: &str) -> Option<i32> {
//     let name = name.to_lowercase();

//     match name.as_str() {
//         "sam" => Some(1),
//         "matt" => Some(5),
//         "katie" => Some(9),
//         _ => None,
//     }
// }

// fn create_user(user_name: &str) -> Option<User> {
//     find_user(user_name).map(|id| User {
//         user_id: id,
//         name: user_name.to_owned(),
//     })
// }
// fn main() {
//     let user = create_user("sam");

//     match user {
//         Some(user) => println!("{:?}", user),
//         None => println!("user not found"),
//     }

//     let user = create_user("bob");

//     match user {
//         Some(user) => println!("{:?}", user),
//         None => println!("user not found"),
//     }
// }
// lesson 66
// fn main() {
//     let add = |a, b| a + b;

//     let del = |a: i32, b: i32| -> i32 { a + b };
//     println!("{}", add(1, 2));
// }
// lesson 65
// use std::collections::HashMap;

// #[derive(Hash, PartialEq, Eq, Debug)]
// enum Furniture {
//     Chair,
//     Bed,
//     Table,
//     Couche,
// }

// struct Item {
//     amount: Option<u32>,
//     name: Furniture,
// }

// fn main() {
//     let mut store = HashMap::new();

//     let chair = Item {
//         name: Furniture::Chair,
//         amount: Some(5),
//     };
//     let bed = Item {
//         name: Furniture::Bed,
//         amount: Some(3),
//     };

//     let table = Item {
//         name: Furniture::Table,
//         amount: Some(2),
//     };

//     let couche = Item {
//         name: Furniture::Couche,
//         amount: None,
//     };

//     store.insert(chair.name, chair.amount);
//     store.insert(bed.name, bed.amount);
//     store.insert(table.name, table.amount);
//     store.insert(couche.name, couche.amount);

//     // iter here doesn't move
//     for (key, value) in store.iter() {
//         match value {
//             Some(x) => println!("Theare are {x} {:?} in the store", key),
//             None => println!("{:?} out of stock", key),
//         }
//     }
// }
// lesson 62
// enum EmployeeType {
//     Maintaince,
//     Marketing,
//     Manager,
//     LineSuperior,
//     Kitchen,
//     Assembly,
// }

// struct Employee {
//     employee_type: EmployeeType,
//     works: bool,
// }

// fn check_access(employee: &Employee) -> Result<bool, String> {
//     match employee.employee_type {
//         EmployeeType::Assembly | EmployeeType::Marketing | EmployeeType::Manager
//             if employee.works == true =>
//         {
//             Ok(true)
//         }
//         _ => Err("Forbidden".to_owned()),
//     }
// }

// fn main() -> Result<(), String> {
//     let empl1 = Employee {
//         employee_type: EmployeeType::LineSuperior,
//         works: false,
//     };

//     let empl2 = Employee {
//         employee_type: EmployeeType::Manager,
//         works: true,
//     };

//     let empl3 = Employee {
//         employee_type: EmployeeType::Kitchen,
//         works: true,
//     };

//     let result2 = check_access(&empl2)?;
//     println!("{result2}");
//     let result3 = check_access(&empl3)?;
//     println!("{result3}");
//     let result1 = check_access(&empl1)?;
//     println!("{result1}");

//     Ok(())
// }

// lesson 61
// struct Customer {
//     age: u32,
//     name: String,
// }

// fn check_purchuase(customer: &Customer) -> Result<bool, String> {
//     match customer.age {
//         age if age < 18 => Err(format!("{} is too young", age)),
//         age => Ok(true),
//     }
// }

// fn run_check(customer: &Customer) -> Result<(), String> {
//     let result = check_purchuase(&customer)?;
//     println!("{}", result);

//     Ok(())
// }

// fn main() {
//     let user1 = Customer {
//         name: String::from("Bob"),
//         age: 20,
//     };

//     let user2 = Customer {
//         name: String::from("Eve"),
//         age: 17,
//     };

//     // println!("{:?}", check_purchuase(&user1));
//     // println!("{:?}", check_purchuase(&user2));

//     run_check(&user1);
//     let error = run_check(&user2);
//     println!("{:?}", error);
// }

// lesson 59 ----------------

// fn get_sound(name: &str) -> Result<SoundData, String> {
//     if name == "alert" {
//         Ok(SoundData::new("alert"))
//     } else {
//         Err("unable to find sounda data".to_owned())
//     }
// }

// #[derive(Debug)]
// struct SoundData {
//     sound: String,
// }

// impl SoundData {
//     fn new(str: &str) -> Self {
//         SoundData {
//             sound: str.to_owned(),
//         }
//     }
// }

// fn main() {
//     let first_sound_result = get_sound("woof");
//     let second_sound_result = get_sound("alert");

//     println!("{:?}", first_sound_result);
//     println!("{:?}", second_sound_result);

//     let sound = get_sound("alert");

//     let sound_data = SoundData {
//         sound: "alert".to_owned(),
//     };

//     match sound {
//         Ok(sound_data) => println!("Sound data is alert"),
//         Err(e) => println!("{e}"),
//     }
// }

// lesson 58 ----------------

// fn main() {
//     print_lowercase("HELLO WORLD");
//     print_uppercase("hello world");
// }

// fn print_lowercase(str: &str) {
//     println!("{}", str.to_lowercase());
// }

// fn print_uppercase(str: &str) {
//     println!("{}", str.to_uppercase());
// }

// lesson 57 ----------------
// fn main() {
//     let vector: Vec<i32> = vec![];

//     match vector.is_empty() {
//         true => println!("is empty"),
//         false => println!("not empty"),
//     }
// }

// lesson 55-----------------
// struct Student {
//     name: String,
//     locker: Option<u32>,
// }

// /// Displays the message based on whether the student has a locker or not
// fn main() {
//     let students = vec![
//         Student {
//             name: "bob".to_owned(),
//             locker: None,
//         },
//         Student {
//             name: "alice".to_owned(),
//             locker: Some(1),
//         },
//         Student {
//             name: "eve".to_owned(),
//             locker: Some(15),
//         },
//     ];

//     for student in &students {
//         if !student.locker.is_none() {
//             match student.locker {
//                 Some(number) => println!("{} locker number is {}", student.name, number),
//                 None => (),
//             }
//         } else {
//             println!("{} has no locker!", student.name)
//         }
//     }
// }

// struct Customer {
//     age: Option<i32>,
//     email: String,
// }

// fn main() {
//     let bob = Customer {
//         age: Some(20),
//         email: "bob@gmail.com".to_owned(),
//     };

//     let alice = Customer {
//         age: None,
//         email: "alice@gmail.com".to_owned(),
//     };

//     match bob.age {
//         Some(18) => println!("Bob is 18!"),
//         Some(age) => println!("Bob is {}", age),
//         None => (),
//     }
// }

// lesson52-----------------------

// enum Ticket {
//     Backstage(f32, String),
//     Vip(f32, String),
//     Standard(f32),
// }

// fn main() {
//     let tickets = vec![
//         Ticket::Backstage(100.2, String::from("Bob")),
//         Ticket::Vip(50.9, String::from("Alice")),
//         Ticket::Standard(10.0),
//     ];

//     for ticket in &tickets {
//         match ticket {
//             Ticket::Backstage(price, name) => println!("{name} {price}"),
//             Ticket::Vip(price, name) => println!("{name} {price}"),
//             Ticket::Standard(price) => println!("{price}"),
//         }
//     }
// }

// lesson51-----------------------
// use std::any;

// #[derive(Debug)]
// enum Metal {
//     Bronze,
//     Silver,
//     Gold(i32),
// }

// fn main() {
//     let metal = Metal::Bronze;
//     let not_gold = Metal::Gold(0);
//     let gold = Metal::Gold(375);

//     match_color(metal);
//     match_color(not_gold);
//     match_color(gold);

//     let person = Person {
//         age: 10,
//         name: String::from("Bob"),
//     };

//     match_struct(&person);
// }

// fn match_color(metal: Metal) {
//     match metal {
//         Metal::Gold(375) => println!("pure gold!"),
//         Metal::Gold(_) => println!("gold!"),
//         any_metal => println!("{:?} not a gold", any_metal),
//     }
// }

// struct Person {
//     name: String,
//     age: u8,
// }

// fn match_struct(person: &Person) {
//     match person {
//         Person { age: 10, .. } => println!("{} is here!", person.name),
//         _ => (),
//     }
// }
// derive is a macro that can be applied to enums and structs (adds additional functionality)
// #[derive(Debug, Clone, Copy)]
// enum Position {
//     Manager,
//     Supervisor,
//     Worker,
// }

// #[derive(Debug, Copy, Clone)] // all fields in struct also should derive
// struct Employee {
//     position: Position,
//     work_hours: u64,
// }

// fn main() {
//     let me = Employee {
//         position: Position::Supervisor,
//         work_hours: 30,
//     };

//     // no move occurs
//     print_employee(me);
//     // :? - debug print token
//     println!("{:?}", me.position);
//     println!("{:?}", me);

//     let mouse_model = Mouse::Model(Roccat::Kova);
//     println!("{:?}", mouse_model);

//     let num = 3;
// }

// fn print_employee(employee: Employee) {
//     println!("{:?}", employee);
// }

// #[derive(Debug)]
// enum Mouse {
//     LeftClick,
//     RightClick,
//     Scroll(i32),
//     Move(i32, i32),
//     Model(Roccat),
// }
// #[derive(Debug)]
// enum Roccat {
//     Kova,
//     Nova,
// }

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
