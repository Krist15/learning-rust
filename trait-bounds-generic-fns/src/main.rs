#![allow(dead_code, unused_variables)]

trait AsJson {
    fn as_json(&self) -> String;
}

fn send_data_as_json<T: AsJson>(value: &T) {
    println!("Sending JSON data to server...");
    println!("-> {}", value.as_json());
    println!("Done!\n");
}

struct Person {
    name: String,
    age: u8,
    favorite_fruit: String,
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool,
}

struct Cat {
    name: String,
    sharp_claws: bool,
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
            self.name, self.age, self.favorite_fruit
        )
    }
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
        format!(
            r#"{{"type": "dog", "name": "{}", "age": "{}", "favoriteFruit": "{}"}}"#,
            self.name, self.color, self.likes_petting
        )
    }
}

fn main() {
    let laura = Person {
        name: String::from("Laura"),
        age: 20,
        favorite_fruit: String::from("apples"),
    };

    let firulais = Dog {
        name: String::from("Firulais"),
        color: String::from("Black"),
        likes_petting: true,
    };

    let kitty = Cat {
        name: String::from("Kitty"),
        sharp_claws: true,
    };

    send_data_as_json(&laura);
    send_data_as_json(&firulais);
    // println!("{}", firulais.as_json());

    //The Cat type does not implement the trait AsJson
    // send_data_as_json(&kitty);
}
