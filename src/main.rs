mod observer {
    pub trait Observer<T> {
        fn update(&mut self, value: &T);
        fn name(&self) -> String;
    }
    pub trait Subject<T> {
        fn register(&mut self, observer: Box<Observer<T>>) -> String;
        fn remove(&mut self, name: String);
        fn notify(&mut self, record: T);
    }
}

mod weather {
    #[derive(Copy, Clone)]
    pub struct WeatherRecord {
        pub temperature: i32,
        pub humidity: i32,
        pub pressure: i32,
    }
    impl WeatherRecord {
        pub fn new() -> WeatherRecord {
            WeatherRecord {
                temperature: 0,
                humidity: 0,
                pressure: 0,
            }
        }
    }

    use observer::{Observer, Subject};
    use std::collections::HashMap;

    pub struct WeatherData {
        observers: HashMap<String, Box<Observer<WeatherRecord>>>,
    }
    impl WeatherData {
        pub fn new() -> Self {
            WeatherData { observers: HashMap::new() }
        }
        fn get_temperature(&self) -> i32 {
            0
        }
        fn get_humidity(&self) -> i32 {
            1
        }
        fn get_pressure(&self) -> i32 {
            2
        }
        pub fn measurements_changed(&mut self) {
            let record = WeatherRecord {
                temperature: self.get_temperature(),
                humidity: self.get_humidity(),
                pressure: self.get_pressure(),
            };
            self.notify(record);
        }
    }
    impl Subject<WeatherRecord> for WeatherData {
        fn register(&mut self, observer: Box<Observer<WeatherRecord>>) -> String {
            let name = observer.name();
            self.observers.insert(name.clone(), observer);
            return name;
        }
        fn remove(&mut self, name: String) {
            self.observers.remove(&name);
        }
        fn notify(&mut self, record: WeatherRecord) {
            for (_, observer) in self.observers.iter_mut() {
                observer.update(&record);
            }
        }
    }
}

mod widget {

    use weather::WeatherRecord;
    use observer::Observer;

    pub trait DisplayWidget {
        fn display(&self);
    }

    pub struct WidgetCurrent {
        name: String,
        current: WeatherRecord,
    }
    impl WidgetCurrent {
        pub fn new<Name: Into<String>>(name: Name) -> WidgetCurrent {
            WidgetCurrent {
                name: name.into(),
                current: WeatherRecord::new(),
            }
        }
    }
    impl Observer<WeatherRecord> for WidgetCurrent {
        fn update(&mut self, record: &WeatherRecord) {
            self.current = *record;
            self.display();
        }
        fn name(&self) -> String {
            self.name.clone()
        }
    }
    impl DisplayWidget for WidgetCurrent {
        fn display(&self) {
            println!("{}", &self.name);
            println!("Temp: {}  Humid: {} Press: {}",
                     &self.current.temperature,
                     &self.current.humidity,
                     &self.current.pressure);
        }
    }

    /// **************************************************

    pub struct WidgetStatistic {
        name: String,
        history_temp: Vec<i32>,
        history_humid: Vec<i32>,
        history_press: Vec<i32>,
    }
    impl WidgetStatistic {
        pub fn new<Name: Into<String>>(name: Name) -> WidgetStatistic {
            WidgetStatistic {
                name: name.into(),
                history_temp: Vec::new(),
                history_humid: Vec::new(),
                history_press: Vec::new(),
            }
        }
    }
    impl Observer<WeatherRecord> for WidgetStatistic {
        fn update(&mut self, record: &WeatherRecord) {
            self.history_temp.push(record.temperature);
            self.history_humid.push(record.humidity);
            self.history_press.push(record.pressure);
            self.display();
        }
        fn name(&self) -> String {
            self.name.clone()
        }
    }
    impl DisplayWidget for WidgetStatistic {
        fn display(&self) {
            println!("{}", &self.name);
            println!("Minimal Temp: {} Minimal Humid: {} Minimal Press: {}",
                     &self.history_temp.iter().min().unwrap(),
                     &self.history_humid.iter().min().unwrap(),
                     &self.history_press.iter().min().unwrap());
            println!("Maximal Temp: {} Minimal Humid: {} Minimal Press: {}",
                     &self.history_temp.iter().max().unwrap(),
                     &self.history_humid.iter().max().unwrap(),
                     &self.history_press.iter().max().unwrap());
        }
    }

}

use widget::*;
use weather::WeatherData;
use observer::Subject;
fn main() {

    let mut weather = WeatherData::new();
    let mut registred = Vec::new();
    registred.push(weather.register(Box::new(WidgetCurrent::new("Current Widget"))));
    registred.push(weather.register(Box::new(WidgetStatistic::new("Statistic Widget"))));
    weather.measurements_changed();

}
