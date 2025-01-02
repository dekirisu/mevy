<h1 align="center">Macro Lab for Bevy</h1>
<p align="center">
    <a href="https://github.com/dekirisu/mevy" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/mevy-ee6677"></a><a href="https://crates.io/crates/mevy" style="position:relative"><img src="https://img.shields.io/crates/v/mevy"></a>
</p>

$${\color{lightblue}basically:\space\color{orange}'bevy,\space but\space write\space things\space differently'}$$

A **growing** set of $${\color{orange}m \color{lightblue}acros}$$ which add some witchcraft into $${\color{lightblue}b \color{orange}evy}$$, currently available: 🪄
- **Style Sheet Notation** for `bevy_ui` components (and your own) - `ui!(( width: 20px; ))`
- **Simplified Notation** for `Color`, `Val` and `UiRect` - `code!{ let red = #ff0000; //..any code }`

> [!IMPORTANT]
> This crate is meant to provide macros only - no additional bevy plugins, resources, components or systems

## CSS-like notation for bevy_ui
Using `ui!((..))` (inner round braces) will return a tuple of **mentioned components** only. See [this example](examples/ui_bundle.rs).
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

**Edge selection acts CSS-like:**
```rust
    border: 5px; // 5px to all edges
    border: 5px 2px; // 5px vertical, 2px horizontal
    border: 5px 2px 8px; // 5px top, 2px horizontal, 8px bottom
    border: 5px 2px 4px 1px // (clockwise) top right bottom left
```

**Corner selection, too:**
```rust
    border-radius: 5px; // 5px to all corners
    border-radius: 5px 0px; // 5px top-left/right, 0px bottom-left/right
    border-radius: 5px 2px 8px; // 5px top-left, 2px top-right, 8px bottom
    border-radius: 5px 2px 4px 1px // (clockwise) top-left top-right bottom-right bottom-left
```

## Code Replacement Macro
Using the `code!{}` macro simplifies constructing:
- `Color` by writing `#rgb`/`#rgba`/`#rrggbb`/`#rrggbbaa`
- `Val` by writing `0px`/`0%`/`0vw`/`0vh`/`0vmin`/`0vmax`/`@`(auto)
- `UiRect` by writing `[>0px]`/`[>0px 0px]`/`[>0px 0px 0px]`/`[>0px 0px 0px 0px]` (css-like)

So you can do fun things like:
```rust
let shadow = code!{BoxShadow{
    // use #... is replaced with Color, meaning you can e.g. use methods 
    color: #FF1265.mix(&#F93ECA,0.4).with_alpha(0.2),
    x_offset: 100px,
    y_offset: 50%,
    spread_radius: 3.1vh,
    blur_radius: 40.23vmax,
}}};
let color = code!{#FF0000};
// or multiple things in the macro
code!{
    let color2 = #00FF00;
    let color3 = #6600AA;
}
println!{"{color2:?}"}
```

## Version
Just to mention the obvious:
- Macros are token-based, meaning they aren't hard-bound to a specific bevy version
- **However:** These are mainly designed for **bevy 0.15** and onwards
- The closer your bevy version is to **0.15**, the more things will work

## Design
Crates are separated into:
- `crate/*/syntax`: token handling, meant to be reusable
- `crate/*`: actual macros, based on that 'syntax'

> [!NOTE]
> **Only relevant if you dig deeper into this crate:** The versions of those are not hard linked, since the macros can keep (or gain) features, even if the the syntax api has changed. So if one of those is `0.2.x` and the other `0.5.x` at some point, don't worry.
