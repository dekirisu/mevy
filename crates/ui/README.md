<p align="center">
    <img src="https://github.com/user-attachments/assets/829a86b8-8dc4-4403-9da4-536daaefbd11">
</p>
<p align="center">
    <a href="https://github.com/dekirisu/mevy" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/mevy-ee6677"></a>
    <a href="https://crates.io/crates/mevy_ui" style="position:relative"><img src="https://img.shields.io/crates/v/mevy_ui"></a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative"><img src="https://img.shields.io/discord/515100001903312898"></a>
</p>

> [!NOTE]
> This crate is part of [mevy](https://github.com/dekirisu/mevy) (tl;dr: more neat macros) so take a look! 🦆

## Setup
Multiple bevy versions are supported and managed by features:
```toml
# bevy 0.16
mevy_ui = {version="0.2",features=["0.16"]}

# bevy 0.15
mevy_ui = {version="0.2",features=["0.15"]}
```

## ⭐ The Star of the Crate: CSS-like Notation
The macro `ui!()` has multiple modes, that are invoked by (1.) the type of delimiters and (2.) if a name is provided:

### 'Slim' Tuple Inline Mode
Short syntax, inspired by TailwindCSS!
```rust
c.spawn(ui!(
    w:100 h:100 radius:6 bg:#fff border:5#f00 
    shadow:10%+10%+3px+8px#fa4
)?));
```

### Tuple Inline Mode
`ui!((..))` (inner round braces) will return a tuple of **mentioned components** only. See [this example](../../examples/ui_bundle.rs).
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
The order of the components are based on 'first mention' and consistent:
```rust
let (shadow,node) = ui!((
    box_shadow: 10% 10% 3px 8px #ffaa44;
    size: 100px 100px;
));
```
### Tuple Function Mode
If a name is defined before the inner braces, a function will be defined returning an `impl Bundle` of its content:
```rust
ui!{neat_box(
    size:       100px 100px;
    background: #ffffff;
)}

// is the same as:
pub fn neat_box() -> impl Bundle {ui!((
    size:       100px 100px;
    background: #ffffff;
))}
```

### Edit Function  Mode
Defining a name and then using curly brackets `{}` instead, will define a function that mutates existing components:
- The parameters of this function are `&mut ..` of the components to be mutated
- Custom fields will be `insert`ed, an `EntityCommands` will be then added to the parameters
- Using `_` instead of a calue will keep ste already existing one:
    - e.g. `box_shadow: _ _ 10px` will only affect the blur radius, since x and y are 'kept' and spread & color not mentioned
```rust
ui!{into_red_glow{
  border: _ #ffffff;
  background: #ff0000;
  // no value mentined, so only color affected:
  box_shadow: #ff0000 
}}

// does the same as:
pub fn into_red_glow(
    border_color: &mut BorderColor,
    background_color: &mut BackgroundColor,
    box_shadow: &mut BoxShadow
){
    border_color.0 = Srgba::hex("#ffffff").unwrap().into();
    background_color.0 = Srgba::hex("#ff0000").unwrap().into();
    box_shadow.color = Srgba::hex("#ff0000").unwrap().into();
}
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
    - checked by the compiler (and LSP), throwing a custom error if not valid!
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

  // Position Type
  position_type: absolute|relative;
  absolute; // shortcut
  relative; // ^

  // Visibility
  hidden; 
  visible;
  inherit;

  // Transform - be aware how it affects UI!
  scale: 1.0 1.2; // x y
  scale: 1.0; // x & y
  rotation: 1.41; // radian
  rotation: 45deg; // degree

  // Positions
  l|left: 1px;
  r|right: 1px;
  t|top: 1px;
  b|bottom: 1px;
  x: 1px; // left & right
  y: 1px; // top & bottom  
  xy: 1px; // x & y
  z|z_index: 10;
  zg|z_global: 10;

  // Size
  w|width: 1px;
  h|height: 1px;
  size: 1px 1px; // width height
  min_w|min_width: 1px;
  min_h|min_height: 1px;
  max_w|max_width: 1px;
  max_h|max_height: 1px;
  aspect_ratio: 1;
 
  // Margin
  m|margin: 1px 1px 1px 1px; // see 'Edge Selection'
  mt|margin_top: 1px;
  mb|margin_bottom: 1px;
  ml|margin_left: 1px;
  mr|margin_right: 1px;
  mx|margin_x: 1px; // ml & mr
  my|margin_y: 1px; // mt & mb
 
  // Padding
  p|padding: 1px 1px 1px 1px; // see 'Edge Selection'
  pt|padding_top: 1px;
  pb|padding_bottom: 1px;
  pl|padding_left: 1px;
  pr|padding_right: 1px;
  px|padding_x: 1px; // pl & pr
  py|padding_y: 1px; // pt & pb

  // Box Styling
  bg|background: red;
  shadow|box_shadow: 1px 1px 1px 1px #ff0000;
  border: 1px 1px 1px 1px #ff0000; // see 'Edge Selection'
  border_color: #ff0000;
  outline: 1px 1px #ff0000;
  radius|border_radius: 1px 1%;

  // Text Styling
  text|text_size|font_size: 20;
  text|justify_text: left|center|right|justified;
  text|line_break: word_boundary|any_character|word_or_character|no_wrap;
  leading|line_height: 1.2;
  color|font_color: #ff0000;
  text_shadow: 2 2 #000; // x y color

  // Align Self
  justify_self: auto|start|end|center|baseline|stretch;
  align_self: ^|flex_start|flex_end;

  // Handle Children
  display: flex|grid|block|none;
  justify_items: default|start|end|center|baseline|stretch;
  align_items: ^|flex_start|flex_end;
  justify_content: default|start|end|flex_start|flex_end|center|stretch|space_between|space_evenly|space_around;
  align_content: ^;
  gap_y|row_gap: 1px;
  gap_x|column_gap: 1px;
  gap: 1px; // gap_x & gap_y
  overflow: visible|clip|hidden|scroll; // set x and y
  overflow: clip clip; // or separately
  overflow_clip_margin: content_box|padding_box|border_box 5; // number optional
 
  // Flex
  flex|flex_direction: row|column|row_reverse|column_reverse;
  flex_basis: 1px;
  flex_grow: 1;
  flex_shrink: 1;
  flex_wrap: no_wrap|wrap|wrap_reverse;

  // Grid
  grid_auto_flow: row|column|row_dense|column_dense;
  grid_row: span|start|end 10;
  grid_row: start_span|start_end|end_span 8 10;
  grid_column: span|start|end 10;
  grid_column: start_span|start_end|end_span 8 10;
  grid_auto_rows: 1px 3% 10fr min_content; // any number of `GridTrack`s
  grid_auto_columns: 1px 3% 10fr; // ^
  grid_template_rows: 10:1px 10:3%; // ^, but {repetition}: before each Track
  grid_template_columns: 10:1px 10:3%; // ^

  // Separate Components
  interaction; // adds `Interaction` component
  cursor_position; // adds `RelativeCursorPosition` component
  focus|focus_policy: pass 
  scroll|scroll_position: 1px 1px;

))
```

