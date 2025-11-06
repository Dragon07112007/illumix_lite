use crate::lib::{
    fixture::{Color, CustomValue, Dimmer, Fixture, FixtureComponent, Position},
    universe::Universe,
};

pub fn get_universe() -> Universe {
    let mut universe = Universe::new();

    let mut test_light = Fixture::new(1, 17, "Test Light".to_string());
    test_light.add_component(FixtureComponent::Dimmer(Dimmer { intensity: 255 }));

    universe.add_fixture(test_light);

    let mut led = Fixture::new(2, 260, "LED Light".to_string());
    led.add_component(FixtureComponent::Position(Position { pan: 0, tilt: 0 }));
    led.add_component(FixtureComponent::Color(Color { r: 0, g: 0, b: 0 }));
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
    

    universe.add_fixture(led);

    return universe;
}
