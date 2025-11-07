use crate::lib::{
    fixture::{Color, ColorWheel, CustomValue, Dimmer, Fixture, FixtureComponent, Focus, GoboWheel, Position},
    universe::Universe,
};

pub fn get_universe() -> Universe {
    let mut universe = Universe::new();

    // let mut test_light = Fixture::new(1, 17, "Test Light".to_string());
    // test_light.add_component(FixtureComponent::Dimmer(Dimmer { intensity: 255 }));

    // universe.add_fixture(test_light);

    //Add 7 LED PAR fixtures, each using 6 channels
    for i in 0..7 {
        let dmx_start = 1 + (i * 6); // Starting from DMX address 1, each using 6 channels
        let mut par = Fixture::new(i + 1 as u8, dmx_start as u16, format!("LED PAR {}", i + 1));

        // Channel 1: Cool White
        par.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "cool_white".to_string(),
            value: 0,
        }));

        // Channel 2: Warm White
        par.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "warm_white".to_string(),
            value: 0,
        }));

        // Channel 3: Amber
        par.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "amber".to_string(),
            value: 0,
        }));

        // Channel 4: Color Temperature Macros
        par.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "color_temp".to_string(),
            value: 0, // 0-18 is OFF by default
        }));

        // Channel 5: Strobe
        par.add_component(FixtureComponent::CustomValue(CustomValue {
            name: "strobe".to_string(),
            value: 0,
        }));

        // Channel 6: Master Dimmer
        par.add_component(FixtureComponent::Dimmer(Dimmer {
            intensity: 255, // Full intensity by default
            local: 255,
        }));

        universe.add_fixture(par);
    }

    for i in 1..=2 {
        let mut moving_head = Fixture::new(7 + i as u8, 100 + (i - 1) * 16, format!("Moving Head {}", i));
        moving_head.add_component(FixtureComponent::Position(Position { pan: 0, tilt: 0 }));
        moving_head.add_component(FixtureComponent::ColorWheel(ColorWheel { index: 0 })); //5
        moving_head.add_component(FixtureComponent::Gobo(GoboWheel { index: 0 })); //6
        moving_head.add_component(FixtureComponent::Zero); //7
        moving_head.add_component(FixtureComponent::Zero); //8
        moving_head.add_component(FixtureComponent::Zero); // 9
        moving_head.add_component(FixtureComponent::Zero); // 10
        moving_head.add_component(FixtureComponent::Focus(Focus { value: 0 })); //11
        moving_head.add_component(FixtureComponent::Max); //12
        moving_head.add_component(FixtureComponent::Dimmer(Dimmer { intensity: 255, local: 255 })); //13
        moving_head.add_component(FixtureComponent::Zero); // 14
        moving_head.add_component(FixtureComponent::Zero); // 15
        moving_head.add_component(FixtureComponent::Zero); // 16


        universe.add_fixture(moving_head);
    }


    return universe;
}
