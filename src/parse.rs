// parse.rs 
pub mod error;

use std::str::FromStr;

use crate::model::{signal::{Wave, Level}, Signal};
use self::error::ParseWaveError;

// impl FromStr traits

impl FromStr for Signal {
    type Err = ParseWaveError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        unimplemented!()
    }
}

impl FromStr for Wave {
    type Err = ParseWaveError;

    // Parse a string to a wave
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let mut wave: Wave = Wave::new();
        for char in s.chars() {
            match char.to_ascii_uppercase() {
                'L' => wave.levels.push(Level::Low),
                'H' => wave.levels.push(Level::High),
                'U' => wave.levels.push(Level::Up),
                'D' => wave.levels.push(Level::Down),
                'I' => wave.levels.push(Level::Idle),
                _ => return Err(ParseWaveError::new("Could not parse signal wave")),    
            }
        }
        Ok(wave)
    }
}