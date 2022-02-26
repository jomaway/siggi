// compose/mod.rs
use svg::{self, node::element::{Group, Text, Path, Rectangle, Line, path::Data}, Node};

use crate::model::{Diagram, markers::{Label, TextAnchor, Line as LineMarker, Marker}, Lane, utils::Color, Signal, signal::Level};

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
#[allow(unused)]
const MIN_WAVE_WIDTH: f64 = WAVE_PERIOD_WIDTH * 4.0;
#[allow(unused)]
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

        let title = Label::from(diag.title());
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
        let y_axis_label_heigh = Text::from(&Label::from(lane.signal.y_axis.0.to_string()).small())
            .translate(wave_offset-10.0, WAVE_PADDING_TOP);
        let y_axis_label_low = Text::from(&Label::from(lane.signal.y_axis.1.to_string()).small())
            .translate(wave_offset-10.0, WAVE_PADDING_TOP + WAVE_HEIGHT );

        // todo!() add posibility to crate a label from the title.
        let signal_name_label = Text::from(&signal_title_to_label(lane.signal.name.to_string(), lane.signal.color)
            ).translate(wave_offset-15.0, WAVE_PADDING_TOP + WAVE_HEIGHT/2.0);

        group.append(y_axis_label_heigh);
        group.append(y_axis_label_low);
        group.append(signal_name_label);

        println!("... y labels composed.");

        // compose dashed lane level lines
        group.append(self.compose_lane_level_lines(wave_offset, wave_end).set("id", format!("lane-{}-level-lines",num)));

        println!("... lane lines composed.");

        group.append(Path::from(&lane.signal)
            .set("id", format!("lane-{}-wave",num))
            .translate(wave_offset, WAVE_PADDING_TOP)
        );
        println!("... signal composed.");

        group.append(self.compose_lane_markers(&lane.markers)
            .set("id",format!("lane-{}-markers",num))
            .translate(wave_offset, 0.0));
        println!("... markers composed.");
        
        // compose labels at the bottom
        group.append(self.compose_lane_labels(&lane.labels)
            .set("id",format!("lane-{}-labels",num))
            .translate(wave_offset, LANE_HEIGHT));
        println!("... labels composed.");

        group
    }

    fn compose_lane_level_lines(&self, start: f64 , end: f64) -> Group {
        Group::new()
            .add(h_dashed_line(start, end, 0.0))
            .add(h_dashed_line(start, end, WAVE_HEIGHT/2.0))
            .add(h_dashed_line(start, end, WAVE_HEIGHT))
            .translate(0.0, WAVE_PADDING_TOP)
    }

    fn compose_lane_markers(&self, markers: &Vec<LineMarker>) -> Group {
        let top_y = WAVE_PADDING_TOP/2.0;
        let bottom_y = WAVE_PADDING_TOP + WAVE_HEIGHT + WAVE_PADDING_BOTTOM/2.0;

        let mut group = Group::new();
        for marker in markers {
            group.append(v_dashed_line(marker.position()*WAVE_PERIOD_WIDTH, top_y, bottom_y).with_color(marker.color).with_size(marker.thickness));
        }
        group
    }

    fn compose_lane_labels(&self, labels: &Vec<Label>) -> Group 
    {
        let mut group = Group::new();
        for label in labels {
            group.append(Text::from(label).translate(label.position() * WAVE_PERIOD_WIDTH, 0.0));
        }
        group
    }
}

// helper functions
fn get_max_wave_len(diagram: &Diagram) -> u32 {
    if let Some(max_sig) = diagram.lanes().iter().map(|l| &l.signal ).max() {
        return max_sig.len();
    } else {
        return 0;
    }
}

fn h_dashed_line(x1: f64,x2: f64,y: f64) -> Line {
    Line::new()
        .set("x1", x1).set("y1", y)
        .set("x2", x2).set("y2", y)
        .with_color(Color::Lightgray)
        .rounded()
        .dash("10 6")
        .with_size(0.5)
}

fn v_dashed_line(x: f64, y1: f64,y2: f64) -> Line {
    Line::new()
        .set("x1", x).set("y1", y1)
        .set("x2", x).set("y2", y2)
        .with_color(Color::Lightgray)
        .with_size(1.0)
        .dash("3 3")
        .rounded()
}

fn signal_title_to_label(title: String, color: Color) -> Label{
    Label::from(title).align(TextAnchor::End).color_with(color)
}


// Impl from model to svg elements. 

impl From<&Label> for Text {
    fn from(label: &Label) -> Text{
        Text::new()
            .with_color(label.color)
            .align_to(label.anchor)
            .set("font-family", "Segoe Print")
            .set("font-size", label.size.to_string())
            .add(svg::node::Text::new(label.text.clone()))
    }
}

impl From<&Signal> for Path {
    fn from(signal: &Signal) -> Self {
        Path::new()
        .set("fill", "none")
        .set("stroke-width", 3)
        .with_color(signal.color)
        .rounded()
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

// Extend svg::Node trait with an color
pub trait Colored: svg::Node {
    fn with_color(self, color:Color) -> Self;
}

impl Colored for Line {
    fn with_color(self, color:Color) -> Self  {
        self.set("stroke", color.to_string())
    }
}

impl Colored for Text {
    fn with_color(self, color:Color) -> Self  {
        self.set("fill", color.to_string())
    } 
}

impl Colored for Path {
    fn with_color(self, color:Color) -> Self {
        self.set("stroke", color.to_string())
    }
}

pub trait Aligned: svg::Node {
    fn align_to(self, anchor: TextAnchor) -> Self;
}

impl Aligned for Text {
    fn align_to(self, anchor: TextAnchor) -> Self {
        self.set("text-anchor", anchor.to_string())
    }
}

pub trait Sized: svg::Node {
    fn with_size(self, size: f64) -> Self;
}

impl Sized for Text {
    fn with_size(self, size: f64) -> Self {
        self.set("font-size", size.to_string())
    }
}

impl Sized for Line {
    fn with_size(self, size: f64) -> Self {
        self.set("stroke-width", size.to_string())
    }
}

pub trait Dashed: svg::Node {
    fn dash(self, value: &str) -> Self;
}

impl Dashed for Line {
    fn dash(self, value: &str) -> Self {
        self.set("stroke-dasharray", value)
    }
}

pub trait Rounded: svg::Node {
    fn rounded(self) -> Self;
}

impl Rounded for Line {
    fn rounded(self) -> Self {
        self.set("stroke-linejoin","round")
        .set("stroke-linecap", "round")
    }
}

impl Rounded for Path {
    fn rounded(self) -> Self {
        self.set("stroke-linejoin","round")
        .set("stroke-linecap", "round")
    }
}