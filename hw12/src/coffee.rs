use crate::coffeebuilder::CoffeBuilder;

#[derive(Debug, PartialEq)]
pub struct Coffee {
    pub name: String,
    pub coffee_gr: u32,
    pub water_ml: u32,
    pub milk_ml: Option<u32>,
    pub sugar_gr: Option<u32>,
    pub brandy_ml: Option<u32>,
    pub chocolate_gr: Option<u32>,
}

impl Coffee {
    pub fn new(name: &str, coffe_gr: u32, water_ml: u32) -> Self {
        Self {
            name: String::from(name),
            coffee_gr: coffe_gr,
            water_ml,
            milk_ml: None,
            sugar_gr: None,
            brandy_ml: None,
            chocolate_gr: None,
        }
    }

    pub fn builder() -> CoffeBuilder {
        CoffeBuilder::builder()
    }
}
