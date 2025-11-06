use std::time;
use std::sync::{Arc, Mutex};

pub trait Effect {
    /// Advance the present by `time_delta` and apply any changes to `universe`.
    fn tick(&mut self, time_delta: time::Duration, universe: &mut crate::lib::universe::Universe);
}

pub struct GradientEffect {
    pub speed: f32,
    pub colors: Vec<[u8; 3]>,
    pub position: f32,
}

impl Effect for GradientEffect {
    fn tick(&mut self, time_delta: time::Duration, universe: &mut crate::lib::universe::Universe) {
        // advance the internal position
        let delta_seconds = time_delta.as_secs_f32();
        self.position += self.speed * delta_seconds;
        if self.position > self.colors.len() as f32 {
            self.position -= self.colors.len() as f32;
        }

        // apply color to fixture #2 if present
        if let Some(fixture) = universe.get_fixture_by_id_mut(2) {
            if let Some(crate::lib::fixture::FixtureComponent::Color(rgb)) = fixture
                .components
                .iter_mut()
                .find(|comp| matches!(comp, crate::lib::fixture::FixtureComponent::Color(_)))
            {
                let color_index = self.position.floor() as usize % self.colors.len();
                let next_color_index = (color_index + 1) % self.colors.len();
                let t = self.position.fract();

                let color1 = self.colors[color_index];
                let color2 = self.colors[next_color_index];

                rgb.r = ((1.0 - t) * color1[0] as f32 + t * color2[0] as f32) as u8;
                rgb.g = ((1.0 - t) * color1[1] as f32 + t * color2[1] as f32) as u8;
                rgb.b = ((1.0 - t) * color1[2] as f32 + t * color2[2] as f32) as u8;
                // println!(
                //     "GradientPresent applied color: r={}, g={}, b={}",
                //     rgb.r, rgb.g, rgb.b
                // );
            }
        }
    }
}



pub fn launch_present_thread(
    universe: Arc<Mutex<crate::lib::universe::Universe>>,
    tick_rate: time::Duration,
) {
    std::thread::spawn(move || {
        let mut last_instant = time::Instant::now();
        loop {
            let now = time::Instant::now();
            let delta = now.duration_since(last_instant);
            last_instant = now;

            {
                let mut universe = universe.lock().unwrap();
                // take presents out so we can mutably borrow the universe while ticking
                let mut presents = std::mem::take(&mut universe.presents);
                for present in presents.iter_mut() {
                    present.tick(delta, &mut universe);
                }
                universe.presents = presents;
            }

            std::thread::sleep(tick_rate);
        }
    });
}