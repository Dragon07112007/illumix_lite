use serde::{Deserialize, Serialize};

use crate::lib::fixture::Fixture;

#[derive(Debug, Clone)]
pub struct Universe {
    pub fixtures: Vec<Fixture>,
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            fixtures: Vec::new(),
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

        return dmx_values;
    }

    pub fn add_fixture(&mut self, fixture: Fixture) {
        self.fixtures.push(fixture);
    }
}
