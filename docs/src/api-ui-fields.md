# CSS-like Fields & Slim Aliases

Complete list of all fields supported by `ui!{}`, with their slim aliases and value patterns.

In **CSS-like mode** (`ui!((...))`) use the full field names. In **slim mode** (`ui!(...)`) you can use the shorter aliases.

## Position

| Field | Slim Alias | Values |
|---|---|---|
| `position_type` | `position_type` | `absolute`, `relative` |
| `absolute` | `absolute` | *(shortcut — no value needed)* |
| `relative` | `relative` | *(shortcut — no value needed)* |

## Visibility

| Field | Slim Alias | Values |
|---|---|---|
| `hidden` | `hidden` | *(shortcut)* |
| `visible` | `visible` | *(shortcut)* |
| `inherit` | `inherit` | *(shortcut)* |

## Transform

::: warning
Be aware how transform affects UI layout!
:::

| Field | Slim Alias | Values |
|---|---|---|
| `scale` | `scale` | `x` or `x y` |
| `rotation` | `rotation` | number (radians) or `numberdeg` |

## Positions

| Field | Slim Alias | Values |
|---|---|---|
| `left` | `l` | `Val` (px, %, vh, vw, vmin, vmax) |
| `right` | `r` | `Val` |
| `top` | `t` | `Val` |
| `bottom` | `b` | `Val` |
| `x` | `x` | `Val` (sets left + right) |
| `y` | `y` | `Val` (sets top + bottom) |
| `xy` | `xy` | `Val` (sets left, right, top, bottom) |
| `z_index` | `z`, `zindex` | `number` |
| `z_global` | `zg` | `number` |

## Size

| Field | Slim Alias | Values |
|---|---|---|
| `width` | `w` | `Val` |
| `height` | `h` | `Val` |
| `size` | `size` | `width height` |
| `min_width` | `min_w` | `Val` |
| `min_height` | `min_h` | `Val` |
| `max_width` | `max_w` | `Val` |
| `max_height` | `max_h` | `Val` |
| `aspect_ratio` | `aspect_ratio` | `f32` |
| `flex_basis` | `flex_basis` | `Val` |

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
| `background` | `bg`, `background` | `#hex`, `#rgba`, or CSS color name |
| `box_shadow` | `shadow` | `x y blur spread color` or `1-4 vals + color` |
| `border` | `border` | `edge selection` (see below) |
| `border_color` | `border_color` | `#hex` or CSS color name |
| `outline` | `outline` | `width offset color` |
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
| `font_size` | `text_size`, `font_size` | `f32` |
| `text` | `text` | `f32` (font size) or `justify_text` / `line_break` enum |
| `justify_text` | `justify_text` | `left`, `center`, `right`, `justified` |
| `line_break` | `line_break` | `word_boundary`, `any_character`, `word_or_character`, `no_wrap` |
| `line_height` | `leading` | `f32` (relative to font) |
| `text_shadow` | `text_shadow` | `x y color` |

## Images

| Field | Slim Alias | Values |
|---|---|---|
| `image` | `img`, `image` | `$var` (`Handle<Image>`), `$(path)`, `#hex`, or `flip_x`/`flip_y` |
| `image_color` | `img_color`, `image_color` | `#hex` or CSS color name |

## Flex

| Field | Slim Alias | Values |
|---|---|---|
| `flex_direction` | `flex` | `row`, `column`, `row_reverse`, `column_reverse` |
| `flex_grow` | `flex_grow` | `f32` |
| `flex_shrink` | `flex_shrink` | `f32` |
| `flex_wrap` | `flex_wrap` | `no_wrap`, `wrap`, `wrap_reverse` |

## Grid

| Field | Slim Alias | Values |
|---|---|---|
| `grid_auto_flow` | `grid_auto_flow` | `row`, `column`, `row_dense`, `column_dense` |
| `grid_row` | `grid_row` | `span n`, `start n`, `end n`, or `start end` |
| `grid_column` | `grid_column` | `span n`, `start n`, `end n`, or `start end` |
| `grid_auto_rows` | `grid_auto_rows` | track list (e.g. `1px 3% 10fr min_content`) |
| `grid_auto_columns` | `grid_auto_columns` | track list |
| `grid_template_rows` | `grid_template_rows` | `repetition:track` list |
| `grid_template_columns` | `grid_template_columns` | `repetition:track` list |

## Other

| Field | Slim Alias | Values |
|---|---|---|
| `display` | `display` | `flex`, `grid`, `block`, `none` |
| `justify_items` | `justify_items` | `default`, `start`, `end`, `center`, `baseline`, `stretch` |
| `align_items` | `align_items` | `default`, `flex_start`, `flex_end` |
| `justify_content` | `justify_content` | `default`, `start`, `end`, `flex_start`, `flex_end`, `center`, `stretch`, `space_between`, `space_evenly`, `space_around` |
| `align_content` | `align_content` | `default`, `start`, `end`, `flex_start`, `flex_end`, `center`, `stretch`, `space_between`, `space_evenly`, `space_around` |
| `justify_self` | `justify_self` | `auto`, `start`, `end`, `center`, `baseline`, `stretch` |
| `align_self` | `align_self` | `auto`, `start`, `end`, `center`, `baseline`, `stretch` |
| `row_gap` | `gap_y` | `Val` |
| `column_gap` | `gap_x` | `Val` |
| `gap` | `gap` | `gap_x gap_y` |
| `overflow` | `overflow` | `visible`, `clip`, `hidden`, `scroll` (or `x y` separately) |
| `overflow_clip_margin` | `overflow_clip_margin` | `visual_box margin` (optional) |
| `relative_cursor_position` | `cursor_pos`, `cursor_position` | *(component only — no value)* |
| `focus_policy` | `focus` | `pass`, `ignore`, `consume` (default: `pass`) |
| `scroll_position` | `scroll` | `x y` (optional) |
| `interaction` | `interaction` | *(component only — no value)* |
