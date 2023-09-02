use image::{RgbImage, Rgb, ImageResult};
use rusttype::{Font, Scale};
use imageproc::drawing::draw_text_mut;

use crate::parsing::ASTNode;

static FONT_COLOR: Rgb<u8> = Rgb([0, 0, 0]);

thread_local! {
    static FONT_FAMILY: Font<'static> = {
        let font_data = include_bytes!("../static/RobotoSlab.ttf");
        Font::try_from_bytes(font_data).expect("Error constructing Font")
    };
}

pub fn render_to_image(ast: ASTNode) -> ImageResult<RgbImage> {
    let mut img = RgbImage::from_pixel(40, 40, Rgb([235, 191, 188]));
    let scale = Scale { x: 20f32, y: 20f32 };

    FONT_FAMILY.with(|font| {
        draw_text_mut(&mut img, FONT_COLOR, 5, 10, scale, font, "uwu");
    });

    img.save("uwu.png")?;
    Ok(img)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::*;

    #[test]
    fn image_is_saved() {
        let ast = ASTNode::Identifier { name: "heelo".into() };
        render_to_image(ast).unwrap();

        assert!(Path::new("uwu.png").exists());
    }
}
