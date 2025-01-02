<h1 align="center">UI Macros for Bevy</h1>
<p align="center">
    <a href="https://github.com/dekirisu/mevy" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/mevy-ee6677"></a>
    <a href="https://crates.io/crates/mevy_ui" style="position:relative"><img src="https://img.shields.io/crates/v/mevy_ui"></a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative"><img src="https://img.shields.io/discord/515100001903312898"></a>
</p>

> [!NOTE]
> This crate is part of [mevy](https://github.com/dekirisu/mevy) (tl;dr: more neat macros) so take a look! 🦆


## ⭐ The Star of the Crate: CSS-like Notation
Using `ui!((..))` (inner round braces) will return a tuple of **mentioned components** only. See [this example](../../examples/ui_bundle.rs).
```rust
c.spawn(ui!((
  size:          100px 100px;
  border:        5px #ff0000;
  box_shadow:    10% 10% 3px 8px #ffaa44;
  background:    #ffffff;
  border_radius: 6px;
  neat_outline;
)?));
//^ optional ? (or any token): hovering shows the returned tuple (if LSP used)

/// function as custom fields or p refabs
fn neat_outline() -> Outline {ui!((
    outline: 3px 1px #00ff00;
))}
```

### Neat Notation
Inside the macro there you can write those things:
- `0px` = `Val::Px(0.)`
- `0%` = `Val::Percent(0.)`
- `0vh` = `Val::Vh(0.)`
- `0vw` = `Val::Vw(0.)`
- `0vmin` = `Val::VMin(0.)`
- `0vmax` = `Val::VMax(0.)`
- `#ff0000` = `Color::Srgba(Srgba::hex("#ff0000").unwrap())`
- `red`|`RED` = `bevy::color::palettes::css::RED`

### Custom Fields
Currently, There are 2 ways to add those:
- a `fn` that returns something that `impl Bundle`
- the `new` method of a struct that `impl Bundle`
```rust
c.spawn(ui!((
  just_glow: #ff0000;
  // targets Outline::new(..)
  Outline: 5px, 2px, #ff0000;
)));

fn just_glow(color:Color) -> BoxShadow {
  BoxShadow {
    spread_radius: Val::Px(5.0),
    blur_radius: Val::Px(5.0),
    color,
    ..default()
  }
}
```

### Edge Selection (CSS-like)
```rust
border: 5px; // 5px to all edges
border: 5px 2px; // 5px vertical, 2px horizontal
border: 5px 2px 8px; // 5px top, 2px horizontal, 8px bottom
border: 5px 2px 4px 1px // (clockwise) top right bottom left
```

### Corner Selection (CSS-like)
```rust
border-radius: 5px; // 5px to all corners
border-radius: 5px 0px; // 5px top-left/right, 0px bottom-left/right
border-radius: 5px 2px 8px; // 5px top-left, 2px top-right, 8px bottom
border-radius: 5px 2px 4px 1px // (clockwise) top-left top-right bottom-right bottom-left
```

### Limitations
At the moment, you can only use variables in custom fields. It's planned to work for built-in fields soon™.


### Built-In Fields
Here's a list of all available out of the box fields, `Val` order same as in bevy
- numbers can be written as int or float
- enum variants can also be written in snake case
```rust
ui!((
  // Node Fields
  left: 1px;
  right: 1px;
  top: 1px;
  bottom: 1px;
  width: 1px;
  height: 1px;
  min_width: 1px;
  max_width: 1px;
  min_height: 1px;
  max_height: 1px;
  flex_basis: 1px;
  row_gap: 1px;
  column_gap: 1px;
  margin: 1px 1px 1px 1px; // see 'Edge Selection'
  padding: 1px 1px 1px 1px; // see 'Edge Selection'
  border: 1px 1px 1px 1px #ff0000; // see 'Edge Selection'
  flex_grow: 1;
  flex_shrink: 1;
  aspect_ratio: 1;
  display: flex|grid|block|none;
  position_type: absolute|relative;
  justify_items: default|start|end|center|baseline|stretch;
  align_items: ^|flex_start|flex_end;
  justify_self: auto|start|end|center|baseline|stretch;
  align_self: ^|flex_start|flex_end;
  justify_content: default|start|end|flex_start|flex_end|center|stretch|space_between|space_evenly|space_around;
  align_content: ^;
  flex_direction: row|column|row_reverse|column_reverse;
  flex_wrap: no_wrap|wrap|wrap_reverse;
  grid_auto_flow: row|column|row_dense|column_dense;
  overflow: visible|clip|hidden|scroll; // set x and y
  overflow: clip clip; // or separately
  overflow_clip_margin: content_box|padding_box|border_box 5; // number optional
  grid_row: span|start|end 10;
  grid_row: start_span|start_end|end_span 8 10;
  grid_column: span|start|end 10;
  grid_column: start_span|start_end|end_span 8 10;
  grid_auto_rows: 1px 3% 10fr min_content; // any number of `GridTrack`s
  grid_auto_columns: 1px 3% 10fr; // ^
  grid_template_rows: 10:1px 10:3%; // ^, but {repetition}: before each Track
  grid_template_columns: 10:1px 10:3%; // ^
  // Separate Components
  background: red; // alias: background_color
  border_color: #ff0000;
  border_radius: 1px 1%;
  outline: 1px 1px #ff0000;
  box_shadow: 1px 1px 1px 1px #ff0000;
  z_index: 10 global; // not writing 'g'|'global' = local
  interaction; // adds `Interaction` component
  cursor_position; // adds `RelativeCursorPosition` component
  focus_policy: pass // alias: focus
  scroll_position: 1px 1px; // alias: scroll
  // Custom Groups
  size: 1px 1px; // width height, of Node
))
```

