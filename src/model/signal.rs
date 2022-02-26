// siggi/model/signal.rs


use super::{utils::Color};


#[derive(Debug,Clone, PartialEq)]
pub struct Signal {
    pub name: String,  // todo!() change to Option<String>
    pub wave: Wave,
    pub phase: f64,     // Phase shift -> default = 0.0
    pub period: f64,    // Period len  -> default = 1.0
    pub color: Color,
    pub y_axis: (String, String),
}

impl Eq for Signal {}

impl PartialOrd for Signal {
    // Order the signals by their wave length.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.wave.partial_cmp(&other.wave) 
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.wave.cmp(&other.wave)
    }
}

#[derive(Debug,Clone, Default)]
pub struct Wave {
    pub levels: Vec<Level>
}

impl Wave {
    pub fn new() -> Self { Self::default() }

    pub fn len(&self) -> usize {
        self.levels.len()
    }
}

impl PartialEq for Wave {
    // Two waves are equal if the length of the wave is the same.
    fn eq(&self, other: &Self) -> bool {
        self.levels.len() == other.levels.len()
    }
}

impl Eq for Wave {}

impl PartialOrd for Wave {
    // Order the waves by length.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.levels.len().partial_cmp(&other.levels.len())
    }
}

impl Ord for Wave {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.levels.len().cmp(&other.levels.len())
    }
}

#[derive(Debug,Clone,Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
    Idle,
    Up,
    Down,
}

impl Default for Signal {
    fn default() -> Self {
        Self { 
            name: String::from("Sig"), 
            wave: Default::default(), 
            phase: 0.0, 
            period: 1.0, 
            color: Default::default(), 
            y_axis: (String::from("H"),String::from("L")) }
    }
}

impl Signal {
    pub fn new<T>(name: T, wave: Wave) -> Self where T: Into<String> {
        Self { name: name.into(), wave, ..Default::default() }
    }

    /// Shift the wave by a given value
    /// Supports builder pattern
    pub fn shift<T>(mut self, phase: T) -> Self where T: Into<f64> {
        self.phase = phase.into();
        self
    }

    /// Scales the peropd duration of the wave by a given value
    /// Supports builder pattern
    pub fn scale<T>(mut self, period: T) -> Self where T: Into<f64> {
        self.period = period.into();
        self
    }

    /// Coloring the wave
    /// Supports builder pattern
    pub fn color_with<T>(mut self, color: T) -> Self where T: Into<Color> {
        self.color = color.into();
        self
    }

    pub fn set_name<T>(&mut self, name: T) -> &mut Self where T: Into<String> {
        self.name = name.into();
        self
    }

    pub fn set_wave<T>(&mut self, wave: T) -> &mut Self where T: Into<Wave> {
        self.wave = wave.into();
        self
    }

    pub fn set_phase<T>(&mut self, phase: T) -> &mut Self where T: Into<f64> {
        self.phase = phase.into();
        self
    }

    pub fn set_period<T>(&mut self, period: T) -> &mut Self where T: Into<f64> {
        self.period = period.into();
        self
    }

    pub fn set_color<T>(&mut self, color: T) -> &mut Self where T: Into<Color> {
        self.color = color.into();
        self
    }

    pub fn len(&self) -> usize {
        self.wave.len()
    }
}

// Signal Generators
pub trait SignalGenerator {
    fn to_signal(&self) -> Signal;
}

#[derive(Debug, Clone, Copy)]
pub struct Clock {
    typ: ClockType,
    periods: usize,
    name: &'static str,
}


impl Clock {
    pub fn positiv(periods: usize) -> Self {
        Self { typ: ClockType::Positiv, periods, name: "Clock"}
    }

    pub fn negativ(periods: usize) -> Self {
        Self { typ: ClockType::Negativ, periods, name: "Clock"}
    }

    /// Set the clock's name with builder pattern.
    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ClockType {
    Negativ, // goes down on tock
    Positiv, // goes up on tock
}

impl SignalGenerator for Clock {
    fn to_signal(&self) -> Signal {
        let wave_data = match self.typ {
            ClockType::Negativ => vec![Level::Up;self.periods],
            ClockType::Positiv => vec![Level::Down;self.periods],
        };
        Signal::new(self.name.to_string(), Wave{levels: wave_data})
    }
}