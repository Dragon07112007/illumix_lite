use serde::{Deserialize, Serialize};

use crate::lib::fixture::Fixture;

pub struct Universe {
    pub fixtures: Vec<Fixture>,
    pub effects: Vec<Box<dyn crate::effect::Effect + Send>>,
    pub global_dimmer: u8,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            fixtures: Vec::new(),
            effects: Vec::new(),
            global_dimmer: 255,
        }
    }

    pub fn get_fixture_by_id(&self, id: u8) -> Option<&Fixture> {
        for fixture in &self.fixtures {
            if fixture.id == id {
                return Some(&fixture);
            }
        }
        None
    }

    pub fn get_fixture_by_id_mut(&mut self, id: u8) -> Option<&mut Fixture> {
        for fixture in &mut self.fixtures {
            if fixture.id == id {
                return Some(fixture);
            }
        }
        None
    }

    pub fn get_dmx_values(&self) -> [u8; 512] {
        let mut dmx_values = [0u8; 512];
        for fixture in &self.fixtures {
            let fixture_values = fixture.get_dmx_values();
            dmx_values[(fixture.dmx_address - 1) as usize
                ..(fixture.dmx_address as usize - 1 + fixture_values.len())]
                .copy_from_slice(&fixture_values);
        }
        //println!("DMX Values: {:?}", &dmx_values[300..310]);
        return dmx_values;
    }

    pub fn add_fixture(&mut self, fixture: Fixture) {
        self.fixtures.push(fixture);
    }

    pub fn insert_present<P: crate::effect::Effect + Send + 'static>(&mut self, present: P) {
        self.effects.push(Box::new(present));
    }
}

impl std::fmt::Debug for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Universe")
            .field("fixtures", &self.fixtures)
            .field("presents_len", &self.effects.len())
            .field("global_dimmer", &self.global_dimmer)
            .finish()
    }
}

impl Clone for Universe {
    fn clone(&self) -> Self {
        Universe {
            fixtures: self.fixtures.clone(),
            effects: Vec::new(),
            global_dimmer: self.global_dimmer,
        }
    }
}
