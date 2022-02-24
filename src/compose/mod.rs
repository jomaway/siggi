// compose/mod.rs
use svg::{self, node::element::{Group, Text, Path, Rectangle, Line, path::Data}, Node};

use crate::model::{Diagram, markers::{Label, TextAnchor}, Lane, utils::Color, Signal, signal::Level};

// Constants
const PADDING: f64 = 30.0;
const _PADDING_TOP: f64 = PADDING;
const _PADDING_BOTTOM: f64 = PADDING;
const _PADDING_LEFT: f64 = PADDING;
const _PADDING_RIGHT: f64 = PADDING;
const HEADER_HEIGHT: f64 = 25.0;  // if no title set to 0;
const FOOTER_HEIGHT: f64 = 0.0;

const WAVE_PERIOD_WIDTH: f64 = 50.0;
const WAVE_HEIGHT: f64 = WAVE_PERIOD_WIDTH;
const WAVE_PADDING: f64 = WAVE_HEIGHT/2.0;
const WAVE_PADDING_TOP: f64 = WAVE_PADDING;
const WAVE_PADDING_BOTTOM: f64 = WAVE_PADDING;
const LANE_HEIGHT: f64 = WAVE_HEIGHT + WAVE_PADDING_TOP + WAVE_PADDING_BOTTOM;

const DEFAULT_WAVE_OFFSET: f64 = WAVE_PERIOD_WIDTH * 1.5;
const MIN_WAVE_WIDTH: f64 = WAVE_PERIOD_WIDTH * 4.0;
const MIN_LANE_WIDTH: f64 = DEFAULT_WAVE_OFFSET + MIN_WAVE_WIDTH;

#[derive(Debug, Clone, Copy, Default)]
pub struct Compositor{
    config: Option<&'static str>
}

impl Compositor {

    pub fn new() -> Self { Self::default() }
    pub fn with(&mut self, config: &'static str) -> &mut Self {
        self.config = Some(config);
        self
    }

    pub fn compose(&self, diag: &Diagram) -> svg::Document {
        // calc document width an height
        let max_wave_width = get_max_wave_len(diag) as f64 * WAVE_PERIOD_WIDTH;
        let width = max_wave_width + DEFAULT_WAVE_OFFSET + PADDING * 2.0;  // todo!() calc correct wave offset.
        let height = diag.lane_count() as f64 * LANE_HEIGHT + HEADER_HEIGHT + FOOTER_HEIGHT + PADDING * 2.0;
        
        println!("Document size is {}x{}",width,height);
        println!("Longest wave found is {} periods", get_max_wave_len(diag));


        let bg = Rectangle::new()
            .set("id", "background")
            .set("fill", diag.background().to_string())
            .set("stroke", "none")
            .set("width", "100%")
            .set("height", "100%");

        let title = Label::new(diag.title());
        let title = Text::from(&title).translate(width/2.0,PADDING);

        let mut lanes = Group::new().set("id", "lanes");

        for (num,lane) in diag.lanes().iter().enumerate() {
            lanes.append(self.compose_lane(num, lane, max_wave_width).translate(0.0, num as f64 * LANE_HEIGHT));
        }


        svg::Document::new()
            .set("viewBox", (0,0,width,height))
            .add(bg)
            .add(title)
            .add(lanes.translate(PADDING, PADDING + HEADER_HEIGHT))
            
    }

    fn compose_lane(&self, num: usize, lane: &Lane, max_ww: f64) -> Group {
        println!("Compose lane-{}",num);
        // tile and y-axis label goes to the left
        // wave starts at wave_offset and goes till the end.
        let mut group = Group::new()
            .set("id", format!("lane-{}",num));

        let wave_offset = DEFAULT_WAVE_OFFSET;
        let wave_end = wave_offset + max_ww; // max_ww gets calculated at the top of consume().

        // compose y-axis labels
        let y_axis_label_heigh = Text::from(&Label::small(lane.sig.y_axis.0))
            .translate(wave_offset-10.0, WAVE_PADDING_TOP);
        let y_axis_label_low = Text::from(&Label::small(lane.sig.y_axis.1))
            .translate(wave_offset-10.0, WAVE_PADDING_TOP + WAVE_HEIGHT );


            // add posibility to crate a label from the title.
        let signal_name_label = Text::from(&signal_title_to_label(lane.sig.name, lane.sig.color)
            ).translate(wave_offset-15.0, WAVE_PADDING_TOP + WAVE_HEIGHT/2.0);

        group.append(y_axis_label_heigh);
        group.append(y_axis_label_low);
        group.append(signal_name_label);

        // compose dashed lane level lines
        group.append(self.compose_lane_level_lines(wave_offset, wave_end).set("id", format!("lane-level-lines-{}",num)));

        group.append(Path::from(&lane.sig)
            .set("id", format!("lane-{}-wave",num))
            .translate(wave_offset, WAVE_PADDING_TOP)
        );


        group
    }

    fn compose_lane_level_lines(&self, start: f64 , end: f64) -> Group {
        Group::new()
            .add(h_dashed_line(start, end, 0.0))
            .add(h_dashed_line(start, end, WAVE_HEIGHT/2.0))
            .add(h_dashed_line(start, end, WAVE_HEIGHT))
            .translate(0.0, WAVE_PADDING_TOP)
    }
}

// helper functions
fn get_max_wave_len(diagram: &Diagram) -> usize {
    if let Some(max_sig) = diagram.lanes().iter().map(|l| &l.sig ).max() {
        return max_sig.wave.len();
    } else {
        return 0;
    }
}

fn h_dashed_line(x1: f64,x2: f64,y: f64) -> Line {
    Line::new()
        .set("stroke", Color::Lightgray.to_string())
        .set("stroke-linecap", "round")
        .set("stroke-width", "0.5")
        .set("stroke-dasharray", "10 6")
        .set("x1", x1).set("y1", y)
        .set("x2", x2).set("y2", y)
}

fn signal_title_to_label(title: &'static str, color: Color) -> Label{
    let mut label = Label::new(title);
    label.set_anchor(TextAnchor::End);
    label.set_color(color);
    label
}


// Impl from model to svg elements. 

impl From<&Label> for Text {
    fn from(label: &Label) -> Text{
        Text::new()
            .set("fill", label.color().to_string())
            .set("font-family", "Segoe Print")
            .set("font-size", label.size().to_string())
            .set("text-anchor", label.anchor().to_string())
            .add(svg::node::Text::new(label.text()))
    }
}

impl From<&Signal> for Path {
    fn from(signal: &Signal) -> Self {
        Path::new()
        .set("fill", "none")
        .set("stroke", signal.color.to_string())
        .set("stroke-width", 3)
        .set("stroke-linejoin","round")
        .set("stroke-linecap", "round")
        .set("d", Data::from(signal))
    }
}

impl From<&Signal> for Data {
    // compose svg paths data from a signal
    fn from(sig: &Signal) -> Self {
        // Set start conditions. 
        let mut prev_value = match sig.wave.levels.first() {
            Some(Level::High) => Level::High,
            Some(Level::Down) => Level::High,
            _ => Level::Low,
        };

        //log::debug!("First value is {:?}", prev_value);
        
        let y = match prev_value {
            Level::Low => WAVE_HEIGHT,
            Level::High => 0.0,
            _ => WAVE_HEIGHT/2.0,
        };

        //log::debug!("Set y to {}", y);

        let mut data = Data::new()
            .move_to((0,y))
            .horizontal_line_by(sig.phase * WAVE_PERIOD_WIDTH);

            for value in sig.wave.levels.iter() {
                if &prev_value == value {
                    data = data.horizontal_line_by(WAVE_PERIOD_WIDTH  * sig.period);
                } else {
                    match value {
                        Level::Low => {
                            data = data
                            .vertical_line_by(WAVE_HEIGHT)
                            .horizontal_line_by(WAVE_PERIOD_WIDTH  * sig.period);
                            prev_value = Level::Low;
                        }
                        Level::High => {
                            data = data
                            .vertical_line_by(-(WAVE_HEIGHT))
                            .horizontal_line_by(WAVE_PERIOD_WIDTH * sig.period);
                            prev_value = Level::High;
                        }
                        Level::Up => {
                            data = data
                                .vertical_line_to(WAVE_HEIGHT)
                                .horizontal_line_by((WAVE_PERIOD_WIDTH/2.0)  * sig.period)
                                .vertical_line_by(-(WAVE_HEIGHT ))
                                .horizontal_line_by((WAVE_PERIOD_WIDTH/2.0)  * sig.period);
                            prev_value = Level::High;
                        }
                        Level::Down => {
                            data = data
                                .vertical_line_to(0)
                                .horizontal_line_by((WAVE_PERIOD_WIDTH/2.0)  * sig.period)
                                .vertical_line_by(WAVE_HEIGHT)
                                .horizontal_line_by((WAVE_PERIOD_WIDTH/2.0)  * sig.period);
                            prev_value = Level::Low;
                        }
                        _ => { unimplemented!()}
                    }
                }            
            }
        data
    }
}

// Transformable

pub trait Transformable
{
    fn translate(self, x:f64, y:f64) -> Self;
}

impl Transformable for Group {
    fn translate(self, x:f64, y:f64) -> Self {
        self.set("transform", format!("translate({},{})",x,y))
    }
}

impl Transformable for Path {
    fn translate(self, x:f64, y:f64) -> Self {
        self.set("transform", format!("translate({},{})",x,y))
    }
}

impl Transformable for Text {
    fn translate(self, x:f64, y:f64) -> Self {
        self.set("transform", format!("translate({},{})",x,y))
    }
}