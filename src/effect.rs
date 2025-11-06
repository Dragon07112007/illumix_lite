use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time;

#[derive(Clone, Copy, Debug)]
pub enum ParColor {
    Cool,
    Warm,
    Amber,
}

impl ParColor {
    fn next(&self) -> ParColor {
        match self {
            ParColor::Cool => ParColor::Warm,
            ParColor::Warm => ParColor::Amber,
            ParColor::Amber => ParColor::Cool,
        }
    }

    fn apply(&self, fixture: &mut crate::lib::fixture::Fixture) {
        // Reset all colors first
        for comp in fixture.components.iter_mut() {
            if let crate::lib::fixture::FixtureComponent::CustomValue(cv) = comp {
                match cv.name.as_str() {
                    "cool_white" | "warm_white" | "amber" => cv.value = 0,
                    _ => {}
                }
            }
        }

        // Set the active color to full
        let component_name = match self {
            ParColor::Cool => "cool_white",
            ParColor::Warm => "warm_white",
            ParColor::Amber => "amber",
        };

        for comp in fixture.components.iter_mut() {
            if let crate::lib::fixture::FixtureComponent::CustomValue(cv) = comp {
                if cv.name == component_name {
                    cv.value = 255;
                    break;
                }
            }
        }
    }
}

pub trait Effect {
    /// Advance the present by `time_delta` and apply any changes to `universe`.
    fn tick(&mut self, time_delta: time::Duration, universe: &mut crate::lib::universe::Universe);

    /// Return a shared Any reference for downcasting; the lifetime is tied to &self.
    fn as_any(&self) -> &dyn Any;

    /// Allow mutable downcasting from trait object to concrete type.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ColorSwapEffect {
    pub bpm: f32,
    pub offset_pattern: bool,
    pub fixture_offsets: Vec<f32>,
    pub smooth: bool,                // Enable/disable smooth transitions
    accumulated_time: f32,
    current_colors: Vec<ParColor>,
    transition_progress: Vec<f32>,   // Tracks transition progress per fixture (0.0 to 1.0)
}

impl ColorSwapEffect {
    pub fn new(bpm: f32, fixture_count: usize, offset_pattern: bool, smooth: bool) -> Self {
        let mut fixture_offsets = vec![0.0; fixture_count];
        let mut current_colors = Vec::with_capacity(fixture_count);
        let mut transition_progress = vec![1.0; fixture_count];

        for i in 0..fixture_count {
            let color_offset = if offset_pattern {
                i % 3
            } else {
                0
            };
            current_colors.push(match color_offset {
                0 => ParColor::Cool,
                1 => ParColor::Warm,
                2 => ParColor::Amber,
                _ => unreachable!(),
            });
        }

        ColorSwapEffect {
            bpm,
            offset_pattern,
            fixture_offsets,
            smooth,
            accumulated_time: 0.0,
            current_colors,
            transition_progress,
        }
    }

    /// Change the offset pattern at runtime
    pub fn set_offset_pattern(&mut self, offset_pattern: bool) {
        self.offset_pattern = offset_pattern;
        for i in 0..self.current_colors.len() {
            let color_offset = if offset_pattern {
                i % 3
            } else {
                0
            };
            self.current_colors[i] = match color_offset {
                0 => ParColor::Cool,
                1 => ParColor::Warm,
                2 => ParColor::Amber,
                _ => unreachable!(),
            };
        }
    }

    fn apply_transition(&self, fixture: &mut crate::lib::fixture::Fixture, color: ParColor, intensity: u8) {
        // Helper to find and update a color component
        let update_component = |name: &str, value: u8, fixture: &mut crate::lib::fixture::Fixture| {
            for comp in fixture.components.iter_mut() {
                if let crate::lib::fixture::FixtureComponent::CustomValue(cv) = comp {
                    if cv.name == name {
                        cv.value = value;
                        break;
                    }
                }
            }
        };

        // Reset all colors first
        update_component("cool_white", 0, fixture);
        update_component("warm_white", 0, fixture);
        update_component("amber", 0, fixture);

        // Set the active color
        let component_name = match color {
            ParColor::Cool => "cool_white",
            ParColor::Warm => "warm_white",
            ParColor::Amber => "amber",
        };
        update_component(component_name, intensity, fixture);
    }
}

impl Effect for ColorSwapEffect {
    fn tick(&mut self, time_delta: time::Duration, universe: &mut crate::lib::universe::Universe) {
        let delta_seconds = time_delta.as_secs_f32();
        self.accumulated_time += delta_seconds;
        
        // Calculate beat duration and global progress
        let seconds_per_beat = 60.0 / self.bpm;
        let cycle_progress = (self.accumulated_time / seconds_per_beat) % 1.0;
        
        // Detect beat change (synchronized for all fixtures)
        let is_beat_change = cycle_progress < delta_seconds / seconds_per_beat;

        // Add debug print for timing
        // if is_beat_change {
        //     println!("Beat change at {} seconds, progress: {}", self.accumulated_time, cycle_progress);
        // }

        // Update each fixture
        for i in 0..self.fixture_offsets.len() {
            let fixture_id = 1 + i as u8; // PAR fixtures are 1-7
            if let Some(fixture) = universe.get_fixture_by_id_mut(fixture_id) {
                if self.smooth {
                    // Crossfade between current and next color over the beat
                    let mut current_color = self.current_colors[i];
                    let mut next_color = current_color.next();
                    let t = cycle_progress;
                    // Only swap color at the end of the beat, but use previous color for this tick's blend
                    if is_beat_change {
                        self.current_colors[i] = next_color;
                        // For this tick, blend from previous color to next color
                        current_color = next_color;
                        next_color = current_color.next();
                    }
                    // Set both colors with crossfade
                    // Reset all colors first
                    for comp in fixture.components.iter_mut() {
                        if let crate::lib::fixture::FixtureComponent::CustomValue(cv) = comp {
                            match cv.name.as_str() {
                                "cool_white" | "warm_white" | "amber" => cv.value = 0,
                                _ => {}
                            }
                        }
                    }
                    let set_intensity = |color: ParColor, value: u8, fixture: &mut crate::lib::fixture::Fixture| {
                        let name = match color {
                            ParColor::Cool => "cool_white",
                            ParColor::Warm => "warm_white",
                            ParColor::Amber => "amber",
                        };
                        for comp in fixture.components.iter_mut() {
                            if let crate::lib::fixture::FixtureComponent::CustomValue(cv) = comp {
                                if cv.name == name {
                                    cv.value = value;
                                }
                            }
                        }
                    };
                    set_intensity(current_color, ((1.0 - t) * 255.0) as u8, fixture);
                    set_intensity(next_color, (t * 255.0) as u8, fixture);
                } else {
                    // For non-smooth, change color instantly at beat
                    if is_beat_change {
                        self.current_colors[i] = self.current_colors[i].next();
                    }
                    self.apply_transition(fixture, self.current_colors[i], 255);
                }
            }
        }
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
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
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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
                let mut effects = std::mem::take(&mut universe.effects);
                for effect in effects.iter_mut() {
                    effect.tick(delta, &mut universe);
                    //println!("Ticked effect");
                }
                universe.effects = effects;
            }

            std::thread::sleep(tick_rate);
        }
    });
}
