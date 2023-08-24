use reqwest::Client;
use tokio; // Import the tokio runtime
use serde_json::{from_str, Value};


#[derive(Debug)]

enum Book {
    Papery, Electronic
}

trait Animal {
    fn sound(&self) -> &'static str;
}

struct Dog;
impl Animal for Dog {
    fn sound(&self) -> &'static str {
        "Woof!"
    }
}

struct Cat;
impl Animal for Cat {
    fn sound(&self) -> &'static str {
        "Meow!"
    }
}

fn get_animal_sound(is_dog: bool) -> Box<dyn Animal> {
    if is_dog {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}



trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn get_shape(is_circle: bool) -> Box<dyn Shape> {
    if is_circle {
        Box::new(Circle { radius: 5.0 })
    } else {
        Box::new(Rectangle { width: 4.0, height: 3.0 })
    }
}



fn add<T>(a: T, b: T) -> T
    where
        T: std::ops::Add<Output = T>,
{
    a + b
}


fn largest<T>(list: &[T]) -> T where
    T: std::cmp::PartialOrd,  // 添加约束，指定 T 需要实现 PartialOrd trait
    T: Copy, {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client.get("http://allinone.sim.devops.com/api/v2/monitor/monitor?page=1&pagesize=10")
        .send()
        .await?; // Use await to await the Future's completion and handle its error using ?

    if response.status().is_success() {
        let json: Value = from_str(&response.text().await?).unwrap();
        println!("{}", json);
        // let value = json["data"];
        println!("Value: \n\n\n{}", json.get("data").unwrap().get("count").unwrap());
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    let dog_sound = get_animal_sound(true);
    let cat_sound = get_animal_sound(false);

    println!("Dog says: {}", dog_sound.sound());
    println!("Cat says: {}", cat_sound.sound());


    let circle = get_shape(true);
    let rectangle = get_shape(false);

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());

    let mut i = 0;
    while i < 10 {
        i += 1;
        println!("{}",i);
    }
    let book = Book::Papery;
    println!("{:?}",book);

    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);


    Ok(())

}