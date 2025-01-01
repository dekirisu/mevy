<h1 align="center">Macro Lab for Bevy</h1>
<p align="center">
    <a href="https://github.com/dekirisu/mevy" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/mevy-ee6677"></a><a href="https://crates.io/crates/mevy" style="position:relative"><img src="https://img.shields.io/crates/v/mevy"></a>
</p>

> [!IMPORTANT]
> At the moment, this crate is meant to provide macros only, without additional bevy plugins, resources, components or systems.
> So see it as **'bevy, but I can optionally type different things'**

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
> **Only relevant if you dig deeper into this crate:** The versions of those are not hard linked, since the macros can keep (or gain) features, even if the the syntax api has changed. So if one of those is `0.2.x` and the other `0.5.0` at some point, don't worry.

## More?
Yes, slowly but surely more fun macros!

