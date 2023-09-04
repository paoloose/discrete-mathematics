pub struct Svg {
    view_box: (f32, f32, f32, f32),
    font: &'static str,
    stroke_width: u32,
    stroke: &'static str, // <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke>
    elements: Vec<String>
}

impl Svg {
    pub fn new(view_box: (f32, f32, f32, f32)) -> Svg {
        Svg {
            view_box,
            stroke_width: 1,
            font: "sans-serif",
            stroke: "black",
            elements: vec![]
        }
    }

    pub fn draw_circle(&mut self, pos: (f32, f32), radius: f32) {
        self.elements.push(
            format!(
                r#"<circle cx="{cx}" cy="{cy}" r="{r}" stroke="{stroke}" stroke-width="{sw}" fill="none" />"#,
                cx = pos.0,
                cy = pos.1,
                r = radius,
                stroke = self.stroke,
                sw = self.stroke_width
            )
        );
    }

    pub fn draw_line(&mut self, start: (f32, f32), end: (f32, f32)) {
        self.elements.push(
            format!(
                r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="black" stroke="{stroke}" stroke-width="{sw}" />"#,
                x1 = start.0,
                y1 = start.1,
                x2 = end.0,
                y2 = end.1,
                stroke = self.stroke,
                sw = self.stroke_width
            )
        );
    }

    pub fn draw_line_with_offset(&mut self, start: (f32, f32), end: (f32, f32), offset: f32) {
        let line_length = f32::sqrt(f32::powi(end.0 - start.0, 2) + f32::powi(end.1 - start.1, 2));

        self.elements.push(
            format!(
                r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="black" stroke="{stroke}" stroke-width="{sw}" stroke-dasharray="{darr} {offset}" stroke-dashoffset="-{offset}" />"#,
                x1 = start.0,
                y1 = start.1,
                x2 = end.0,
                y2 = end.1,
                stroke = self.stroke,
                sw = self.stroke_width,
                darr = line_length - (2f32 * offset)
            )
        );
    }

    pub fn draw_text(&mut self, pos: (f32, f32), text: &str, font_size: u32) {
        self.elements.push(
            format!(
                r#"<text x="{x}" y="{y}" font-family="{font}" font-size="{font_size}">{text}</text>"#,
                x = pos.0,
                y = pos.1,
                font = self.font,
                text = text,
                font_size = font_size
            )
        );
    }

    pub fn draw_circle_with_text(&mut self, pos: (f32, f32), radius: f32, text: &str, font_size: u32) {
        self.draw_circle(pos, radius);
        self.elements.push(
            format!(
                r#"<text x="{x}" y="{y}" font-family="{font}" font-size="{font_size}" text-anchor="middle" alignment-baseline="central">{text}</text>"#,
                x = pos.0,
                y = pos.1,
                font = self.font,
                text = text,
                font_size = font_size
            )
        );
    }

    pub fn as_xml(&self) -> String {
        format!(
            r#"<svg viewBox="{vb}" xmlns="http://www.w3.org/2000/svg">{elements}</svg>"#,
            vb = format!("{x} {y} {w} {h}", x = self.view_box.0, y = self.view_box.1, w = self.view_box.2, h = self.view_box.3),
            elements = self.elements.join("\n")
        )
    }
}
