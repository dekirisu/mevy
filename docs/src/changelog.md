# Changelog

All notable changes to mevy are documented here.

## [0.3.2] — 2026-06-20

### Added
- Comprehensive documentation site with mdBook
- Grid track syntax support in `ui!{}` (`auto_fill`, `auto_fit`, `fit_content`, `minmax`)
- `0.18` feature for `mevy_ui` and `mevy_core`

### Fixed
- Hex color validation for 5-digit hex codes (e.g., `#f0a` → `#ff00aa`)

### Changed
- `mevy_ecs` default feature now includes `experimental` (breaking for users who don't want experimental macros)

---

## [0.3.1]

### Added
- `0.17` feature support for `mevy_ecs`
- `On<T>` observer trigger support for Bevy 0.17
- `trigger.event_target()` for Bevy 0.17+

### Fixed
- `scroll_position` field path for Bevy 0.17+ (`x`/`y` instead of `x_offset`/`y_offset`)

---

## [0.3.0]

### Added
- `0.16` feature support for all crates
- `0.16-rc` feature support for all crates
- `child_builder` support in `entity!{}`
- `EntityWorldMut` support in `entity!{}`
- `ancestors[]` array for nested children
- `try` conditional insertion in `entity!{}`
- `modify!{}` shorthand macro
- `cen![]`, `den![]`, `wen![]` alternative entity macros
- `gere![]`, `edre![]`, `geco![]`, `edco![]` experimental helpers

### Changed
- `border_radius` now maps to `Node::border_radius` in Bevy 0.18+
- `BoxShadow` field access updated for Bevy 0.16+ (array-based)
- `BorderColor` structure updated for Bevy 0.17+ (per-edge)
- `line_height` now uses separate `LineHeight` component in Bevy 0.18+

---

## [0.2.x]

### Added
- Initial `entity!{}` macro with hierarchy spawning
- Initial `ui!{}` macro with CSS-like notation
- Initial `code!{}` macro with hex/Val/UiRect shorthand
- Slim mode for `ui!{}`

---

## Migration

See [Migration Guide](migration.md) for details on migrating between Bevy versions.

[Unreleased]: https://github.com/dekirisu/mevy/compare/v0.3.2...HEAD
[0.3.2]: https://github.com/dekirisu/mevy/releases/tag/v0.3.2
[0.3.1]: https://github.com/dekirisu/mevy/releases/tag/v0.3.1
[0.3.0]: https://github.com/dekirisu/mevy/releases/tag/v0.3.0
