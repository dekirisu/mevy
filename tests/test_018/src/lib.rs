use bevy::prelude::*;
use shared::shared_tests;
use mevy::*;

shared_tests!();

// Section 12: `ui!{}` — Tuple Mode: OverflowClipBox variant (0.18 only)

#[test]
fn ui_tuple_node_overflow_clip_margin_content_box() {
    let node = ui!((
        overflow_clip_margin: content_box;
    ));
    assert_eq!(node.overflow_clip_margin.visual_box, OverflowClipBox::ContentBox);
}

// Section 14: `ui!{}` — Tuple Mode: Text Styling (font_size f32 variant)

#[test]
fn ui_tuple_text_size() {
    let text_font = ui!((
        text_size: 20;
    ));
    assert_eq!(text_font.font_size, 20.0);
}
