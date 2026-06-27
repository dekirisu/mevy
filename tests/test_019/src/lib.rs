use bevy::prelude::*;
#[allow(unused_imports)]
use bevy::text::{FontSource, FontSize, LetterSpacing, EditableText};
#[allow(unused_imports)]
use bevy::ui::InlineDirection;
use shared::shared_tests;
use mevy::*;

shared_tests!();

// Section 12: `ui!{}` — Tuple Mode: OverflowClipBox variant (0.19 only) \\

#[test]
fn ui_tuple_node_overflow_clip_margin_content_box() {
    let node = ui!((
        overflow_clip_margin: content_box;
    ));
    assert_eq!(node.overflow_clip_margin.visual_box, VisualBox::ContentBox);
}

// Section 14: `ui!{}` — Tuple Mode: Text Styling (font_size enum variant) \\

#[test]
fn ui_tuple_text_size() {
    let text_font = ui!((
        text_size: 20;
    ));
    assert_eq!(text_font.font_size, FontSize::Px(20.0));
}

// Section 17: `ui!{}` — Bevy 0.19 Specific Fields \\
// ================================================== \\

#[test]
fn ui_019_editable_text() {
    let et = ui!((
        edit;
    ));
    let default = EditableText::default();
    // cursor_width is a public field that we can compare to verify the macro produced a valid EditableText
    assert_eq!(et.cursor_width, default.cursor_width);
}

#[test]
fn ui_019_direction() {
    let node = ui!((
        direction: ltr;
    ));
    assert_eq!(node.direction, InlineDirection::Ltr);
}

#[test]
fn ui_019_font_source() {
    let tf = ui!((
        font_source: sans_serif;
    ));
    assert!(matches!(tf.font, FontSource::SansSerif));
}

#[test]
fn ui_019_letter_spacing() {
    let ls = ui!((
        letter_spacing: 4;
    ));
    assert_eq!(ls, LetterSpacing::Px(4.0));
}

#[test]
fn ui_019_font_size_enum() {
    let tf = ui!((
        font_size: 24;
    ));
    assert_eq!(tf.font_size, FontSize::Px(24.0));
}
