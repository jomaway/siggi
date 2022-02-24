// siggi/model/signal.rs

use super::utils::Color;


#[derive(Debug,Clone, PartialEq)]
pub struct Signal {
    pub name: &'static str,
    pub wave: Wave,
    pub phase: f64,
    pub period: f64,
    pub color: Color,
    pub y_axis: (&'static str, &'static str),
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

#[derive(Debug,Clone,Copy)]
pub enum Level {
    LOW,
    HIGH,
    IDLE,
    UP,
    DOWN,
}

impl Default for Signal {
    fn default() -> Self {
        Self { 
            name: "Sig", 
            wave: Default::default(), 
            phase: Default::default(), 
            period: Default::default(), 
            color: Default::default(), 
            y_axis: ("H","L") }
    }
}

impl Signal {
    pub fn name<T>(&mut self, name: T) -> &mut Self where T: Into<&'static str> {
        self.name = name.into();
        self
    }

    pub fn wave<T>(&mut self, wave: T) -> &mut Self where T: Into<Wave> {
        self.wave = wave.into();
        self
    }

    pub fn phase<T>(&mut self, phase: T) -> &mut Self where T: Into<f64> {
        self.phase = phase.into();
        self
    }

    pub fn period<T>(&mut self, period: T) -> &mut Self where T: Into<f64> {
        self.period = period.into();
        self
    }

    pub fn color<T>(&mut self, color: T) -> &mut Self where T: Into<Color> {
        self.color = color.into();
        self
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
            ClockType::Negativ => vec![Level::UP;self.periods],
            ClockType::Positiv => vec![Level::DOWN;self.periods],
        };
        Signal::default()
            .name(self.name)
            .wave(Wave{levels: wave_data})
            .clone() //  todo!() check if consuming builder would be better here.
    }
}