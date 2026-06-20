# Migration Guide

This guide helps you migrate between Bevy versions when using mevy. The key principle: **mevy handles API differences automatically**. You only need to change the version feature in `Cargo.toml`.

## Bevy 0.18

Update your `Cargo.toml`:
```toml
mevy = { version = "0.3", features = ["0.18"] }
```

All `entity!{}` features work with Bevy 0.18.

API differences mevy handles automatically (for `mevy_ui` and `mevy_core`):
- `border_radius` now writes to `Node::border_radius` instead of `BorderRadius`
- `line_height` now uses a separate `LineHeight` component instead of `TextFont::line_height`
- Observer entity access uses `trigger.event_target()` (not `trigger.target()` or `trigger.entity()`)

## Bevy 0.17

Update your `Cargo.toml`:
```toml
mevy = { version = "0.3", features = ["0.17"] }
```

No code changes needed. Key changes mevy handles automatically:
- Observer triggers changed from `Trigger<T>` to `On<T>`
- Entity access changed from `trigger.target()` (0.16) to `trigger.event_target()` (0.17+)

## Bevy 0.16

Update your `Cargo.toml`:
```toml
mevy = { version = "0.3", features = ["0.16"] }
```

No code changes needed. Key changes mevy handles automatically:
- `BoxShadow` changed from a direct struct to `Vec<ShadowStyle>`
- Child relationships use `ChildOf` instead of `set_parent()`
- Observer entity access uses `trigger.target()` (not `trigger.entity()` or `trigger.event_target()`)

## Bevy 0.15

Update your `Cargo.toml`:
```toml
mevy = { version = "0.3", features = ["0.15"] }
```

No code changes needed. Key differences in Bevy 0.15:
- `ChildBuilder` uses `.parent_entity()` and `.spawn()` instead of `.target_entity()` and `.commands_mut()`
- `BorderColor` is a single `Srgba` value, not per-edge
- Observer entity access uses `trigger.entity()` (not `trigger.target()` or `trigger.event_target()`)

## Version Support Matrix

| Feature | mevy_ui | mevy_core | mevy_ecs |
|---|---|---|---|
| `0.15` | ✅ | ✅ | ✅ |
| `0.16` | ✅ | ✅ | ✅ |
| `0.16-rc` | ✅ | ✅ | ✅ |
| `0.17` | ✅ | ✅ | ✅ |
| `0.18` | ✅ | ✅ | ✅ |

## Tips

1. **Always pin your Bevy version** — never omit the version feature.
2. **Test with your target Bevy version** — macro expansions differ between versions.
3. **Check the changelog** for breaking changes between mevy versions.
