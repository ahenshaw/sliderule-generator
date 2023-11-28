use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::Text;
use svg::Document;
use svg::Node;

enum Division {
    Primary,
    // Secondary,
    // Tertiary,
}

pub enum Transform {
    Linear,
    Log,
}

fn linear(x: f32) -> f32 {
    x
}

fn log(x: f32) -> f32 {
    x.log10()
}

struct Scale {
    transform: Transform,
    _label: String,
}

impl Scale {
    fn new(label: &str, transform: Transform) -> Self {
        Self {
            _label: label.to_string(),
            transform,
        }
    }

    fn generate(
        &self,
        _division: Division,
        start: f32,
        end: f32,
        step: f32,
        width: f32,
    ) -> Vec<Box<dyn Node>> {
        let mut x = start;
        let mut data = Data::new();
        let mut nodes = Vec::<Box<dyn Node>>::new();
        while x <= end {
            let tx = match self.transform {
                Transform::Linear => linear(x),
                Transform::Log => log(x),
            };
            data = data.move_to((width * tx, 0.0)).line_by((0.0, 5.0));
            nodes.push(Box::new(Text::new(format!("{x}"))));
            x += step;
        }
        data = data.close();

        nodes.push(Box::new(
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.1)
                .set("d", data),
        ));
        nodes
    }
}

fn main() {
    let mut document = Document::new().set("viewBox", (0, 0, 250, 10));
    let scale = Scale::new("L", Transform::Linear);
    let nodes = scale.generate(Division::Primary, 0.0, 10.0, 1.0, 25.0);
    for node in nodes {
        document = document.add(node);
    }

    svg::save("image.svg", &document).unwrap();
}
