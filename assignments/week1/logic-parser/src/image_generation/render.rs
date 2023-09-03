use image::{RgbImage, Rgb, ImageResult};
use rusttype::{Font, Scale};
use imageproc::drawing::{draw_text_mut, draw_hollow_circle_mut};

use crate::parsing::ASTNode;

static BLACK: Rgb<u8> = Rgb([0, 0, 0]);
static CIRCLE_RADIUS: i32 = 100;

thread_local! {
    static FONT_FAMILY: Font<'static> = {
        let font_data = include_bytes!("../static/RobotoSlab.ttf");
        Font::try_from_bytes(font_data).expect("Error constructing Font")
    };
}

pub fn render_to_image(ast: ASTNode) -> ImageResult<RgbImage> {
    let mut img = RgbImage::from_pixel(40, 40, Rgb([235, 191, 188]));
    let scale = Scale { x: 20f32, y: 20f32 };
    let children_stack: Vec<ASTNode> = Vec::with_capacity(4);

    loop {
        for child in children_stack {

        }
    }
    // let img = FONT_FAMILY.with(move |font| {
    //     draw_text_mut(&mut img, BLACK, 5, 10, scale, font, "test");
    //     draw_hollow_circle_mut(&mut img, (0, 1), 5, BLACK);
    //     img
    // });

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
