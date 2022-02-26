// parse.rs 
pub mod error;

use serde::Deserialize;


use std::str::FromStr;

use crate::model::{signal::{Wave, Level, Clock, SignalGenerator}, Signal, Diagram, Lane, utils::Color};
use self::error::{ParseWaveError, ParseError};


fn default_period() -> f64 {
    1.0
}

#[derive(Debug, Deserialize)]
struct JsonData {
    signals: Vec<JsonSignal>,
    config: JsonConfig,
}
#[derive(Debug, Deserialize)]
struct JsonSignal {
    name: String,
    wave: String,
    #[serde(default)]
    phase: f64,
    #[serde(default = "default_period")]
    period: f64,
    color: Color,
}

#[derive(Debug, Deserialize)]
struct  JsonConfig {
    title: Option<String>,
    _background: Option<String>,
}

// parse diagram from json str
pub fn from_json_str(json: &str) -> Result<Diagram,ParseError> {
        
    let data : JsonData = serde_json::from_str(json)?;

    let mut diagram = Diagram::from(data.config);

    for json_signal in data.signals {
        diagram.append(Lane::new(Signal::try_from(json_signal)?));
    }
    
    Ok(diagram)
}

// parse diagram from args  -> not all features are supported
pub fn from_args(title: String, dark: bool, clocks: Vec<String>, signals: Vec<String>) -> Result<Diagram,ParseError> {
    let mut diag = Diagram::new(Some(title)).dark(dark);
    
    for clock in clocks.iter() {
        diag.append(Lane::new(clock.parse::<Signal>().expect("Could not parse clock from given string")));
    }
    
    for (num,wave) in signals.iter().enumerate() {
        let wave = wave.parse::<Wave>()?;
        let signal = Signal::new(format!("s-{}",num), wave);
        
        diag.append(Lane::new(signal));
    }
    Ok(diag)
}


// impl FromStr traits

impl FromStr for Signal {
    type Err = ParseWaveError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        if let Some((typ, periods)) = s.split_once('-'){
            match typ {
                "pclk" =>  return Ok(Clock::positiv(periods.parse::<usize>().unwrap()).to_signal()), // Change this unwrap()
                "nclk" =>  return Ok(Clock::negativ(periods.parse::<usize>().unwrap()).to_signal()),
                _ => return Err(ParseWaveError::new("Could not parse signal")),
            }
        } 
        Err(ParseWaveError::new("Could not parse signal from given string"))
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


impl TryFrom<JsonSignal> for Signal {
    type Error = ParseError;

    fn try_from(json_signal: JsonSignal) -> Result<Self, Self::Error> {
        Ok(Signal::new(json_signal.name, json_signal.wave.parse()?)
            .shift(json_signal.phase)
            .scale(json_signal.period)
            .color_with(json_signal.color))
    }
} 

impl From<JsonConfig> for Diagram {
    fn from(json_config: JsonConfig) -> Self {
        Diagram::new(json_config.title)
        // background not supported yet
    }
}