use std::vec;

#[derive(Debug, Clone)]
pub struct Fixture {
    pub id: u8,
    pub name: String,
    pub dmx_address: u16,
    pub components: Vec<FixtureComponent>,
}

impl Fixture {
    pub fn new(id: u8, dmx_address: u16, name: String) -> Fixture {
        Fixture {
            id,
            dmx_address,
            components: Vec::new(),
            name,
        }
    }
    pub fn add_component(&mut self, component: FixtureComponent) {
        self.components.push(component);
    }
    pub fn get_dmx_values(&self) -> Vec<u8> {
        let mut dmx_values = Vec::new();
        for component in &self.components {
            dmx_values.append(&mut component.get_dmx_values());
        }
        return dmx_values;
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum FixtureComponent {
    Color(Color),
    Dimmer(Dimmer),
    Position(Position),
    Placeholder,
    ColorWheel(ColorWheel),
    CustomValue(CustomValue),
    Max,
    Zero,
    Gobo(GoboWheel),
    Focus(Focus),
}

impl FixtureComponent {
    fn get_dmx_values(&self) -> Vec<u8> {
        match self {
            FixtureComponent::Color(c) => {
                vec![c.r, c.g, c.b]
            }
            FixtureComponent::Dimmer(d) => {
                vec![d.intensity]
            }
            FixtureComponent::Position(p) => {
                let pan_lower = (p.pan & 0xFF) as u8;
                let pan_higher = (p.pan >> 8) as u8;
                let tilt_lower = (p.tilt & 0xFF) as u8;
                let tilt_higher = (p.tilt >> 8) as u8;
                vec![pan_higher, pan_lower, tilt_higher, tilt_lower]
            }
            FixtureComponent::Placeholder => {
                vec![0]
            }
            FixtureComponent::ColorWheel(c) => {
                vec![c.index]
            }
            FixtureComponent::CustomValue(c) => {
                vec![c.value]
            }
            FixtureComponent::Max => {
                vec![255]
            }
            FixtureComponent::Zero => {
                vec![0]
            }
            FixtureComponent::Gobo(g) => {
                vec![g.index]
            }
            FixtureComponent::Focus(f) => {
                vec![f.value]
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Dimmer {
    /// The user-set local dimmer (0-255). This is the fixture's own dimmer value
    /// before global scaling is applied.
    pub local: u8,
    /// The effective intensity (0-255) after applying global dimmer scaling.
    pub intensity: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    pub pan: u16,
    pub tilt: u16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ColorWheel {
    pub index: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CustomValue {
    pub name: String,
    pub value: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GoboWheel {
    pub index: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Focus {
    pub value: u8,
}
