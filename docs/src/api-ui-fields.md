# CSS-like Fields & Slim Aliases

Complete list of all fields supported by `ui!{}`, with their slim aliases and value patterns.

## How to Use This Reference

This page documents every field you can use inside `ui!{}`. Each field maps to one or more Bevy components. Use **full field names** in CSS-like mode (`ui!((...))`) and **slim aliases** in slim mode (`ui!(...)`).

The `Values` column describes what you can write after the field name:
- `Val` — any numeric value with units (px, %, vh, vw, vmin, vmax)
- `#hex` — any hex color (`#f00`, `#ff0000`, `#ff000080`)
- `edge selection` — CSS-like multi-value notation (see [Edge Selection](macros-ui.md#edge-selection-css-like))
- `corner selection` — CSS-like corner notation (see [Corner Selection](macros-ui.md#corner-selection-css-like))
- `track list` — CSS Grid track syntax (see [Grid Track Syntax](api-grid-tracks.md))

## Position

| Field | Slim Alias | Values |
|---|---|---|
| `position_type` | *(none)* | `absolute`, `relative` |
| `position` | *(none)* | `absolute`, `relative` |
| `absolute` | `absolute` | *(shortcut — no value needed)* |
| `relative` | `relative` | *(shortcut — no value needed)* |

## Visibility

| Field | Slim Alias | Values |
|---|---|---|
| `hidden` | *(none)* | *(shortcut — no value needed)* |
| `visible` | *(none)* | *(shortcut — no value needed)* |
| `inherit` | *(none)* | *(shortcut — no value needed)* |

## Transform

> [!WARNING]
> Transform affects UI layout differently than you might expect. Scale and rotation are applied **after** layout, not before. This means a rotated element won't affect its siblings' layout.

| Field | Slim Alias | Values |
|---|---|---|
| `scale` | *(none)* | `x` or `x y` |
| `rotation` | *(none)* | number (radians) or `numberdeg` |

## Positions

| Field | Slim Alias | Values |
|---|---|---|
| `left` | `l` | `Val` (px, %, vh, vw, vmin, vmax) |
| `right` | `r` | `Val` |
| `top` | `t` | `Val` |
| `bottom` | `b` | `Val` |
| `x` | *(none)* | `Val` (sets left + right) |
| `y` | *(none)* | `Val` (sets top + bottom) |
| `xy` | *(none)* | `Val` (sets left, right, top, bottom) |
| `z_index` | `z`, `zindex` | `number` |
| `z_global` | `zg` | `number` |

## Size

| Field | Slim Alias | Values |
|---|---|---|
| `width` | `w` | `Val` |
| `height` | `h` | `Val` |
| `size` | *(none)* | `width height` |
| `min_width` | `min_w` | `Val` |
| `min_height` | `min_h` | `Val` |
| `max_width` | `max_w` | `Val` |
| `max_height` | `max_h` | `Val` |
| `aspect_ratio` | *(none)* | `f32` |
| `flex_basis` | *(none)* | `Val` |

## Margin

| Field | Slim Alias | Values |
|---|---|---|
| `margin` | `m` | `1` value (all edges) or `edge selection` |
| `margin_top` | `mt` | `Val` |
| `margin_bottom` | `mb` | `Val` |
| `margin_left` | `ml` | `Val` |
| `margin_right` | `mr` | `Val` |
| `margin_x` | `mx` | `Val` (sets left + right) |
| `margin_y` | `my` | `Val` (sets top + bottom) |

## Padding

| Field | Slim Alias | Values |
|---|---|---|
| `padding` | `p` | `1` value (all edges) or `edge selection` |
| `padding_top` | `pt` | `Val` |
| `padding_bottom` | `pb` | `Val` |
| `padding_left` | `pl` | `Val` |
| `padding_right` | `pr` | `Val` |
| `padding_x` | `px` | `Val` (sets left + right) |
| `padding_y` | `py` | `Val` (sets top + bottom) |

## Box Styling

| Field | Slim Alias | Values |
|---|---|---|
| `background_color` | `background`, `bg` | `#hex`, `#rgba`, or CSS color name |
| `box_shadow` | `shadow` | `x y blur spread color` or `1-4 vals + color` |
| `border` | `border` | `edge selection` (see below) |
| `border_color` | *(none)* | `#hex` or CSS color name |
| `outline` | *(none)* | `width offset color` |
| `border_radius` | `round`, `rounded` | `corner selection` (see below) |

### Border Edge Selection

```rust
border: 5px;           // all 4 edges
border: 5px 2px;       // vertical, horizontal
border: 5px 2px 8px;   // top, horizontal, bottom
border: 5px 2px 4px 1px; // top, right, bottom, left
```

### Border Radius Corner Selection

```rust
border_radius: 5px;           // all 4 corners
border_radius: 5px 0px;       // top-left/right, bottom-left/right
border_radius: 5px 2px 8px;   // top-left, top-right, bottom
border_radius: 5px 2px 4px 1px; // clockwise: TL, TR, BR, BL
```

## Text Styling

| Field | Slim Alias | Values |
|---|---|---|
| `font_color` | `color` | `#hex`, `#rgba`, or CSS color name |
| `font_size` | `text_size` | `f32` |
| `text` | *(none)* | `f32` (font size), or enum variants: `Left`, `Center`, `Right`, `Justified` (justify), or `WordBoundary`, `AnyCharacter`, `WordOrCharacter`, `NoWrap` (line break; defaults to `WordBoundary`) |
| `justify_text` | *(none)* | `left`, `center`, `right`, `justified` (converted to PascalCase: `JustifyText::*` or `Justify::*` depending on version; defaults to `left`) |
| `line_break` | *(none)* | `word_boundary`, `any_character`, `word_or_character`, `no_wrap` (converted to `LineBreak::*`; defaults to `word_boundary`) |
| `line_height` | `leading` | `f32` (relative to font) |
| `text_shadow` | *(none)* | `x y color` |

## Images

| Field | Slim Alias | Values |
|---|---|---|
| `image` | `img` | `$var` (variable), `#hex` (color), or `flip_x`/`flip_y` |
| `image_color` | `img_color` | `#hex` or CSS color name |

## Flex

| Field | Slim Alias | Values |
|---|---|---|
| `flex_direction` | `flex` | `row`, `column`, `row_reverse`, `column_reverse` (converted to `FlexDirection::*`) |
| `flex_grow` | *(none)* | `f32` |
| `flex_shrink` | *(none)* | `f32` |
| `flex_wrap` | *(none)* | `no_wrap`, `wrap`, `wrap_reverse` (converted to `FlexWrap::*`) |

## Grid

| Field | Slim Alias | Values |
|---|---|---|
| `grid_auto_flow` | *(none)* | `row`, `column`, `row_dense`, `column_dense` (converted to `GridAutoFlow::*`) |
| `grid_row` | *(none)* | `span n`, `start n`, `end n`, or `start n end n` (note: 3+ value syntax has a known bug in the code) |
| `grid_column` | *(none)* | `span n`, `start n`, `end n`, or `start n end n` (note: 3+ value syntax has a known bug in the code) |
| `grid_auto_rows` | *(none)* | track list (e.g. `1px 3% 10fr min_content`) |
| `grid_auto_columns` | *(none)* | track list |
| `grid_template_rows` | *(none)* | `repetition:track` list |
| `grid_template_columns` | *(none)* | `repetition:track` list |

## Other

| Field | Slim Alias | Values |
|---|---|---|
| `display` | *(none)* | `flex`, `grid`, `block`, `none` (converted to `Display::*`) |
| `justify_items` | *(none)* | `default`, `start`, `end`, `center`, `baseline`, `stretch` |
| `align_items` | *(none)* | `default`, `start`, `end`, `center`, `baseline`, `stretch` |
| `justify_content` | *(none)* | `default`, `start`, `end`, `flex_start`, `flex_end`, `center`, `stretch`, `space_between`, `space_evenly`, `space_around` |
| `align_content` | *(none)* | `default`, `start`, `end`, `flex_start`, `flex_end`, `center`, `stretch`, `space_between`, `space_evenly`, `space_around` |
| `justify_self` | *(none)* | `auto`, `start`, `end`, `center`, `baseline`, `stretch` |
| `align_self` | *(none)* | `auto`, `start`, `end`, `center`, `baseline`, `stretch` |
| `row_gap` | `gap_y` | `Val` |
| `column_gap` | `gap_x` | `Val` |
| `gap` | *(none)* | `gap_x gap_y` |
| `overflow` | *(none)* | `visible`, `clip`, `hidden`, `scroll` (converted to PascalCase: `OverflowAxis::*`; no value = `Overflow::DEFAULT`; or `x y` separately) |
| `overflow_clip_margin` | *(none)* | *(no value = `DEFAULT`)*, `border_box`, `padding_box`, `content_box`, or `border_box 5` (visual_box + optional margin) |
| `relative_cursor_position` | `cursor_pos`, `cursor_position` | *(component only — no value)* |
| `focus_policy` | `focus` | `pass`, `ignore`, `consume` (default: `pass`) |
| `scroll_position` | `scroll` | *(no value = adds component)*, `x y` (optional) |
| `interaction` | *(none)* | *(component only — no value)* |
