use crate::coffee::Coffee;

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CoffeBuilderError {
    #[error("required name")]
    NoName,
    #[error("required water")]
    NoWater,
    #[error("required coffe")]
    NoCoffe,
}

#[derive(Debug)]
pub struct CoffeBuilder {
    name: Option<String>,
    coffer_gr: Option<u32>,
    water_ml: Option<u32>,
    milk_ml: Option<u32>,
    sugar_gr: Option<u32>,
    brandy_ml: Option<u32>,
    chocolate_gr: Option<u32>,
}

impl CoffeBuilder {
    pub fn builder() -> CoffeBuilder {
        CoffeBuilder {
            name: None,
            coffer_gr: None,
            water_ml: None,
            milk_ml: None,
            sugar_gr: None,
            brandy_ml: None,
            chocolate_gr: None,
        }
    }

    pub fn name(mut self, name: &str) -> CoffeBuilder {
        self.name = Some(String::from(name));
        self
    }

    pub fn water(mut self, water_ml: u32) -> CoffeBuilder {
        self.water_ml = Some(water_ml);
        self
    }

    pub fn coffe(mut self, coffe_gr: u32) -> CoffeBuilder {
        self.coffer_gr = Some(coffe_gr);
        self
    }

    pub fn sugar(mut self, sugar_gr: u32) -> CoffeBuilder {
        self.sugar_gr = Some(sugar_gr);
        self
    }

    pub fn chocolate(mut self, chocolate_gr: u32) -> CoffeBuilder {
        self.chocolate_gr = Some(chocolate_gr);
        self
    }

    pub fn milk(mut self, milk_ml: u32) -> CoffeBuilder {
        self.milk_ml = Some(milk_ml);
        self
    }

    pub fn brandy(mut self, brandy_ml: u32) -> CoffeBuilder {
        self.brandy_ml = Some(brandy_ml);
        self
    }
    pub fn build(self) -> Result<Coffee, CoffeBuilderError> {
        if self.name.is_none() {
            return Err(CoffeBuilderError::NoName);
        }
        if self.water_ml.is_none() {
            return Err(CoffeBuilderError::NoWater);
        }

        if self.coffer_gr.is_none() {
            return Err(CoffeBuilderError::NoCoffe);
        }

        Ok(Coffee {
            name: self.name.unwrap().clone(),
            coffee_gr: self.coffer_gr.unwrap(),
            water_ml: self.water_ml.unwrap(),
            milk_ml: self.milk_ml,
            sugar_gr: self.sugar_gr,
            brandy_ml: self.brandy_ml,
            chocolate_gr: self.chocolate_gr,
        })
    }
}
