use bevy::prelude::*;
#[allow(unused_imports)]
use bevy::text::LineHeight;
#[allow(unused_imports)]
use bevy::ui::FocusPolicy;
use mevy::*;

// Section 1: `code!{}` — Hex Colors \\
// ==================================== \\

#[test]
fn code_hex_rgb_short() {
    let c = code!{#f00};
    let srgba = c.to_srgba();
    assert_eq!(srgba.red, 1.0);
    assert_eq!(srgba.green, 0.0);
    assert_eq!(srgba.blue, 0.0);
    assert_eq!(srgba.alpha, 1.0);
}

#[test]
fn code_hex_rgba_short() {
    let c = code!{#f00a};
    let srgba = c.to_srgba();
    assert_eq!(srgba.red, 1.0);
    assert_eq!(srgba.green, 0.0);
    assert_eq!(srgba.blue, 0.0);
    // #f00a expands to #ff0000aa, so alpha = 0xaa/255 = 170/255
    assert!((srgba.alpha - 170.0/255.0).abs() < 0.01);
}

#[test]
fn code_hex_rgb() {
    let c = code!{#ff0000};
    let srgba = c.to_srgba();
    assert_eq!(srgba.red, 1.0);
    assert_eq!(srgba.green, 0.0);
    assert_eq!(srgba.blue, 0.0);
}

#[test]
fn code_hex_rgba() {
    let c = code!{#ff0000ff};
    let srgba = c.to_srgba();
    assert_eq!(srgba.red, 1.0);
    assert_eq!(srgba.green, 0.0);
    assert_eq!(srgba.blue, 0.0);
    assert_eq!(srgba.alpha, 1.0);
}

#[test]
fn code_hex_rgba_partial() {
    let c = code!{#ff000080};
    let srgba = c.to_srgba();
    assert_eq!(srgba.alpha, 128.0 / 255.0);
}

#[test]
fn code_hex_uppercase() {
    let c1 = code!{#FF0000};
    let c2 = code!{#ff0000};
    assert_eq!(c1.to_srgba().red, c2.to_srgba().red);
    assert_eq!(c1.to_srgba().green, c2.to_srgba().green);
    assert_eq!(c1.to_srgba().blue, c2.to_srgba().blue);
}

#[test]
fn code_hex_mixed_case() {
    let c = code!{#Ff00Aa};
    let srgba = c.to_srgba();
    assert_eq!(srgba.red, 1.0);
    assert_eq!(srgba.green, 0.0);
    assert!((srgba.blue - 170.0/255.0).abs() < 0.001);
    assert_eq!(srgba.alpha, 1.0);
}

#[test]
fn code_hex_short_expands_correctly() {
    let c = code!{#abc};
    let srgba = c.to_srgba();
    assert_eq!(srgba.red, 170.0 / 255.0);
    assert_eq!(srgba.green, 187.0 / 255.0);
    assert_eq!(srgba.blue, 204.0 / 255.0);
}

#[test]
fn code_hex_short_alpha_expands_correctly() {
    let c = code!{#abcd};
    let srgba = c.to_srgba();
    assert_eq!(srgba.red, 170.0 / 255.0);
    assert_eq!(srgba.green, 187.0 / 255.0);
    assert_eq!(srgba.blue, 204.0 / 255.0);
    assert_eq!(srgba.alpha, 221.0 / 255.0);
}

#[test]
fn code_hex_color_method_chaining() {
    let c = code!{#FF1265.mix(&#F93ECA, 0.4).with_alpha(0.2)};
    let srgba = c.to_srgba();
    assert_eq!(srgba.alpha, 0.2);
}

// Section 2: `code!{}` — CSS-like Val Units \\
// =========================================== \\

#[test]
fn code_val_px() {
    let v = code!{100px};
    match v {
        Val::Px(val) => assert_eq!(val, 100.0),
        _ => panic!("expected Val::Px, got {:?}", v),
    }
}

#[test]
fn code_val_negative_px() {
    let v = code!{-50px};
    match v {
        Val::Px(val) => assert_eq!(val, -50.0),
        _ => panic!("expected Val::Px, got {:?}", v),
    }
}

#[test]
fn code_val_zero_px() {
    let v = code!{0px};
    match v {
        Val::Px(val) => assert_eq!(val, 0.0),
        _ => panic!("expected Val::Px, got {:?}", v),
    }
}

#[test]
fn code_val_percent() {
    let v = code!{50%};
    match v {
        Val::Percent(val) => assert_eq!(val, 50.0),
        _ => panic!("expected Val::Percent, got {:?}", v),
    }
}

#[test]
fn code_val_zero_percent() {
    let v = code!{0%};
    match v {
        Val::Percent(val) => assert_eq!(val, 0.0),
        _ => panic!("expected Val::Percent, got {:?}", v),
    }
}

#[test]
fn code_val_vw() {
    let v = code!{3vw};
    match v {
        Val::Vw(val) => assert_eq!(val, 3.0),
        _ => panic!("expected Val::Vw, got {:?}", v),
    }
}

#[test]
fn code_val_vh() {
    let v = code!{2.5vh};
    match v {
        Val::Vh(val) => assert_eq!(val, 2.5),
        _ => panic!("expected Val::Vh, got {:?}", v),
    }
}

#[test]
fn code_val_vmin() {
    let v = code!{1vmin};
    match v {
        Val::VMin(val) => assert_eq!(val, 1.0),
        _ => panic!("expected Val::VMin, got {:?}", v),
    }
}

#[test]
fn code_val_vmax() {
    let v = code!{4vmax};
    match v {
        Val::VMax(val) => assert_eq!(val, 4.0),
        _ => panic!("expected Val::VMax, got {:?}", v),
    }
}

#[test]
fn code_val_auto() {
    let v = code!{@};
    match v {
        Val::Auto => {},
        _ => panic!("expected Val::Auto, got {:?}", v),
    }
}

#[test]
fn code_val_float_px() {
    let v = code!{3.1415px};
    match v {
        Val::Px(val) => assert!((val - 3.1415).abs() < 0.0001),
        _ => panic!("expected Val::Px, got {:?}", v),
    }
}

#[test]
fn code_val_negative_percent() {
    let v = code!{-25%};
    match v {
        Val::Percent(val) => assert_eq!(val, -25.0),
        _ => panic!("expected Val::Percent, got {:?}", v),
    }
}

// Section 3: `code!{}` — `!` Syntax (Default) \\
// ============================================== \\

#[test]
fn code_default_in_struct() {
    let n = code!{Node {
        width: 80px,
        height: 80px,
        ..default()
    }};
    assert_eq!(n.width, Val::Px(80.0));
    assert_eq!(n.height, Val::Px(80.0));
}

// Section 4: `code!{}` — UiRect \\
// ============================== \\

#[test]
fn code_ui_rect_single() {
    let r = code!{[>10px]};
    assert_eq!(r.left, Val::Px(10.0));
    assert_eq!(r.right, Val::Px(10.0));
    assert_eq!(r.top, Val::Px(10.0));
    assert_eq!(r.bottom, Val::Px(10.0));
}

#[test]
fn code_ui_rect_two() {
    let r = code!{[>10px 5px]};
    assert_eq!(r.top, Val::Px(10.0));
    assert_eq!(r.bottom, Val::Px(10.0));
    assert_eq!(r.left, Val::Px(5.0));
    assert_eq!(r.right, Val::Px(5.0));
}

#[test]
fn code_ui_rect_three() {
    let r = code!{[>10px 5px 3px]};
    assert_eq!(r.top, Val::Px(10.0));
    assert_eq!(r.bottom, Val::Px(3.0));
    assert_eq!(r.left, Val::Px(5.0));
    assert_eq!(r.right, Val::Px(5.0));
}

#[test]
fn code_ui_rect_four() {
    let r = code!{[>10px 5px 3px 1px]};
    assert_eq!(r.top, Val::Px(10.0));
    assert_eq!(r.right, Val::Px(5.0));
    assert_eq!(r.bottom, Val::Px(3.0));
    assert_eq!(r.left, Val::Px(1.0));
}

#[test]
fn code_ui_rect_percent() {
    let r = code!{[>10% 5% 0% 3%]};
    assert_eq!(r.top, Val::Percent(10.0));
    assert_eq!(r.right, Val::Percent(5.0));
    assert_eq!(r.bottom, Val::Percent(0.0));
    assert_eq!(r.left, Val::Percent(3.0));
}

#[test]
fn code_ui_rect_zero() {
    let r = code!{[>0px]};
    assert_eq!(r.left, Val::Px(0.0));
    assert_eq!(r.right, Val::Px(0.0));
    assert_eq!(r.top, Val::Px(0.0));
    assert_eq!(r.bottom, Val::Px(0.0));
}

#[test]
fn code_ui_rect_mixed_units() {
    let r = code!{[>10px 5% 3vh 2vw]};
    assert_eq!(r.top, Val::Px(10.0));
    assert_eq!(r.right, Val::Percent(5.0));
    assert_eq!(r.bottom, Val::Vh(3.0));
    assert_eq!(r.left, Val::Vw(2.0));
}

// Section 5: `code!{}` — Struct Literals \\
// ======================================== \\

#[test]
fn code_node_struct() {
    let n = code!{Node {
        width: 80px,
        height: 80px,
        margin: [>16px],
        ..default()
    }};
    assert_eq!(n.width, Val::Px(80.0));
    assert_eq!(n.height, Val::Px(80.0));
    assert_eq!(n.margin.left, Val::Px(16.0));
    assert_eq!(n.margin.right, Val::Px(16.0));
    assert_eq!(n.margin.top, Val::Px(16.0));
    assert_eq!(n.margin.bottom, Val::Px(16.0));
}

#[test]
fn code_box_shadow_struct() {
    let shadow = code!{BoxShadow(vec![ShadowStyle {
        color: #FF1265.mix(&#F93ECA, 0.4).with_alpha(0.2),
        x_offset: 100px,
        y_offset: 50%,
        spread_radius: 3.1vh,
        blur_radius: 40.23vmax,
    }])};
    let style = &shadow.0[0];
    assert_eq!(style.x_offset, Val::Px(100.0));
    assert_eq!(style.y_offset, Val::Percent(50.0));
    assert_eq!(style.spread_radius, Val::Vh(3.1));
    assert_eq!(style.blur_radius, Val::VMax(40.23));
}

#[test]
fn code_multiple_assignments() {
    let c1 = code!{#FF0000};
    let c2 = code!{#00FF00};
    let c3 = code!{#0000FF};
    let s1 = c1.to_srgba();
    let s2 = c2.to_srgba();
    let s3 = c3.to_srgba();
    assert_eq!(s1.red, 1.0);
    assert_eq!(s1.green, 0.0);
    assert_eq!(s1.blue, 0.0);
    assert_eq!(s2.red, 0.0);
    assert_eq!(s2.green, 1.0);
    assert_eq!(s2.blue, 0.0);
    assert_eq!(s3.red, 0.0);
    assert_eq!(s3.green, 0.0);
    assert_eq!(s3.blue, 1.0);
}

// Section 6: `ui!{}` — Tuple Mode: Node Component \\
// ================================================= \\

#[test]
fn ui_tuple_node_size() {
    let node = ui!((
        size: 100px 100px;
    ));
    assert_eq!(node.width, Val::Px(100.0));
    assert_eq!(node.height, Val::Px(100.0));
}

#[test]
fn ui_tuple_node_size_single() {
    let node = ui!((
        width: 50px;
        height: 30px;
    ));
    assert_eq!(node.width, Val::Px(50.0));
    assert_eq!(node.height, Val::Px(30.0));
}

#[test]
fn ui_tuple_node_min_max_size() {
    let node = ui!((
        min_width: 50px;
        min_height: 30px;
        max_width: 200px;
        max_height: 100px;
    ));
    assert_eq!(node.min_width, Val::Px(50.0));
    assert_eq!(node.min_height, Val::Px(30.0));
    assert_eq!(node.max_width, Val::Px(200.0));
    assert_eq!(node.max_height, Val::Px(100.0));
}

#[test]
fn ui_tuple_node_flex_basis() {
    let node = ui!((
        flex_basis: 100px;
    ));
    assert_eq!(node.flex_basis, Val::Px(100.0));
}

#[test]
fn ui_tuple_node_aspect_ratio() {
    let node = ui!((
        aspect_ratio: 1.5;
    ));
    assert_eq!(node.aspect_ratio, Some(1.5));
}

#[test]
fn ui_tuple_node_display_flex() {
    let node = ui!((
        display: flex;
    ));
    assert_eq!(node.display, Display::Flex);
}

#[test]
fn ui_tuple_node_display_grid() {
    let node = ui!((
        display: grid;
    ));
    assert_eq!(node.display, Display::Grid);
}

#[test]
fn ui_tuple_node_display_block() {
    let node = ui!((
        display: block;
    ));
    assert_eq!(node.display, Display::Block);
}

#[test]
fn ui_tuple_node_display_none() {
    let node = ui!((
        display: none;
    ));
    assert_eq!(node.display, Display::None);
}

#[test]
fn ui_tuple_node_position_type_absolute() {
    let node = ui!((
        position_type: absolute;
    ));
    assert_eq!(node.position_type, PositionType::Absolute);
}

#[test]
fn ui_tuple_node_position_type_relative() {
    let node = ui!((
        position_type: relative;
    ));
    assert_eq!(node.position_type, PositionType::Relative);
}

#[test]
fn ui_tuple_node_position_type_absolute_shortcut() {
    let node = ui!((
        absolute;
    ));
    assert_eq!(node.position_type, PositionType::Absolute);
}

#[test]
fn ui_tuple_node_position_type_relative_shortcut() {
    let node = ui!((
        relative;
    ));
    assert_eq!(node.position_type, PositionType::Relative);
}

#[test]
fn ui_tuple_node_flex_direction_row() {
    let node = ui!((
        flex_direction: row;
    ));
    assert_eq!(node.flex_direction, FlexDirection::Row);
}

#[test]
fn ui_tuple_node_flex_direction_column() {
    let node = ui!((
        flex_direction: column;
    ));
    assert_eq!(node.flex_direction, FlexDirection::Column);
}

#[test]
fn ui_tuple_node_flex_direction_row_reverse() {
    let node = ui!((
        flex_direction: row_reverse;
    ));
    assert_eq!(node.flex_direction, FlexDirection::RowReverse);
}

#[test]
fn ui_tuple_node_flex_direction_column_reverse() {
    let node = ui!((
        flex_direction: column_reverse;
    ));
    assert_eq!(node.flex_direction, FlexDirection::ColumnReverse);
}

#[test]
fn ui_tuple_node_flex_grow() {
    let node = ui!((
        flex_grow: 2;
    ));
    assert_eq!(node.flex_grow, 2.0);
}

#[test]
fn ui_tuple_node_flex_shrink() {
    let node = ui!((
        flex_shrink: 0;
    ));
    assert_eq!(node.flex_shrink, 0.0);
}

#[test]
fn ui_tuple_node_flex_wrap_no_wrap() {
    let node = ui!((
        flex_wrap: no_wrap;
    ));
    assert_eq!(node.flex_wrap, FlexWrap::NoWrap);
}

#[test]
fn ui_tuple_node_flex_wrap_wrap() {
    let node = ui!((
        flex_wrap: wrap;
    ));
    assert_eq!(node.flex_wrap, FlexWrap::Wrap);
}

#[test]
fn ui_tuple_node_flex_wrap_wrap_reverse() {
    let node = ui!((
        flex_wrap: wrap_reverse;
    ));
    assert_eq!(node.flex_wrap, FlexWrap::WrapReverse);
}

#[test]
fn ui_tuple_node_justify_content_center() {
    let node = ui!((
        justify_content: center;
    ));
    assert_eq!(node.justify_content, JustifyContent::Center);
}

#[test]
fn ui_tuple_node_justify_content_stretch() {
    let node = ui!((
        justify_content: stretch;
    ));
    assert_eq!(node.justify_content, JustifyContent::Stretch);
}

#[test]
fn ui_tuple_node_justify_items_center() {
    let node = ui!((
        justify_items: center;
    ));
    assert_eq!(node.justify_items, JustifyItems::Center);
}

#[test]
fn ui_tuple_node_justify_self_center() {
    let node = ui!((
        justify_self: center;
    ));
    assert_eq!(node.justify_self, JustifySelf::Center);
}

#[test]
fn ui_tuple_node_align_items_center() {
    let node = ui!((
        align_items: center;
    ));
    assert_eq!(node.align_items, AlignItems::Center);
}

#[test]
fn ui_tuple_node_align_self_center() {
    let node = ui!((
        align_self: center;
    ));
    assert_eq!(node.align_self, AlignSelf::Center);
}

#[test]
fn ui_tuple_node_gap_x_y() {
    let node = ui!((
        gap_x: 10px;
        gap_y: 20px;
    ));
    assert_eq!(node.column_gap, Val::Px(10.0));
    assert_eq!(node.row_gap, Val::Px(20.0));
}

#[test]
fn ui_tuple_node_column_gap_row_gap() {
    let node = ui!((
        column_gap: 5px;
        row_gap: 15px;
    ));
    assert_eq!(node.column_gap, Val::Px(5.0));
    assert_eq!(node.row_gap, Val::Px(15.0));
}

#[test]
fn ui_tuple_node_overflow_visible() {
    let node = ui!((
        overflow: visible;
    ));
    assert_eq!(node.overflow.x, OverflowAxis::Visible);
    assert_eq!(node.overflow.y, OverflowAxis::Visible);
}

#[test]
fn ui_tuple_node_overflow_hidden() {
    let node = ui!((
        overflow: hidden;
    ));
    assert_eq!(node.overflow.x, OverflowAxis::Hidden);
    assert_eq!(node.overflow.y, OverflowAxis::Hidden);
}

#[test]
fn ui_tuple_node_overflow_clip() {
    let node = ui!((
        overflow: clip clip;
    ));
    assert_eq!(node.overflow.x, OverflowAxis::Clip);
    assert_eq!(node.overflow.y, OverflowAxis::Clip);
}

#[test]
fn ui_tuple_node_overflow_clip_margin_content_box() {
    let node = ui!((
        overflow_clip_margin: content_box;
    ));
    assert_eq!(node.overflow_clip_margin.visual_box, OverflowClipBox::ContentBox);
}

#[test]
fn ui_tuple_node_grid_auto_flow_row() {
    let node = ui!((
        grid_auto_flow: row;
    ));
    assert_eq!(node.grid_auto_flow, GridAutoFlow::Row);
}

#[test]
fn ui_tuple_node_grid_auto_flow_column() {
    let node = ui!((
        grid_auto_flow: column;
    ));
    assert_eq!(node.grid_auto_flow, GridAutoFlow::Column);
}

#[test]
fn ui_tuple_node_grid_auto_flow_row_dense() {
    let node = ui!((
        grid_auto_flow: row_dense;
    ));
    assert_eq!(node.grid_auto_flow, GridAutoFlow::RowDense);
}

#[test]
fn ui_tuple_node_grid_auto_flow_column_dense() {
    let node = ui!((
        grid_auto_flow: column_dense;
    ));
    assert_eq!(node.grid_auto_flow, GridAutoFlow::ColumnDense);
}

#[test]
fn ui_tuple_node_grid_row_start() {
    let node = ui!((
        grid_row: start 1;
    ));
    assert_eq!(node.grid_row, GridPlacement::start(1));
}

#[test]
fn ui_tuple_node_grid_row_span() {
    let node = ui!((
        grid_row: span 2;
    ));
    assert_eq!(node.grid_row, GridPlacement::span(2));
}

#[test]
fn ui_tuple_node_grid_row_end() {
    let node = ui!((
        grid_row: end 3;
    ));
    assert_eq!(node.grid_row, GridPlacement::end(3));
}

#[test]
fn ui_tuple_node_grid_row_start_span() {
    let node = ui!((
        grid_row: start_span 2 4;
    ));
    assert_eq!(node.grid_row, GridPlacement::start_span(2, 4));
}

#[test]
fn ui_tuple_node_grid_row_start_end() {
    let node = ui!((
        grid_row: start_end 2 6;
    ));
    assert_eq!(node.grid_row, GridPlacement::start_end(2, 6));
}

#[test]
fn ui_tuple_node_grid_row_end_span() {
    let node = ui!((
        grid_row: end_span 6 4;
    ));
    assert_eq!(node.grid_row, GridPlacement::end_span(6, 4));
}

#[test]
fn ui_tuple_node_grid_column_span() {
    let node = ui!((
        grid_column: span 2;
    ));
    assert_eq!(node.grid_column, GridPlacement::span(2));
}

#[test]
fn ui_tuple_node_grid_auto_rows() {
    let node = ui!((
        grid_auto_rows: 100px 30%;
    ));
    let should: Vec<GridTrack> = vec![
        GridTrack::px(100.0),
        GridTrack::percent(30.0),
    ];
    assert_eq!(node.grid_auto_rows, should);
}

#[test]
fn ui_tuple_node_grid_auto_columns() {
    let node = ui!((
        grid_auto_columns: 100px 30%;
    ));
    let should: Vec<GridTrack> = vec![
        GridTrack::px(100.0),
        GridTrack::percent(30.0),
    ];
    assert_eq!(node.grid_auto_columns, should);
}

#[test]
fn ui_tuple_node_grid_template_rows() {
    let node = ui!((
        grid_template_rows: 2:100px;
    ));
    let should: Vec<RepeatedGridTrack> = vec![
        RepeatedGridTrack::px(2, 100.0),
    ];
    assert_eq!(node.grid_template_rows, should);
}

#[test]
fn ui_tuple_node_grid_template_columns() {
    let node = ui!((
        grid_template_columns: 3:1fr;
    ));
    let should: Vec<RepeatedGridTrack> = vec![
        RepeatedGridTrack::fr(3, 1.0),
    ];
    assert_eq!(node.grid_template_columns, should);
}

// Section 7: `ui!{}` — Tuple Mode: Transform Component \\
// ===================================================== \\

#[test]
fn ui_tuple_transform_scale_xy() {
    let transform = ui!((
        scale: 2.0 1.5;
    ));
    assert_eq!(transform.scale.x, 2.0);
    assert_eq!(transform.scale.y, 1.5);
}

#[test]
fn ui_tuple_transform_scale_single() {
    let transform = ui!((
        scale: 3.0;
    ));
    assert_eq!(transform.scale.x, 3.0);
    assert_eq!(transform.scale.y, 3.0);
}

#[test]
fn ui_tuple_transform_rotation_radians() {
    let transform = ui!((
        rotation: 1.41;
    ));
    let expected = Quat::from_rotation_z(1.41);
    assert!((transform.rotation.x - expected.x).abs() < 0.001);
    assert!((transform.rotation.y - expected.y).abs() < 0.001);
    assert!((transform.rotation.z - expected.z).abs() < 0.001);
}

#[test]
fn ui_tuple_transform_rotation_degrees() {
    let transform = ui!((
        rotation: 45deg;
    ));
    let expected = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);
    assert!((transform.rotation.x - expected.x).abs() < 0.001);
    assert!((transform.rotation.y - expected.y).abs() < 0.001);
    assert!((transform.rotation.z - expected.z).abs() < 0.001);
}

// Section 8: `ui!{}` — Tuple Mode: Visibility Component \\
// ======================================================= \\

#[test]
fn ui_tuple_visibility_hidden() {
    let vis = ui!((
        hidden;
    ));
    assert_eq!(vis, Visibility::Hidden);
}

#[test]
fn ui_tuple_visibility_visible() {
    let vis = ui!((
        visible;
    ));
    assert_eq!(vis, Visibility::Visible);
}

#[test]
fn ui_tuple_visibility_inherit() {
    let vis = ui!((
        inherit;
    ));
    assert_eq!(vis, Visibility::Inherited);
}

// Section 9: `ui!{}` — Tuple Mode: Node Positional Fields \\
// ========================================================= \\

#[test]
fn ui_tuple_node_left() {
    let node = ui!((
        left: 10px;
    ));
    assert_eq!(node.left, Val::Px(10.0));
}

#[test]
fn ui_tuple_node_right() {
    let node = ui!((
        right: 20px;
    ));
    assert_eq!(node.right, Val::Px(20.0));
}

#[test]
fn ui_tuple_node_top() {
    let node = ui!((
        top: 30px;
    ));
    assert_eq!(node.top, Val::Px(30.0));
}

#[test]
fn ui_tuple_node_bottom() {
    let node = ui!((
        bottom: 40px;
    ));
    assert_eq!(node.bottom, Val::Px(40.0));
}

#[test]
fn ui_tuple_node_all_positions() {
    let node = ui!((
        left: 10px;
        right: 20px;
        top: 30px;
        bottom: 40px;
    ));
    assert_eq!(node.left, Val::Px(10.0));
    assert_eq!(node.right, Val::Px(20.0));
    assert_eq!(node.top, Val::Px(30.0));
    assert_eq!(node.bottom, Val::Px(40.0));
}

#[test]
fn ui_tuple_node_x() {
    let node = ui!((
        x: 10px;
    ));
    assert_eq!(node.left, Val::Px(10.0));
    assert_eq!(node.right, Val::Px(10.0));
}

#[test]
fn ui_tuple_node_y() {
    let node = ui!((
        y: 20px;
    ));
    assert_eq!(node.top, Val::Px(20.0));
    assert_eq!(node.bottom, Val::Px(20.0));
}

#[test]
fn ui_tuple_node_xy() {
    let node = ui!((
        xy: 15px;
    ));
    assert_eq!(node.left, Val::Px(15.0));
    assert_eq!(node.right, Val::Px(15.0));
    assert_eq!(node.top, Val::Px(15.0));
    assert_eq!(node.bottom, Val::Px(15.0));
}

#[test]
fn ui_tuple_node_z_index() {
    let zi = ui!((
        z_index: 10;
    ));
    assert_eq!(zi.0, 10);
}

#[test]
fn ui_tuple_node_z_global() {
    let zg = ui!((
        z_global: 20;
    ));
    assert_eq!(zg.0, 20);
}

// Section 10: `ui!{}` — Tuple Mode: Margin \\
// ========================================== \\

#[test]
fn ui_tuple_margin_four_values() {
    let node = ui!((
        margin: 10px 5px 3px 1px;
    ));
    assert_eq!(node.margin.top, Val::Px(10.0));
    assert_eq!(node.margin.right, Val::Px(5.0));
    assert_eq!(node.margin.bottom, Val::Px(3.0));
    assert_eq!(node.margin.left, Val::Px(1.0));
}

#[test]
fn ui_tuple_margin_two_values() {
    let node = ui!((
        margin: 10px 5px;
    ));
    assert_eq!(node.margin.top, Val::Px(10.0));
    assert_eq!(node.margin.bottom, Val::Px(10.0));
    assert_eq!(node.margin.left, Val::Px(5.0));
    assert_eq!(node.margin.right, Val::Px(5.0));
}

#[test]
fn ui_tuple_margin_single() {
    let node = ui!((
        margin: 10px;
    ));
    assert_eq!(node.margin.top, Val::Px(10.0));
    assert_eq!(node.margin.right, Val::Px(10.0));
    assert_eq!(node.margin.bottom, Val::Px(10.0));
    assert_eq!(node.margin.left, Val::Px(10.0));
}

#[test]
fn ui_tuple_margin_x() {
    let node = ui!((
        margin_x: 15px;
    ));
    assert_eq!(node.margin.left, Val::Px(15.0));
    assert_eq!(node.margin.right, Val::Px(15.0));
}

#[test]
fn ui_tuple_margin_y() {
    let node = ui!((
        margin_y: 25px;
    ));
    assert_eq!(node.margin.top, Val::Px(25.0));
    assert_eq!(node.margin.bottom, Val::Px(25.0));
}

// Section 11: `ui!{}` — Tuple Mode: Padding \\
// =========================================== \\

#[test]
fn ui_tuple_padding_four_values() {
    let node = ui!((
        padding: 20px 10px 5px 2px;
    ));
    assert_eq!(node.padding.top, Val::Px(20.0));
    assert_eq!(node.padding.right, Val::Px(10.0));
    assert_eq!(node.padding.bottom, Val::Px(5.0));
    assert_eq!(node.padding.left, Val::Px(2.0));
}

#[test]
fn ui_tuple_padding_two_values() {
    let node = ui!((
        padding: 20px 10px;
    ));
    assert_eq!(node.padding.top, Val::Px(20.0));
    assert_eq!(node.padding.bottom, Val::Px(20.0));
    assert_eq!(node.padding.left, Val::Px(10.0));
    assert_eq!(node.padding.right, Val::Px(10.0));
}

#[test]
fn ui_tuple_padding_single() {
    let node = ui!((
        padding: 15px;
    ));
    assert_eq!(node.padding.top, Val::Px(15.0));
    assert_eq!(node.padding.right, Val::Px(15.0));
    assert_eq!(node.padding.bottom, Val::Px(15.0));
    assert_eq!(node.padding.left, Val::Px(15.0));
}

#[test]
fn ui_tuple_padding_x() {
    let node = ui!((
        padding_x: 12px;
    ));
    assert_eq!(node.padding.left, Val::Px(12.0));
    assert_eq!(node.padding.right, Val::Px(12.0));
}

#[test]
fn ui_tuple_padding_y() {
    let node = ui!((
        padding_y: 18px;
    ));
    assert_eq!(node.padding.top, Val::Px(18.0));
    assert_eq!(node.padding.bottom, Val::Px(18.0));
}

// Section 12: `ui!{}` — Tuple Mode: New bevy 0.18 Node Fields \\
// ============================================================== \\

#[test]
fn ui_tuple_node_box_sizing_border_box() {
    let node = ui!((
        box_sizing: border_box;
    ));
    assert_eq!(node.box_sizing, BoxSizing::BorderBox);
}

#[test]
fn ui_tuple_node_box_sizing_content_box() {
    let node = ui!((
        box_sizing: content_box;
    ));
    assert_eq!(node.box_sizing, BoxSizing::ContentBox);
}

#[test]
fn ui_tuple_node_scrollbar_width() {
    let node = ui!((
        scrollbar_width: 10;
    ));
    assert_eq!(node.scrollbar_width, 10.0);
}

#[test]
fn ui_tuple_node_overflow_scroll() {
    let node = ui!((
        overflow: scroll;
    ));
    assert_eq!(node.overflow.x, OverflowAxis::Scroll);
    assert_eq!(node.overflow.y, OverflowAxis::Scroll);
}

#[test]
fn ui_tuple_node_overflow_scroll_separate() {
    let node = ui!((
        overflow: scroll visible;
    ));
    assert_eq!(node.overflow.x, OverflowAxis::Scroll);
    assert_eq!(node.overflow.y, OverflowAxis::Visible);
}

// Section 13: `ui!{}` — Tuple Mode: Box Styling \\
// ================================================ \\

#[test]
fn ui_tuple_background_hex() {
    let bg = ui!((
        background: #ff0000;
    ));
    let srgba = bg.0.to_srgba();
    assert_eq!(srgba.red, 1.0);
    assert_eq!(srgba.green, 0.0);
    assert_eq!(srgba.blue, 0.0);
}

#[test]
fn ui_tuple_background_css_name() {
    let bg = ui!((
        background: gray;
    ));
    let srgba = bg.0.to_srgba();
    assert!((srgba.red - 128.0/255.0).abs() < 0.01);
    assert!((srgba.green - 128.0/255.0).abs() < 0.01);
    assert!((srgba.blue - 128.0/255.0).abs() < 0.01);
}

#[test]
fn ui_tuple_border_single_value() {
    let node = ui!((
        border: 5px;
    ));
    assert_eq!(node.border.top, Val::Px(5.0));
    assert_eq!(node.border.right, Val::Px(5.0));
    assert_eq!(node.border.bottom, Val::Px(5.0));
    assert_eq!(node.border.left, Val::Px(5.0));
}

#[test]
fn ui_tuple_border_four_values_color() {
    let (node, _border) = ui!((
        border: 5px 2px 4px 1px #ff0000;
    ));
    assert_eq!(node.border.top, Val::Px(5.0));
    assert_eq!(node.border.right, Val::Px(2.0));
    assert_eq!(node.border.bottom, Val::Px(4.0));
    assert_eq!(node.border.left, Val::Px(1.0));
}

#[test]
fn ui_tuple_border_color() {
    let border = ui!((
        border_color: #00ff00;
    ));
    let srgba = border.top.to_srgba();
    assert_eq!(srgba.green, 1.0);
}

#[test]
fn ui_tuple_border_radius_single() {
    let rounded = ui!((
        border_radius: 6px;
    ));
    assert_eq!(rounded.border_radius.top_left, Val::Px(6.0));
    assert_eq!(rounded.border_radius.top_right, Val::Px(6.0));
    assert_eq!(rounded.border_radius.bottom_right, Val::Px(6.0));
    assert_eq!(rounded.border_radius.bottom_left, Val::Px(6.0));
}

#[test]
fn ui_tuple_border_radius_multi() {
    let rounded = ui!((
        border_radius: 5px 0px 3px 1px;
    ));
    assert_eq!(rounded.border_radius.top_left, Val::Px(5.0));
    assert_eq!(rounded.border_radius.top_right, Val::Px(0.0));
    assert_eq!(rounded.border_radius.bottom_right, Val::Px(3.0));
    assert_eq!(rounded.border_radius.bottom_left, Val::Px(1.0));
}

#[test]
fn ui_tuple_border_radius_two_values() {
    let rounded = ui!((
        border_radius: 5px 0px;
    ));
    assert_eq!(rounded.border_radius.top_left, Val::Px(5.0));
    assert_eq!(rounded.border_radius.top_right, Val::Px(5.0));
    assert_eq!(rounded.border_radius.bottom_right, Val::Px(0.0));
    assert_eq!(rounded.border_radius.bottom_left, Val::Px(0.0));
}

#[test]
fn ui_tuple_box_shadow_full() {
    let shadow = ui!((
        box_shadow: 10px 10px 3px 8px #ffaa44;
    ));
    let style = &shadow.0[0];
    assert_eq!(style.x_offset, Val::Px(10.0));
    assert_eq!(style.y_offset, Val::Px(10.0));
    assert_eq!(style.blur_radius, Val::Px(3.0));
    assert_eq!(style.spread_radius, Val::Px(8.0));
    let srgba = style.color.to_srgba();
    assert_eq!(srgba.red, 255.0/255.0);
    assert_eq!(srgba.green, 170.0/255.0);
    assert_eq!(srgba.blue, 68.0/255.0);
}

#[test]
fn ui_tuple_box_shadow_with_percent() {
    let shadow = ui!((
        box_shadow: 10% 10% 3px 8px #ffaa44;
    ));
    let style = &shadow.0[0];
    assert_eq!(style.x_offset, Val::Percent(10.0));
    assert_eq!(style.y_offset, Val::Percent(10.0));
}

#[test]
fn ui_tuple_box_shadow_keep_values() {
    let shadow = ui!((
        box_shadow: _ _ 0px 0px;
    ));
    let style = &shadow.0[0];
    assert_eq!(style.blur_radius, Val::Px(0.0));
    assert_eq!(style.spread_radius, Val::Px(0.0));
}

#[test]
fn ui_tuple_box_shadow_color_only() {
    let shadow = ui!((
        box_shadow: #ff0000;
    ));
    let style = &shadow.0[0];
    let srgba = style.color.to_srgba();
    assert_eq!(srgba.red, 1.0);
    assert_eq!(srgba.green, 0.0);
    assert_eq!(srgba.blue, 0.0);
}

#[test]
fn ui_tuple_outline_full() {
    let outline = ui!((
        outline: 3px 1px #00ff00;
    ));
    assert_eq!(outline.width, Val::Px(3.0));
    assert_eq!(outline.offset, Val::Px(1.0));
    let srgba = outline.color.to_srgba();
    assert_eq!(srgba.green, 1.0);
}

// Section 14: `ui!{}` — Tuple Mode: Text Styling \\
// ================================================= \\

#[test]
fn ui_tuple_text_size() {
    let text_font = ui!((
        text_size: 20;
    ));
    assert_eq!(text_font.font_size, 20.0);
}

#[test]
fn ui_tuple_font_color() {
    let text_color = ui!((
        font_color: #ff0000;
    ));
    let srgba = text_color.0.to_srgba();
    assert_eq!(srgba.red, 1.0);
    assert_eq!(srgba.green, 0.0);
    assert_eq!(srgba.blue, 0.0);
}

#[test]
fn ui_tuple_font_color_css_name() {
    let text_color = ui!((
        font_color: blue;
    ));
    let srgba = text_color.0.to_srgba();
    assert_eq!(srgba.blue, 1.0);
}

#[test]
fn ui_tuple_line_break_word_boundary() {
    let layout = ui!((
        line_break: word_boundary;
    ));
    assert_eq!(layout.linebreak, LineBreak::WordBoundary);
}

#[test]
fn ui_tuple_line_break_any_character() {
    let layout = ui!((
        line_break: any_character;
    ));
    assert_eq!(layout.linebreak, LineBreak::AnyCharacter);
}

#[test]
fn ui_tuple_line_break_word_or_character() {
    let layout = ui!((
        line_break: word_or_character;
    ));
    assert_eq!(layout.linebreak, LineBreak::WordOrCharacter);
}

#[test]
fn ui_tuple_line_break_no_wrap() {
    let layout = ui!((
        line_break: no_wrap;
    ));
    assert_eq!(layout.linebreak, LineBreak::NoWrap);
}

#[test]
fn ui_tuple_line_height() {
    let lh = ui!((
        line_height: 1.2;
    ));
    match lh {
        LineHeight::RelativeToFont(val) => assert_eq!(val, 1.2),
        _ => panic!("expected RelativeToFont, got {:?}", lh),
    }
}

#[test]
fn ui_tuple_text_shadow() {
    let shadow = ui!((
        text_shadow: 2 2 #000;
    ));
    assert_eq!(shadow.offset.x, 2.0);
    assert_eq!(shadow.offset.y, 2.0);
    let srgba = shadow.color.to_srgba();
    assert_eq!(srgba.red, 0.0);
    assert_eq!(srgba.green, 0.0);
    assert_eq!(srgba.blue, 0.0);
}

#[test]
fn ui_tuple_justify_text_center() {
    let layout = ui!((
        justify_text: center;
    ));
    assert_eq!(layout.justify, Justify::Center);
}

// Section 15: `ui!{}` — Tuple Mode: Image Fields \\
// ================================================= \\

#[test]
fn ui_tuple_image_flip_x() {
    let img = ui!((
        image: flip_x;
    ));
    assert_eq!(img.flip_x, true);
}

#[test]
fn ui_tuple_image_flip_y() {
    let img = ui!((
        image: flip_y;
    ));
    assert_eq!(img.flip_y, true);
}

#[test]
fn ui_tuple_image_color() {
    let img = ui!((
        image_color: #ff0000;
    ));
    let srgba = img.color.to_srgba();
    assert_eq!(srgba.red, 1.0);
    assert_eq!(srgba.green, 0.0);
    assert_eq!(srgba.blue, 0.0);
}

#[test]
fn ui_tuple_focus_policy_pass() {
    let policy = ui!((
        focus_policy: pass;
    ));
    assert_eq!(policy, FocusPolicy::Pass);
}

#[test]
fn ui_tuple_focus_policy_block() {
    let policy = ui!((
        focus_policy: block;
    ));
    assert_eq!(policy, FocusPolicy::Block);
}

#[test]
fn ui_tuple_focus_shortcut() {
    let policy = ui!((
        focus: pass;
    ));
    assert_eq!(policy, FocusPolicy::Pass);
}

// Section 16: `ui!{}` — Slim Inline Mode \\
// ======================================== \\

#[test]
fn ui_slim_size() {
    let node = ui!(
        w:100 h:200
    );
    assert_eq!(node.width, Val::Px(100.0));
    assert_eq!(node.height, Val::Px(200.0));
}

#[test]
fn ui_slim_background() {
    let bg = ui!(
        bg:#ffffff
    );
    let srgba = bg.0.to_srgba();
    assert_eq!(srgba.blue, 1.0);
}

#[test]
fn ui_slim_border() {
    let (node, _border) = ui!(
        w:100 h:100 border:5 #f00
    );
    assert_eq!(node.width, Val::Px(100.0));
}

#[test]
fn ui_slim_round() {
    let rounded = ui!(
        round:6
    );
    assert_eq!(rounded.border_radius.top_left, Val::Px(6.0));
    assert_eq!(rounded.border_radius.top_right, Val::Px(6.0));
    assert_eq!(rounded.border_radius.bottom_right, Val::Px(6.0));
    assert_eq!(rounded.border_radius.bottom_left, Val::Px(6.0));
}

#[test]
fn ui_slim_margin() {
    let node = ui!(
        m:10px
    );
    assert_eq!(node.margin.top, Val::Px(10.0));
    assert_eq!(node.margin.right, Val::Px(10.0));
    assert_eq!(node.margin.bottom, Val::Px(10.0));
    assert_eq!(node.margin.left, Val::Px(10.0));
}

#[test]
fn ui_slim_padding() {
    let node = ui!(
        p:15px
    );
    assert_eq!(node.padding.top, Val::Px(15.0));
    assert_eq!(node.padding.right, Val::Px(15.0));
    assert_eq!(node.padding.bottom, Val::Px(15.0));
    assert_eq!(node.padding.left, Val::Px(15.0));
}

#[test]
fn ui_slim_shadow() {
    let shadow = ui!(
        shadow:0+0+3+8#ff0
    );
    let style = &shadow.0[0];
    assert_eq!(style.x_offset, Val::Px(0.0));
    assert_eq!(style.y_offset, Val::Px(0.0));
    assert_eq!(style.blur_radius, Val::Px(3.0));
    assert_eq!(style.spread_radius, Val::Px(8.0));
}

// Section 17: `ui!{}` — Function Mode (returns impl Bundle) \\
// =========================================================== \\

ui! {test_neat_box(
    size: 100px 100px;
    background: #ffffff;
    border: 5px #ff0000;
)}

// Section 18: `ui!{}` — Edit Function Mode \\
// ========================================== \\

ui! {test_edit_red{
    background: #ff0000;
}}

#[test]
fn ui_edit_function_defined() {
    let mut bg = BackgroundColor::default();
    test_edit_red(&mut bg);
    let srgba = bg.0.to_srgba();
    assert_eq!(srgba.green, 0.0);
    assert_eq!(srgba.blue, 0.0);
}

ui! {test_edit_border{
    border: _ #00ff00;
}}

#[test]
fn ui_edit_function_with_keep() {
    let mut border = BorderColor::default();
    test_edit_border(&mut border);
    let srgba = border.top.to_srgba();
    assert_eq!(srgba.green, 1.0);
}

ui! {test_edit_transform{
    scale: 2.0 1.5;
}}

#[test]
fn ui_edit_function_transform() {
    let mut transform = Transform::default();
    test_edit_transform(&mut transform);
    assert_eq!(transform.scale.x, 2.0);
    assert_eq!(transform.scale.y, 1.5);
}

ui! {test_edit_node{
    width: 50px;
    height: 30px;
    margin: [>10px];
}}

#[test]
fn ui_edit_function_node() {
    let mut node = Node::default();
    test_edit_node(&mut node);
    assert_eq!(node.width, Val::Px(50.0));
    assert_eq!(node.height, Val::Px(30.0));
}

// Section 19: Combined Realistic Usage \\
// ===================================== \\

#[test]
fn combined_spawn_ui_node() {
    let mut world = World::new();
    world.spawn(Camera2d::default());
    world.spawn(ui!((
        padding: 24px;
        column_gap: 24px;
        box_shadow: _ _ 0px 0px;
    )));

    let mut query = world.query::<&Node>();
    let nodes: Vec<_> = query.iter(&mut world).collect();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].padding.left, Val::Px(24.0));
    assert_eq!(nodes[0].padding.right, Val::Px(24.0));
}

#[test]
fn combined_spawn_entity_with_ui() {
    let mut world = World::new();
    world.spawn(Camera2d::default());
    let ent = world.spawn(ui!((
        size: 80px 50px;
        background: gray;
        border_radius: 8px;
    ))).id();

    let mut query = world.query::<(&Node, &BackgroundColor)>();
    let entries: Vec<_> = query.iter(&mut world).collect();
    assert_eq!(entries.len(), 1);
    let (node, bg) = entries[0];
    assert_eq!(node.width, Val::Px(80.0));
    assert_eq!(node.height, Val::Px(50.0));
    let srgba = bg.0.to_srgba();
    assert!((srgba.red - 128.0/255.0).abs() < 0.01);

    // Verify entity still exists
    assert_eq!(world.entity(ent).id(), ent);
}

#[test]
fn combined_slim_mode_spawn() {
    let mut world = World::new();
    world.spawn(Camera2d::default());
    world.spawn(ui!(
        w:100 h:100 bg:#fff round:6
    ));
    let mut query = world.query::<(&Node, &BackgroundColor)>();
    let entries: Vec<_> = query.iter(&mut world).collect();
    assert_eq!(entries.len(), 1);
    let (node, bg) = entries[0];
    assert_eq!(node.width, Val::Px(100.0));
    assert_eq!(node.height, Val::Px(100.0));
    let srgba = bg.0.to_srgba();
    assert_eq!(srgba.red, 1.0);
    assert_eq!(srgba.green, 1.0);
    assert_eq!(srgba.blue, 1.0);
}

#[test]
fn combined_function_mode_spawn() {
    let mut world = World::new();
    world.spawn(Camera2d::default());
    world.spawn(test_neat_box());
    let mut query = world.query::<(&Node, &BackgroundColor, &BorderColor)>();
    let entries: Vec<_> = query.iter(&mut world).collect();
    assert_eq!(entries.len(), 1);
    let (node, bg, border) = entries[0];
    assert_eq!(node.width, Val::Px(100.0));
    assert_eq!(node.height, Val::Px(100.0));
    let srgba = bg.0.to_srgba();
    assert_eq!(srgba.red, 1.0);
    let border_srgba = border.top.to_srgba();
    assert_eq!(border_srgba.red, 1.0);
}
