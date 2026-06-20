# `code!{}` — Code Replacement Macro

The `code!{}` macro replaces tedious patterns inside regular Rust code. It works anywhere you'd write a regular expression — in function arguments, struct fields, `let` bindings, etc. Unlike `ui!{}`, it doesn't return components; it returns the actual values you need.

## Why This Exists

In Bevy, you constantly write the same patterns:

```rust
// Before: verbose
let color = Srgba::hex("#FF1265").unwrap().into();
let width = Val::Px(100.0);
let rect = UiRect { top: Val::Px(10.0), right: Val::Px(5.0), bottom: Val::Px(10.0), left: Val::Px(5.0) };

// After: concise
let color = code!{#FF1265};
let width = code!{100px};
let rect  = code!{[>10px 5px]};
```

The macro is purely syntactic sugar — it expands to regular Rust at compile time. Your IDE autocomplete, type inference, and error messages all work normally.

## Hex Colors

Write `#hexcode` and it becomes a `Color`:

```rust
let color = code!{#FF0000};
// → Color::Srgba(Srgba::hex("#FF0000").unwrap())

let color = code!{#f00};
// → 3-digit shorthand: #ff0000

let color = code!{#ff000080};
// → 8-digit: includes alpha
```

Supported formats: 3-digit (`#rgb`), 4-digit (`#rgba`), 5-digit (`#rgb+alpha`), 6-digit (`#rrggbb`), 8-digit (`#rrggbbaa`). The compiler validates hex strings at compile time — invalid codes produce a clear error.

### Chaining Methods

Since the result is a `Color`, you can chain methods:

```rust
code!{BoxShadow{
    color: #FF1265.mix(&#F93ECA, 0.4).with_alpha(0.2),
}}
```

This is particularly useful for gradients, transparency adjustments, and color manipulation.

## Values (`Val`)

Write numbers with units and they become `Val`:

| Syntax | Result |
|---|---|
| `100px` | `Val::Px(100.)` |
| `50%` | `Val::Percent(50.)` |
| `3vw` | `Val::Vw(3.)` |
| `1vh` | `Val::Vh(1.)` |
| `2vmin` | `Val::VMin(2.)` |
| `4vmax` | `Val::VMax(4.)` |
| `@` | `Val::Auto` |

```rust
let width = code!{100px};
let height = code!{50%};
let auto = code!{@};
```

> [!NOTE]
> `@` inside `code!{}` means `Val::Auto`. This is different from `entity!{}` where `@` selects from resources. Don't use `@` in `code!{}` if you might confuse it with the `entity!{}` resource selector.

The `@` symbol maps to `Val::Auto`, which is commonly used for `width` and `height` in flex layouts.

## `UiRect`

Write `[>val1 val2 ...]` for CSS-like edge notation:

```rust
// Single value → all edges
code!{[>10px]}
// → UiRect { top: 10px, right: 10px, bottom: 10px, left: 10px }

// Two values → vertical, horizontal
code!{[>10px 5px]}
// → top/bottom: 10px, left/right: 5px

// Three values → top, horizontal, bottom
code!{[>10px 5px 8px]}

// Four values → top, right, bottom, left
code!{[>10px 5px 8px 3px]}
```

The same edge-selection conventions as CSS apply: 1 value = all, 2 = vertical/horizontal, 3 = top/horizontal/bottom, 4 = clockwise.

## Default Shorthand

Use `!` to insert `Default::default()`:

```rust
code!{Node{
    width: 100px,
    height: 100px,
    !        // ..Default::default()
}}
```

- `!` at end of token (no following `;`) → `::default()`
- `!` followed by `;` → `::default()`
- `!` inside a struct → `..Default::default()`

## Multiple Expressions

Write multiple statements inside the macro:

```rust
code!{
    let color1 = #FF0000;
    let color2 = #00FF00;
    let color3 = #0000FF;
}
```

This is useful for defining several values at once without repeating the macro syntax.

## Complete Example

```rust
use bevy::prelude::*;
use mevy::*;

pub fn main() {
    println!("{:#?}", code!{BoxShadow(vec![ShadowStyle{
        color:         #FF1265.mix(&#F93ECA, 0.4).with_alpha(0.2),
        x_offset:      100px,
        y_offset:      50%,
        spread_radius: 3.1vh,
        blur_radius:   40.23vmax,
    }])});
}
```

## See Also

- [Macros Overview](macros-overview.md) — All macro families
- [Quick Start](quick-start.md) — First app with all macros
