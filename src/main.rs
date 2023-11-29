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

struct Scale {
    transform: Transform,
    _label: String,
    start: f32,
    end: f32,
    width: f32,
}

impl Scale {
    fn new(label: &str, transform: Transform, start: f32, end: f32, width: f32) -> Self {
        Self {
            _label: label.to_string(),
            transform,
            start,
            end,
            width,
        }
    }

    fn linear(&self, x: f32) -> f32 {
        let range = self.end - self.start;
        let s = self.width / range;
        s * (x - self.start)
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
        step: f32,
        length: f32,
    ) -> Vec<Box<dyn Node>> {
        let mut data = Data::new();
        let mut nodes = Vec::<Box<dyn Node>>::new();
        let end = match division {
            Division::Primary => end + 0.001,
            _ => end - 0.001,
        };
        let mut x = match division {
            Division::Primary => start,
            _ => start + step,
        };

        while x < end {
            let tx = match self.transform {
                Transform::Linear => self.linear(x),
                Transform::Log => self.log(x),
            };
            data = data.move_to((tx, 0.0)).line_by((0.0, length));
            match division {
                Division::Primary => {
                    nodes.push(Box::new(
                        Text::new()
                            .set("x", tx)
                            .set("y", length + 5.0)
                            .set("font-size", "8")
                            .set("font-family", "Arial")
                            .set("text-anchor", "middle")
                            .set("dominant-baseline", "central")
                            .add(svg::node::Text::new(format!("{x}"))),
                    ));
                    if x < self.end {
                        for node in self.generate(
                            Division::Secondary,
                            x,
                            x + step,
                            step / 10.0,
                            length / 2.0,
                        ) {
                            nodes.push(node);
                        }
                    }
                }
                _ => (),
            }

            x += step;
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
    let mut document = Document::new().set("viewBox", (-10, -20, 520, 100));
    let scale = Scale::new("L", Transform::Log, 1.0, 10.0, 500.0);

    let nodes = scale.generate(Division::Primary, 1.0, 10.0, 1.0, 20.0);
    for node in nodes {
        document = document.add(node);
    }

    svg::save("output/image.svg", &document).unwrap();
}
