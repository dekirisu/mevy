<h1 align="center">Macro Lab for Bevy</h1>
<p align="center">
    <a href="https://github.com/dekirisu/mevy" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/mevy-ee6677"></a><a href="https://crates.io/crates/mevy" style="position:relative"><img src="https://img.shields.io/crates/v/mevy"></a>
</p>

## Code Replacement Macro
Using the `code!{}` macro will simplify constructing:
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

## More?
Yes, slowly but surely more fun macros!

