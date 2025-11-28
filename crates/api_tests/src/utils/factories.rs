use fake::{
    faker::{address::en::*, internet::en::*, name::en::*, phone_number::en::*},
    Fake,
};
use rand::Rng;

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

    pub fn city() -> String {
        CityName().fake()
    }

    pub fn uuid() -> String {
        uuid::Uuid::new_v4().to_string()
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

#[allow(dead_code)]
mod uuid {
    pub struct Uuid;
    impl Uuid {
        pub fn new_v4() -> Self {
            Uuid
        }
        pub fn to_string(&self) -> String {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            format!(
                "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
                rng.gen::<u32>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u64>() & 0xFFFFFFFFFFFF
            )
        }
    }
}
