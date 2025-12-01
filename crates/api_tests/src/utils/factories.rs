use chrono::{Datelike, Timelike, Utc};
use fake::{
    faker::{address::en::*, internet::en::*, name::en::*, phone_number::en::*},
    Fake,
};
use rand::Rng;
use std::time::Duration;

pub struct TestDataFactory;

impl TestDataFactory {
    pub fn email() -> String {
        SafeEmail().fake()
    }

    pub fn phone() -> String {
        PhoneNumber().fake()
    }

    pub fn name() -> String {
        Name().fake()
    }

    pub fn address() -> String {
        format!(
            "{} {}",
            BuildingNumber().fake::<String>(),
            StreetName().fake::<String>()
        )
    }

    pub fn time_fake() -> String {
        let max_days_back = 30;
        let seconds_back = rand::thread_rng().gen_range(0..(max_days_back * 24 * 3600));

        let now = Utc::now().naive_utc();
        let t = now - Duration::from_secs(seconds_back);

        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            t.year(),
            t.month(),
            t.day(),
            t.hour(),
            t.minute(),
            t.second()
        )
    }

    pub fn city() -> String {
        CityName().fake()
    }

    pub fn random_string(length: usize) -> String {
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    pub fn random_number(min: i32, max: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }
}
