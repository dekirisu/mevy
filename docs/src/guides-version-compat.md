# Bevy Version Compatibility

mevy supports Bevy 0.15 through 0.18 via Cargo features. Some APIs differ between versions.

## Required Feature

You **must** specify the Bevy version feature:

```toml
[dependencies]
mevy = { version = "0.3", features = ["0.18"] }
```

## Version Differences

### `box_shadow`

| Version | Field Path |
|---|---|
| 0.15 | `BoxShadow::blur_radius` (direct field) |
| 0.16+ | `BoxShadow::[0].blur_radius` (array access) |

### `border_radius`

| Version | Field Path |
|---|---|
| 0.15 | `BorderRadius::top_left` (direct field) |
| 0.18+ | `Node::border_radius.top_left` (on Node) |

### `scroll_position`

| Version | Field Path |
|---|---|
| 0.15-0.16 | `ScrollPosition::x_offset`, `y_offset` |
| 0.17+ | `ScrollPosition::x`, `y` |

### `line_height`

| Version | Component |
|---|---|
| 0.15-0.17 | `TextFont::line_height` |
| 0.18+ | Separate `LineHeight` component |

### `border_color`

| Version | Structure |
|---|---|
| 0.15-0.16 | `BorderColor(Srgba)` (single value) |
| 0.17+ | `BorderColor { top, right, bottom, left }` (per-edge) |

### `observer` trigger

| Version | Trigger Type |
|---|---|
| 0.15-0.16 | `Trigger::<Pointer<Click>>` |
| 0.17+ | `On::<Pointer<Click>>` |

### `observer` entity access

| Version | Method |
|---|---|
| 0.15 | `trigger.entity()` |
| 0.16 | `trigger.target()` |
| 0.17+ | `trigger.event_target()` |

### `ChildBuilder` methods

| Version | Method |
|---|---|
| 0.15 | `.parent_entity()`, `.spawn(...)` |
| 0.16+ | `.target_entity()`, `.commands_mut()` |

### `ChildOf` relationship

| Version | Method |
|---|---|
| 0.15 | `.set_parent(parent)` |
| 0.16+ | `.insert(ChildOf(parent))` |

## What Stays the Same

- `code!{}` hex color and Val syntax
- `ui!{}` field names and edge/corner selection
- `entity!{}` selector syntax
- All shorthand aliases (`w`, `h`, `bg`, `px`, etc.)

## Recommendation

Always pin your Bevy version feature:

```toml
# For Bevy 0.18
mevy = { version = "0.3", features = ["0.18"] }

# For Bevy 0.17
mevy = { version = "0.3", features = ["0.17"] }
```

This ensures the correct macro expansions for your Bevy version.
