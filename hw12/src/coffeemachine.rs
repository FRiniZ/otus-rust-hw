use crate::coffee::Coffee;

pub trait CoffeeRecept {
    fn make_coffee(&self) -> Coffee;
}

pub struct CoffeeMachine;

impl CoffeeMachine {
    pub fn brew_coffee<T: CoffeeRecept>(recept: T) -> Coffee {
        recept.make_coffee()
    }
}

pub struct BlackCoffeeRecept;
impl CoffeeRecept for BlackCoffeeRecept {
    fn make_coffee(&self) -> Coffee {
        Coffee::builder()
            .name("Black coffee")
            .water(200)
            .coffe(20)
            .sugar(5)
            .build()
            .unwrap()
    }
}

pub struct CoffeeWithMilkRecept;
impl CoffeeRecept for CoffeeWithMilkRecept {
    fn make_coffee(&self) -> Coffee {
        Coffee::builder()
            .name("Coffee with milk")
            .water(150)
            .coffe(20)
            .sugar(5)
            .milk(50)
            .build()
            .unwrap()
    }
}

pub struct CoffeeWithChocolateRecept;
impl CoffeeRecept for CoffeeWithChocolateRecept {
    fn make_coffee(&self) -> Coffee {
        Coffee::builder()
            .name("Coffee with chocolate")
            .water(200)
            .coffe(20)
            .sugar(5)
            .chocolate(10)
            .build()
            .unwrap()
    }
}

pub struct CoffeeWithAlcoholRecept;
impl CoffeeRecept for CoffeeWithAlcoholRecept {
    fn make_coffee(&self) -> Coffee {
        Coffee::builder()
            .name("Coffee with alcohol")
            .water(150)
            .coffe(20)
            .sugar(5)
            .brandy(50)
            .build()
            .unwrap()
    }
}
