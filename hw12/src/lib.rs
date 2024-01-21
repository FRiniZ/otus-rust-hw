pub mod coffee;
pub mod coffeebuilder;
pub mod coffeemachine;

#[cfg(test)]
mod tests {
    use crate::{
        coffee::Coffee,
        coffeemachine::{
            BlackCoffeeRecept, CoffeeMachine, CoffeeWithAlcoholRecept, CoffeeWithChocolateRecept,
            CoffeeWithMilkRecept,
        },
    };

    #[test]
    fn pattern_builder1() {
        let recept1: Coffee = Coffee {
            name: "Black Coffe".to_string(),
            coffee_gr: 5,
            water_ml: 200,
            milk_ml: None,
            sugar_gr: None,
            brandy_ml: None,
            chocolate_gr: None,
        };

        let coffe = Coffee::builder()
            .name(recept1.name.as_str())
            .water(recept1.water_ml)
            .coffe(recept1.coffee_gr)
            .build()
            .unwrap();

        assert_eq!(coffe, recept1);
    }

    #[test]
    fn pattern_builder2() {
        let recept2: Coffee = Coffee {
            name: "Coffee with Milk(25ml)".to_string(),
            coffee_gr: 5,
            water_ml: 200,
            milk_ml: Some(25),
            sugar_gr: None,
            brandy_ml: None,
            chocolate_gr: None,
        };

        let coffe = Coffee::builder()
            .name(recept2.name.as_str())
            .water(recept2.water_ml)
            .coffe(recept2.coffee_gr)
            .milk(recept2.milk_ml.unwrap())
            .build()
            .unwrap();

        assert_eq!(coffe, recept2);
    }

    #[test]
    fn pattern_builder3() {
        let recept3: Coffee = Coffee {
            name: "Coffee with Milk(25ml) and Chocolate(30gr)".to_string(),
            coffee_gr: 5,
            water_ml: 200,
            milk_ml: Some(25),
            sugar_gr: None,
            brandy_ml: None,
            chocolate_gr: Some(30),
        };

        let coffe = Coffee::builder()
            .name(recept3.name.as_str())
            .water(recept3.water_ml)
            .coffe(recept3.coffee_gr)
            .milk(recept3.milk_ml.unwrap())
            .chocolate(recept3.chocolate_gr.unwrap())
            .build()
            .unwrap();

        assert_eq!(coffe, recept3);
    }

    #[test]
    fn pattern_builder4() {
        let recept4: Coffee = Coffee {
            name: "Coffee with Milk(25ml) and Chocolate(30gr) and Sugar(5)".to_string(),
            coffee_gr: 5,
            water_ml: 200,
            milk_ml: Some(25),
            sugar_gr: Some(5),
            brandy_ml: None,
            chocolate_gr: Some(30),
        };

        let coffe = Coffee::builder()
            .name(recept4.name.as_str())
            .water(recept4.water_ml)
            .coffe(recept4.coffee_gr)
            .milk(recept4.milk_ml.unwrap())
            .chocolate(recept4.chocolate_gr.unwrap())
            .sugar(recept4.sugar_gr.unwrap())
            .build()
            .unwrap();

        assert_eq!(coffe, recept4);
    }

    #[test]
    fn pattern_builder5() {
        let recept5: Coffee = Coffee {
            name: "Coffee with Milk(25ml) and Chocolate(30gr) and Sugar(5) and Brandy(5)"
                .to_string(),
            coffee_gr: 5,
            water_ml: 200,
            milk_ml: Some(25),
            sugar_gr: Some(5),
            brandy_ml: Some(5),
            chocolate_gr: Some(30),
        };

        let coffe = Coffee::builder()
            .name(recept5.name.as_str())
            .water(recept5.water_ml)
            .coffe(recept5.coffee_gr)
            .milk(recept5.milk_ml.unwrap())
            .chocolate(recept5.chocolate_gr.unwrap())
            .sugar(recept5.sugar_gr.unwrap())
            .brandy(recept5.brandy_ml.unwrap())
            .build()
            .unwrap();

        assert_eq!(coffe, recept5);
    }

    #[test]
    fn pattern_strategy1() {
        let recept5: Coffee = Coffee {
            name: "Black coffee".to_string(),
            coffee_gr: 20,
            water_ml: 200,
            milk_ml: None,
            sugar_gr: Some(5),
            brandy_ml: None,
            chocolate_gr: None,
        };

        let coffee = CoffeeMachine::brew_coffee(BlackCoffeeRecept);

        assert_eq!(coffee, recept5);
    }

    #[test]
    fn pattern_strategy2() {
        let recept5: Coffee = Coffee {
            name: "Coffee with milk".to_string(),
            coffee_gr: 20,
            water_ml: 150,
            milk_ml: Some(50),
            sugar_gr: Some(5),
            brandy_ml: None,
            chocolate_gr: None,
        };

        let coffee = CoffeeMachine::brew_coffee(CoffeeWithMilkRecept);

        assert_eq!(coffee, recept5);
    }

    #[test]
    fn pattern_strategy3() {
        let recept5: Coffee = Coffee {
            name: "Coffee with chocolate".to_string(),
            coffee_gr: 20,
            water_ml: 200,
            milk_ml: None,
            sugar_gr: Some(5),
            brandy_ml: None,
            chocolate_gr: Some(10),
        };

        let coffee = CoffeeMachine::brew_coffee(CoffeeWithChocolateRecept);

        assert_eq!(coffee, recept5);
    }

    #[test]
    fn pattern_strategy4() {
        let recept5: Coffee = Coffee {
            name: "Coffee with alcohol".to_string(),
            coffee_gr: 20,
            water_ml: 150,
            milk_ml: None,
            sugar_gr: Some(5),
            brandy_ml: Some(50),
            chocolate_gr: None,
        };

        let coffee = CoffeeMachine::brew_coffee(CoffeeWithAlcoholRecept);

        assert_eq!(coffee, recept5);
    }
}
