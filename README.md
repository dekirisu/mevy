<p align="center">
    <img src="https://github.com/user-attachments/assets/829a86b8-8dc4-4403-9da4-536daaefbd11">
</p>
<p align="center">
    <a href="https://github.com/dekirisu/mevy" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/mevy-ee6677"></a>
    <a href="https://crates.io/crates/mevy" style="position:relative"><img src="https://img.shields.io/crates/v/mevy"></a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative"><img src="https://img.shields.io/discord/515100001903312898"></a>
</p>

<p align="center">
    <img src="https://github.com/user-attachments/assets/891599e8-98a9-4d32-9f6e-6dfa76e51a31">
</p

A **growing** set of **m**acros which add some witchcraft into b**evy**, currently available: 🪄
- **Simpler** `Entity` spawning & modifying
- **Style Sheet Notation** for `bevy_ui` components (and your own) - `ui!(( width: 20px; ))`
- **Simplified Notation** for `Color`, `Val` and `UiRect` - `code!{ let red = #ff0000; //..any code }`

> [!IMPORTANT]
> This crate is meant to provide macros only - no additional bevy plugins, resources, components or systems

## Setup
Multiple bevy versions are supported and managed by features:
```toml
# bevy 0.17
mevy = {version="0.3",features=["0.17"]}

# bevy 0.16
mevy = {version="0.3",features=["0.16"]}

# bevy 0.15
mevy = {version="0.3",features=["0.15"]}
```

Then just `use` all of it:
```rust
use bevy::prelude::*;
use mevy::*;
```

## Simpler Hierarchy Spawning
Spawn children just by stating `[]` - the 'names' are just variables containing their `Entity`
- those variables can be used anywhere in the macro - even 'before'
- [read more](crates/ecs/README.md) or see [this example](examples/ecs_simple_spawn.rs).

```rust
entity!{
    <world> // pass a mut World, Commands, ... variable
    SpecificChild(optional_child_name); // insert component
    .observe(..);                       // use method
    > Pointer<Click>{..};               // quick observe (e.g. 'on click')
    // component/bundle;
    // .method(..);
    [optional_child_name][
        // component;
        // .method(..);
    ]
}
```

Modify entities in a 'quick and dirty' way: 
- [read more](crates/ecs/README.md) or see [this example](examples/entity_macro.rs).

```rust
entity!{
    <world|#Component> // select every entity with this Component
    <Children.iter()>  // > select all children of those
    <Children.iter()>  // >> infinitely chain those selectors
    .despawn();        // despawn all of the last selected
}
```

## CSS-like notation for bevy_ui
Using `ui!((..))` (inner round braces) will return a tuple of **mentioned components** only.
- read about **available fields**, custom fields & notation in [this readme](crates/ui/README.md)
- see [this example](examples/ui_bundle.rs).
```rust
// Slim Mode
c.spawn(ui!(
    w:100 h:100 bg:#fff round:6 border:5#f00 
    shadow:10%10%3+8#fa4 neat_outline
));

// CSS-Like Mode (does the same)
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

## Design
Crates are separated into:
- `crate/*/syntax`: token handling, meant to be reusable
- `crate/*`: actual macros, based on that 'syntax'

> [!NOTE]
> **Only relevant if you dig deeper into this crate:** The versions of those are not hard linked, since the macros can keep (or gain) features, even if the the syntax api has changed. So if one of those is `0.2.x` and the other `0.5.x` at some point, don't worry.
