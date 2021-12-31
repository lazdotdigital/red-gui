use red_gui::{run, Red, WindowConfig};
use std::collections::HashMap;

struct App {
    count: i32,
    name: String,
}

impl Red for App {
    fn button(&mut self, event: &str) {
        match event {
            "increase-count" => self.count += 1,
            "decrease-count" => self.count -= 1,
            _ => unimplemented!(),
        }
    }

    fn input(&mut self, id: &str, value: &str) {
        match id {
            "name" => self.name = value.to_owned(),
            _ => unimplemented!(),
        }
    }

    fn values(&self) -> HashMap<&'static str, String> {
        let mut map = HashMap::new();

        map.insert("name", self.name.to_uppercase().clone());
        map.insert("count", self.count.to_string());

        map
    }
}

fn main() {
    let config = WindowConfig {
        title: "Red Example",
        size: (800, 600),
        resizable: true,
        frameless: false,
        debug: true,
    };

    let app = App {
        count: 0,
        name: "Laz".to_owned(),
    };

    run(app, config, include_str!("example.html")).expect("couldn't start");
}
