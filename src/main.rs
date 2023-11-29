use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::element::Text;
use svg::Document;
use svg::Node;

enum Division {
    Primary,
    Secondary,
    // Tertiary,
}

pub enum Transform {
    Linear,
    Log,
}

const BASE_TICK_LENGTH: f32 = 20.0;

pub struct Scale {
    transform: Transform,
    _label: String,
    start: f32,
    end: f32,
    width: f32,
    x_offset: f32,
    y_offset: f32,
    flip: bool,
}

impl Scale {
    pub fn new(label: &str, transform: Transform, start: f32, end: f32, width: f32) -> Self {
        Self {
            _label: label.to_string(),
            transform,
            start,
            end,
            width,
            x_offset: 0.0,
            y_offset: 0.0,
            flip: false,
        }
    }

    pub fn with_x_offset(mut self, offset: f32) -> Self {
        self.x_offset = offset;
        self
    }

    pub fn with_y_offset(mut self, offset: f32) -> Self {
        self.y_offset = offset;
        self
    }

    pub fn with_flip(mut self, flip: bool) -> Self {
        self.flip = flip;
        self
    }

    fn linear(&self, x: f32) -> f32 {
        let range = self.end - self.start;
        let s = self.width / range;
        s * (x - self.start) + self.x_offset
    }

    fn log(&self, x: f32) -> f32 {
        let range = self.end.log10() - self.start.log10();
        let s = self.width / range;
        s * (x.log10() - self.start.log10())
    }

    fn generate(
        &self,
        division: Division,
        start: f32,
        end: f32,
        ticks: usize,
    ) -> Vec<Box<dyn Node>> {
        let mut data = Data::new();
        let mut nodes = Vec::<Box<dyn Node>>::new();

        let spacing = (end - start) / (ticks as f32);
        let range = match division {
            Division::Primary => 0..ticks + 1,
            _ => 1..ticks,
        };

        for i in range {
            let length = match (&division, i) {
                (Division::Primary, _) => BASE_TICK_LENGTH,
                (_, 5) => BASE_TICK_LENGTH * 0.75,
                _ => BASE_TICK_LENGTH * 0.5,
            };
            let length = if self.flip { -length } else { length };
            let x = start + (i as f32) * spacing;
            let tx = match self.transform {
                Transform::Linear => self.linear(x),
                Transform::Log => self.log(x),
            };
            data = data.move_to((tx, self.y_offset)).line_by((0.0, length));
            match division {
                Division::Primary => {
                    nodes.push(Box::new(
                        Text::new()
                            .set("x", tx)
                            .set("y", self.y_offset + length * 1.2)
                            .set("font-size", "8")
                            .set("font-family", "Nimbus Sans Narrow")
                            .set("text-anchor", "middle")
                            .set("dominant-baseline", "central")
                            .add(svg::node::Text::new(format!("{x}"))),
                    ));
                    if x < self.end {
                        for node in self.generate(Division::Secondary, x, x + spacing, 10) {
                            nodes.push(node);
                        }
                    }
                }
                _ => (),
            }
        }
        data = data.close();

        nodes.push(Box::new(
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.2)
                .set("d", data),
        ));
        nodes
    }
}

fn main() {
    let output_path = "output/image.svg";
    let mut document = Document::new().set("viewBox", (-10, -100, 600, 200));
    let c_scale = Scale::new("C", Transform::Log, 1.0, 10.0, 500.0).with_flip(true);
    let l_scale = Scale::new("L", Transform::Linear, 0.0, 10.0, 500.0).with_y_offset(0.1);

    for node in c_scale.generate(Division::Primary, 1.0, 10.0, 9) {
        document = document.add(node);
    }

    for node in l_scale.generate(Division::Primary, 0.0, 10.0, 10) {
        document = document.add(node);
    }

    match svg::save(&output_path, &document) {
        Ok(()) => println!("SVG written to '{output_path}'"),
        Err(e) => eprintln!("Failed to save to '{output_path}'\nReason: {e}"),
    }
}
