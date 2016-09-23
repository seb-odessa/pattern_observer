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

mod data {
    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};
    pub struct DataGen {
        base: i32,
        rgen: rand::ThreadRng,
        rang: rand::distributions::Range<i32>,
    }
    impl DataGen {
        pub fn new(base: i32, delta: i32) -> Self {
            let rgen = rand::thread_rng();
            let rang = Range::new(0, delta);
            DataGen {
                base: base,
                rgen: rgen,
                rang: rang,
            }
        }
    }
    impl Iterator for DataGen {
        type Item = i32;
        fn next(&mut self) -> Option<i32> {
            let value = self.base + self.rang.ind_sample(&mut self.rgen);
            return Some(value);
        }
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
                temperature: 0,
                humidity: 0,
                pressure: 0,
            }
        }
    }

    use data::DataGen;
    use observer::{Observer, Observable};
    use std::collections::HashMap;

    pub struct WeatherData {
        temperature: DataGen,
        humidity: DataGen,
        pressure: DataGen,
        observers: HashMap<String, Box<Observer<WeatherRecord>>>,
    }
    impl WeatherData {
        pub fn new() -> Self {
            WeatherData {
                temperature: DataGen::new(10, 10),
                humidity: DataGen::new(40, 60),
                pressure: DataGen::new(700, 90),
                observers: HashMap::new(),
            }
        }
        fn get_temperature(&mut self) -> Temperature {
            self.temperature.next().unwrap()
        }
        fn get_humidity(&mut self) -> Humidity {
            self.humidity.next().unwrap()
        }
        fn get_pressure(&mut self) -> Pressure {
            self.pressure.next().unwrap()
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
            for (_, observer) in self.observers.iter_mut() {
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
    use std::ops::AddAssign;
    pub struct WidgetStatistic {
        name: String,
        history_length: usize,
        history_temp: LinkedList<Temperature>,
        history_humid: LinkedList<Humidity>,
        history_press: LinkedList<Pressure>,
    }
    impl WidgetStatistic {
        pub fn new<Name: Into<String>>(name: Name) -> WidgetStatistic {
            WidgetStatistic {
                name: name.into(),
                history_length: 10,
                history_temp: LinkedList::new(),
                history_humid: LinkedList::new(),
                history_press: LinkedList::new(),
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
        fn statistic<T: Copy + Ord + AddAssign>(list: &LinkedList<T>) -> (T, T, T) {
            let first = list.front().unwrap();
            let mut min: T = first.clone();
            let mut max: T = first.clone();
            let mut sum: T = first.clone();
            for record in list.into_iter().skip(1) {
                let curr: T = record.clone();
                if min > curr {
                    min = curr
                }
                if max < curr {
                    max = curr
                }
                sum += curr;
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

            let (min, max, sum) = WidgetStatistic::statistic(&self.history_temp);
            let avg: f32 = sum as f32 / self.history_temp.len() as f32;
            println!("\tTemperature (min/max/avg)\t: {} / {} / {}", min, max, avg);

            let (min, max, sum) = WidgetStatistic::statistic(&self.history_humid);
            let avg: f32 = sum as f32 / self.history_humid.len() as f32;
            println!("\tHumidity (min/max/avg) \t\t: {} / {} / {}", min, max, avg);

            let (min, max, sum) = WidgetStatistic::statistic(&self.history_press);
            let avg: f32 = sum as f32 / self.history_humid.len() as f32;
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

    for _ in 0..10 {
        weather.measurements_changed();
    }

}
