use crate::lib::{
    fixture::{Color, CustomValue, Dimmer, Fixture, FixtureComponent, Position},
    universe::Universe,
};

pub fn get_universe() -> Universe {
    let mut universe = Universe::new();

    // let mut test_light = Fixture::new(1, 17, "Test Light".to_string());
    // test_light.add_component(FixtureComponent::Dimmer(Dimmer { intensity: 255 }));

    // universe.add_fixture(test_light);
    for i in 1..=7  {
        let mut led = Fixture::new(i, (300 + (i as u16 - 1) * 20), "LED Light".to_string());
        led.add_component(FixtureComponent::Position(Position { pan: 0, tilt: 0 }));
        //led.add_component(FixtureComponent::Color(Color { r: 0, g: 0, b: 0 }));
                // Channel 1: Cool White
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "cool_white".to_string(),
            value: 0,
        }));
        
        // Channel 2: Warm White
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "warm_white".to_string(),
            value: 0,
        }));
        
        // Channel 3: Amber
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "amber".to_string(),
            value: 0,
        }));

        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "white".to_string(),
            value: 128,
        }));
        led.add_component(FixtureComponent::Max);
        led.add_component(FixtureComponent::Dimmer(Dimmer { intensity: 255 })); //shutter
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "zoom".to_string(),
            value: 128,
        }));
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "strobe".to_string(),
            value: 0,
        }));
        universe.add_fixture(led);

    }

    let mut led = Fixture::new(8, 260, "LED Light".to_string());
        led.add_component(FixtureComponent::Position(Position { pan: 0, tilt: 0 }));
        //led.add_component(FixtureComponent::Color(Color { r: 0, g: 0, b: 0 }));
                // Channel 1: Cool White
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "cool_white".to_string(),
            value: 0,
        }));
        
        // Channel 2: Warm White
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "warm_white".to_string(),
            value: 0,
        }));
        
        // Channel 3: Amber
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "amber".to_string(),
            value: 0,
        }));

        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "white".to_string(),
            value: 128,
        }));
        led.add_component(FixtureComponent::Max);
        led.add_component(FixtureComponent::Dimmer(Dimmer { intensity: 255 })); //shutter
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "zoom".to_string(),
            value: 128,
        }));
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "strobe".to_string(),
            value: 0,
        }));
        universe.add_fixture(led);

    let mut led = Fixture::new(9, 240, "LED Light".to_string());
        led.add_component(FixtureComponent::Position(Position { pan: 0, tilt: 0 }));
        //led.add_component(FixtureComponent::Color(Color { r: 0, g: 0, b: 0 }));
                // Channel 1: Cool White
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "cool_white".to_string(),
            value: 0,
        }));
        
        // Channel 2: Warm White
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "warm_white".to_string(),
            value: 0,
        }));
        
        // Channel 3: Amber
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "amber".to_string(),
            value: 0,
        }));

        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "white".to_string(),
            value: 128,
        }));
        led.add_component(FixtureComponent::Max);
        led.add_component(FixtureComponent::Dimmer(Dimmer { intensity: 255 })); //shutter
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "zoom".to_string(),
            value: 128,
        }));
        led.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "strobe".to_string(),
            value: 0,
        }));
        universe.add_fixture(led);

    // 

    // Add 7 LED PAR fixtures, each using 6 channels
    // for i in 0..7 {
    //     let dmx_start = 1 + (i * 6); // Starting from DMX address 1, each using 6 channels
    //     let mut par = Fixture::new(10 + i as u8, dmx_start, format!("LED PAR {}", i + 1));
        
    //     // Channel 1: Cool White
    //     par.add_component(FixtureComponent::CustomValue(CustomValue {
    //         name: "cool_white".to_string(),
    //         value: 0,
    //     }));
        
    //     // Channel 2: Warm White
    //     par.add_component(FixtureComponent::CustomValue(CustomValue {
    //         name: "warm_white".to_string(),
    //         value: 0,
    //     }));
        
    //     // Channel 3: Amber
    //     par.add_component(FixtureComponent::CustomValue(CustomValue {
    //         name: "amber".to_string(),
    //         value: 0,
    //     }));
        
    //     // Channel 4: Color Temperature Macros
    //     par.add_component(FixtureComponent::CustomValue(CustomValue {
    //         name: "color_temp".to_string(),
    //         value: 0, // 0-18 is OFF by default
    //     }));
        
    //     // Channel 5: Strobe
    //     par.add_component(FixtureComponent::CustomValue(CustomValue {
    //         name: "strobe".to_string(),
    //         value: 0,
    //     }));
        
    //     // Channel 6: Master Dimmer
    //     par.add_component(FixtureComponent::Dimmer(Dimmer {
    //         intensity: 255, // Full intensity by default
    //     }));

    //     universe.add_fixture(par);
    // }

    return universe;
}
