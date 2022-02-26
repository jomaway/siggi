// parse.rs 
pub mod error;

use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use std::str::FromStr;

use crate::model::{signal::{Wave, Level, Clock, SignalGenerator}, Signal, Diagram, Lane, utils::Color, marker::Marker};
use self::error::{ParseWaveError, ParseError};


fn default_to_1() -> f64 {
    1.0
}

fn default_yaxis() -> String {
    "H,L".into()
}

#[derive(Debug, Deserialize)]
struct JsonData {
    signals: Vec<JsonSignal>,
    config: Option<JsonConfig>,
}

#[derive(Debug, Deserialize, Default)]
struct  JsonConfig {
    title: Option<String>,
    _background: Option<String>,
    //_show_ticks: bool,
}

#[derive(Debug, Deserialize)]
struct JsonSignal {
    name: String,
    wave: String,
    #[serde(default)]
    phase: f64,
    #[serde(default = "default_to_1")]
    period: f64,
    #[serde(default)]
    color: Color,
    #[serde(default = "default_yaxis")]
    _yaxis: String, // todo!()
    #[serde(default, deserialize_with = "de_markers")]
    markers: Vec<Marker>,

}

fn de_markers<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Marker>, D::Error> {
    let mut markers = Vec::<Marker>::new();

    for value in Value::deserialize(deserializer) {
        if let Ok(Value::Array(arr)) = Value::deserialize(value) {
            for val in arr {
                match val {
                    Value::Number(pos) => { 
                        if let Some(pos) = pos.as_f64() {
                            markers.push(Marker::default().at(pos));
                        }
                    },
                    Value::Object(marker) => {
                        let color : Color = if let Some(field) = marker.get("color") { 
                            serde_json::from_value(field.clone()).unwrap_or(Color::default()) 
                        } else { Color::default() };

                        let dashed : bool = if let Some(field) = marker.get("dashed") { 
                            serde_json::from_value(field.clone()).unwrap_or(true) 
                        } else { true };

                        let thickness : f64 = if let Some(field) = marker.get("thickness") { 
                            serde_json::from_value(field.clone()).unwrap_or(1.0) 
                        } else { 1.0 };
                        
                        if let Some(positions) = marker["at"].as_array() {
                            for pos in positions {
                                if let Some(pos) = pos.as_f64() {
                                    markers.push(Marker::new(pos, dashed, thickness, color));
                                }
                            }
                        }
                    },
                    _ => return Err(de::Error::custom("wrong type"))
                }
            }
        }
    }

    Ok(markers)

}


// parse diagram from json str
pub fn from_json_str(json: &str) -> Result<Diagram,ParseError> {
        
    let data : JsonData = serde_json::from_str(json)?;

    let mut diagram = Diagram::from(data.config.unwrap_or_default());

    for json_signal in data.signals {
        diagram.append(Lane::try_from(&json_signal)?);
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


impl TryFrom<&JsonSignal> for Signal {
    type Error = ParseError;

    fn try_from(json_signal: &JsonSignal) -> Result<Self, Self::Error> {
        Ok(Signal::new(&json_signal.name, json_signal.wave.parse()?)
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


impl TryFrom<&JsonSignal> for Lane {
    type Error = ParseError;

    fn try_from(json_signal: &JsonSignal) -> Result<Self, Self::Error> {
        let mut lane = Lane::new(Signal::try_from(json_signal)?);

        for marker in json_signal.markers.iter() {
            lane.append_marker(*marker);
        }

        Ok(lane)
    }
}