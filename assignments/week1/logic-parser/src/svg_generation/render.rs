use crate::parsing::ASTNode;
use super::svg::Svg;

/// Returns the tree depth (>= 1)
fn ast_depth(ast: &ASTNode) -> u32 {
    match ast {
        ASTNode::And { left, right } |
        ASTNode::Or { left, right } |
        ASTNode::Implies { left, right } |
        ASTNode::IfAndOnlyIf { left, right } => {
            let l = ast_depth(&left) + 1;
            let r = ast_depth(&right) + 1;
            std::cmp::max(l, r)
        },
        ASTNode::Not { operand } => {
            ast_depth(&operand) + 1
        },
        _ => 1
    }
}

static FONT_SIZE: u32 = 12;

/// The rendered tree is `tree(s, h, r)` where:
///
/// ```md
///         —— (r)adius
///       (  )     ╻
///       /  \     │
///      /    \    │ (h)eight
///     /      \   │
///   (  )    (  ) ╹
///         ————
///          (s)eparation
/// ```
///
/// It is highly recommended that you choose `s >= r` and `h >= 2r`.
///
/// The rendered image dimensions will be `width`x`height` where:
/// - `width` = `s(2^n - 2) + 2r`
/// - `height` = `h(n - 1) + 2r`
///
/// Where:
/// - `n` = The depth of the tree
pub fn render_to_svg(ast: ASTNode, s: f32, h: f32, r: f32) -> Svg {
    let n = ast_depth(&ast) as i32; // >= 1
    let middle_grid = f32::powi(2f32, n - 1) as u32 - 1;
    let padding = r + 60_f32;

    let width = 2f32 * middle_grid as f32 * s + (padding * 2f32);
    let height = h * (n - 1) as f32 + (padding * 2f32);

    let get_real_xy = |grid_x: u32, grid_y: u32| {
        let x: f32 = grid_x as f32 * s + padding;
        let y: f32 = grid_y as f32 * h + padding;

        (x, y)
    };

    let mut img = Svg::new((0_f32, 0_f32, width, height));
    let mut stack: Vec<(&ASTNode, u32, u32)> = vec![(&ast, middle_grid, 0)];

    loop {
        if stack.is_empty() {
            break;
        }
        let (node, grid_x, grid_y) = stack.pop().unwrap();
        let pos = get_real_xy(grid_x, grid_y);

        let next_step = u32::pow(2, (n as u32) - (grid_y + 1)) / 2;
        img.draw_circle_with_text(pos, r, node.repr(), FONT_SIZE);

        match node {
            ASTNode::And { left, right } |
            ASTNode::Or { left, right } |
            ASTNode::Implies { left, right } |
            ASTNode::IfAndOnlyIf { left, right } => {
                stack.push((left, grid_x - next_step, grid_y + 1));
                stack.push((right, grid_x + next_step, grid_y + 1));

                let left_to = get_real_xy(grid_x - next_step, grid_y + 1);
                let right_to = get_real_xy(grid_x + next_step, grid_y + 1);
                img.draw_line_with_offset(pos, left_to, r);
                img.draw_line_with_offset(pos, right_to, r);
            },
             ASTNode::Not { operand } => {
                stack.push((operand, grid_x, grid_y + 1));
                img.draw_line_with_offset(pos, get_real_xy(grid_x, grid_y + 1), r);
            },
            _ => {}
        }
    }

    img
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

    #[test]
    fn svg_has_correct_amount_of_lines() -> Result<(), Box<dyn Error>> {
        let tokens = crate::lexing::Lexer::new("p").parse()?;
        let ast = crate::parsing::Parser::new(&tokens).parse()?;
        let horizontal_separation: f32 = 20_f32;
        let vertical_separation: f32 = 30_f32;
        let radius: f32 = 15_f32;
        assert_eq!(
            render_to_svg(
                ast,
                horizontal_separation,
                vertical_separation,
                radius
            ).render().lines().count(),
            2
        );

        Ok(())
    }
}
