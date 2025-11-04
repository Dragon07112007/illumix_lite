use crate::lib::{fixture::{Dimmer, Fixture, FixtureComponent}, universe::Universe};




pub fn get_universe() -> Universe {
    let mut universe = Universe::new();


    let mut test_light = Fixture::new(1, 17, "Test Light".to_string());
    test_light.add_component(FixtureComponent::Dimmer(Dimmer { intensity: 255 }));

    universe.add_fixture(test_light);

    return universe;

}