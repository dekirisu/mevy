# API Reference

Complete reference documentation for mevy's macro features. This page organizes the detailed references by topic.

## Overview

| Reference | Description |
|---|---|
| [CSS-like Fields & Slim Aliases](api-ui-fields.md) | All `ui!{}` fields with their slim shortcuts — the complete field reference |
| [Entity Selectors](api-entity-selectors.md) | `entity!{}` selector syntax — how to target entities and resources |
| [Grid Track Syntax](api-grid-tracks.md) | Grid track sizing functions — CSS Grid syntax for Bevy |

## Quick Navigation

### `ui!{}` Fields

The `ui!{}` macro supports 80+ fields organized into categories:

- **Position**: `position_type`, `position`, `absolute`, `relative`
- **Visibility**: `hidden`, `visible`, `inherit`
- **Transform**: `scale`, `rotation`
- **Size**: `width`, `height`, `size`, `min_width`, `min_height`, `max_width`, `max_height`, `aspect_ratio`, `flex_basis`
- **Margin & Padding**: `margin`, `margin_x`, `margin_y`, `margin_left`, `margin_right`, `margin_top`, `margin_bottom`, `padding`, `padding_x`, `padding_y`, `padding_left`, `padding_right`, `padding_top`, `padding_bottom`
- **Box Styling**: `background_color`, `border`, `border_radius`, `box_shadow`, `outline`
- **Text**: `font_color`, `font_size`, `text`, `justify_text`, `line_break`, `line_height`, `text_shadow`
- **Images**: `image`, `image_color`
- **Flex**: `flex_direction`, `flex_grow`, `flex_shrink`, `flex_wrap`, `flex_basis`
- **Grid**: `grid_auto_rows`, `grid_auto_columns`, `grid_template_rows`, `grid_template_columns`, `grid_auto_flow`, `grid_row`, `grid_column`
- **Other**: `display`, `justify_items`, `align_items`, `justify_content`, `align_content`, `justify_self`, `align_self`, `overflow`, `overflow_clip_margin`, `z_index`, `z_global`, `focus_policy`, `scroll_position`, `relative_cursor_position`, `interaction`

See [CSS-like Fields](api-ui-fields.md) for the complete reference with slim aliases.

### `entity!{}` Selectors

The selector syntax determines where to operate and what to target:

- **World Types**: `Commands`, `World`, `DeferredWorld`, `EntityCommands`, `ChildBuilder`, `EntityWorldMut`
- **Component Queries**: `<|#*Comp.all()>`, `<|!#Marker>`, `<|!#Comp.0>`
- **Resource Queries**: `<|@Resource.get()>`, `<|!@Resource.0>`, `<|@*Resource.all()>`
- **Redirection**: Chain `<Children.iter()>` to drill down through entities
- **Leaking**: `>` (leak entities), `<` (return root), `@` (capture as closure)

See [Entity Selectors](api-entity-selectors.md) for the complete selector reference.

### Grid Tracks

Grid tracks define the columns and rows of a grid layout:

- **Basic**: `1px`, `3%`, `10fr`, `10!`
- **Named**: `auto`, `min_content`, `max_content`, `min: 100px`, `max: 200px`
- **Minmax**: `minmax(100px 200px)`
- **Repeated**: `10: 1px`, `auto_fill: 1px`, `auto_fit: 200px`
- **Fit Content**: `10fit_px`, `10fit%`

See [Grid Track Syntax](api-grid-tracks.md) for the complete reference.
