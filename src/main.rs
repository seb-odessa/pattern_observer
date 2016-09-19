mod observer {
    pub trait Observer<T> {
        fn update(&mut self, value: &T);
        fn name(&self) -> String;
    }
    pub trait Observable<T> {
        fn register(&mut self, observer: Box<Observer<T>>) -> String;
        fn remove(&mut self, name: String);
        fn notify(&mut self, record: T);
    }
}

mod weather {

    pub type Temperature = i32;
    pub type Humidity = i32;
    pub type Pressure = i32;

    #[derive(Copy, Clone)]
    pub struct WeatherRecord {
        pub temperature: Temperature,
        pub humidity: Humidity,
        pub pressure: Pressure,
    }
    impl WeatherRecord {
        pub fn new() -> WeatherRecord {
            WeatherRecord {
                temperature : 0,
                humidity    : 0,
                pressure    : 0,
            }
        }
    }

    struct DataGen<T> {
        base : T
    }

    use observer::{Observer, Observable};
    use std::collections::HashMap;

    pub struct WeatherData {
        observers: HashMap<String, Box<Observer<WeatherRecord>>>,
    }
    impl WeatherData {
        pub fn new() -> Self {
            WeatherData { observers: HashMap::new() }
        }
        fn get_temperature(&self) -> Temperature {
            0
        }
        fn get_humidity(&self) -> Humidity {
            1
        }
        fn get_pressure(&self) -> Pressure {
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
    impl Observable<WeatherRecord> for WeatherData {
        fn register(&mut self, observer: Box<Observer<WeatherRecord>>) -> String {
            let name = observer.name();
            self.observers.insert(name.clone(), observer);
            return name;
        }
        fn remove(&mut self, name: String) {
            self.observers.remove(&name);
        }
        fn notify(&mut self, record: WeatherRecord) {
            for (name, observer) in self.observers.iter_mut() {
                println!("Notify '{}'", name);
                observer.update(&record);
            }
        }
    }
}

mod widget {
    use weather::{WeatherRecord, Temperature, Humidity, Pressure};
    use observer::Observer;

    pub trait DisplayWidget {
        fn display(&self);
    }

    /// ********************* WidgetCurrent *****************************
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
            println!("\tTemperature\t: {}\n\tHumid\t\t: {}\n\tPress\t\t: {}",
                     &self.current.temperature,
                     &self.current.humidity,
                     &self.current.pressure);
        }
    }

    /// ********************* WidgetStatistic *****************************
    use std::collections::LinkedList;
    use std::ops::{AddAssign};
    pub struct WidgetStatistic {
        name            : String,
        history_length  : usize,
        history_temp    : LinkedList<Temperature>,
        history_humid   : LinkedList<Humidity>,
        history_press   : LinkedList<Pressure>,
    }
    impl WidgetStatistic {
        pub fn new<Name: Into<String>>(name: Name) -> WidgetStatistic {
            WidgetStatistic {
                name            : name.into(),
                history_length  : 10,
                history_temp    : LinkedList::new(),
                history_humid   : LinkedList::new(),
                history_press   : LinkedList::new(),
            }
        }
        fn strip_list(&mut self) {
            if self.history_temp.len() >= self.history_length {
                self.history_temp.pop_front();
            }
            if self.history_humid.len() >= self.history_length {
                self.history_humid.pop_front();
            }
            if self.history_press.len() >= self.history_length {
                self.history_press.pop_front();
            }
        }
        //
        fn statistic<T : Copy+Ord+AddAssign+Div>(list :&LinkedList<T>) -> (T, T, T) {
            let first = list.front().unwrap();
            let mut min : T = first.clone();
            let mut max : T = first.clone();
            let mut sum : T = first.clone();
            for record in list.into_iter().skip(1) {
                let curr : T = record.clone();
                if min > curr { min = curr }
                if max < curr { max = curr }
                sum +=  curr;
            }
            return (min, max, sum);
        }
    }
    impl Observer<WeatherRecord> for WidgetStatistic {
        fn update(&mut self, record: &WeatherRecord) {
            self.history_temp.push_back(record.temperature);
            self.history_humid.push_back(record.humidity);
            self.history_press.push_back(record.pressure);
            self.strip_list();
            self.display();
        }
        fn name(&self) -> String {
            self.name.clone()
        }
    }
    impl DisplayWidget for WidgetStatistic {
        fn display(&self) {
            println!("{}", &self.name);

            let (min,max,sum) = WidgetStatistic::statistic(&self.history_temp);
            let avg : f32 = sum as f32 / self.history_temp.len() as f32;
            println!("\tTemperature (min/max/avg)\t: {} / {} / {}", min, max, avg);

            let (min,max,sum) = WidgetStatistic::statistic(&self.history_humid);
            let avg : f32 = sum as f32 / self.history_humid.len() as f32;
            println!("\tHumidity (min/max/avg) \t\t: {} / {} / {}", min, max, avg);

            let (min,max,sum) = WidgetStatistic::statistic(&self.history_press);
            let avg : f32 = sum as f32 / self.history_humid.len() as f32;
            println!("\tPressure (min/max/avg) \t\t: {} / {} / {}", min, max, avg);
        }
    }
}

use widget::*;
use weather::WeatherData;
use observer::Observable;
fn main() {

    let mut weather = WeatherData::new();
    let mut registred = Vec::new();
    registred.push(weather.register(Box::new(WidgetCurrent::new("Current Widget"))));
    registred.push(weather.register(Box::new(WidgetStatistic::new("Statistic Widget"))));
    weather.measurements_changed();

}
