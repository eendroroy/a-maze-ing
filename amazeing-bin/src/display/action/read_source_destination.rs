use crate::context::DrawContext;
use crate::helper::get_node_from_mouse_pos;
use amazeing::DNode;
use macroquad::input::{is_key_down, KeyCode};

pub(crate) fn populate_source_destination(
    ctx: &DrawContext,
    from: &mut Option<DNode>,
    to: &mut Option<DNode>,
) {
    let (r, c) = get_node_from_mouse_pos(ctx);
    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
        *to = Some((r, c));
    } else {
        *from = Some((r, c));
    }
}
