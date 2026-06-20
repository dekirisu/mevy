# Bevy Version Compatibility

mevy supports Bevy 0.15 through 0.18 via Cargo features. All API differences are handled automatically — you **don't need to change your code** when switching versions.

## Why Version Features?

Bevy changes its APIs between minor versions. For example, `BoxShadow` changed from a direct struct to `Vec<ShadowStyle>` in 0.16, and `BorderColor` changed from a single value to per-edge in 0.17. mevy handles these differences for you, but it needs to know which version you're targeting to generate the correct code.

## Required Feature

You **must** specify the Bevy version feature:

```toml
[dependencies]
mevy = { version = "0.3", features = ["0.18"] }
```

## What Stays the Same

The following patterns are stable across all supported versions:

- `code!{}` hex color and Val syntax
- `ui!{}` field names and edge/corner selection
- `entity!{}` selector syntax
- All shorthand aliases (`w`, `h`, `bg`, `px`, etc.)

You can write the same mevy code for Bevy 0.15 and 0.18 — just change the version feature.

## Version Compatibility Matrix

| Feature | mevy_ui | mevy_core | mevy_ecs |
|---|---|---|---|
| `0.15` | ✅ | ✅ | ✅ |
| `0.16` | ✅ | ✅ | ✅ |
| `0.16-rc` | ✅ | ✅ | ✅ |
| `0.17` | ✅ | ✅ | ✅ |
| `0.18` | ✅ | ✅ | ✅ |

## Version Differences

These are the API changes mevy handles automatically. You don't need to do anything — just set the correct version feature.

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

### Observer Changes

| Version | Trigger Type | Entity Access |
|---|---|---|
| 0.15 | `Trigger::<T>` | `trigger.entity()` |
| 0.16 | `Trigger::<T>` | `trigger.target()` |
| 0.17+ | `On::<T>` | `trigger.event_target()` |

### ChildBuilder Changes

| Version | Methods |
|---|---|
| 0.15 | `.parent_entity()`, `.spawn(...)` |
| 0.16+ | `.target_entity()`, `.commands_mut()` |

### ChildOf Relationship

| Version | Method |
|---|---|
| 0.15 | `.set_parent(parent)` |
| 0.16+ | `.insert(ChildOf(parent))` |

## See Also

- [Installation](installation.md) — Setup and version selection
- [Migration Guide](migration.md) — Step-by-step migration between versions
- [FAQ](faq.md) — Common questions about version compatibility
