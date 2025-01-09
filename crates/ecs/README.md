<p align="center">
    <img src="https://github.com/user-attachments/assets/829a86b8-8dc4-4403-9da4-536daaefbd11">
</p>
<p align="center">
    <a href="https://github.com/dekirisu/mevy" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/mevy-ee6677"></a>
    <a href="https://crates.io/crates/mevy_ecs" style="position:relative"><img src="https://img.shields.io/crates/v/mevy_ecs"></a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative"><img src="https://img.shields.io/discord/515100001903312898"></a>
</p>

This crate is part of [mevy](https://github.com/dekirisu/mevy) (tl;dr: more neat macros) so take a look! 🦆

## A simpler way of spawning
The macro `spawn!{..}` allows you to spawn hierarchies with this patter:
```rust
spawn!{
    // component;
    // .method(..);
    [optional_child_name][
        // component;
        // .method(..);
    ]
}
```

This macro expects a `world` variable - which is anything that can spawn things.
- This means it works with `Commands`, `World`, things that `impl ChildBuild`, ...
```rust
fn startup(mut world: Commands){
    spawn!{Camera2d::default()}
}
```

The 'Child Names' are variables containing the child entity.
- Inside the macro they can be used anywhere, even 'before' you wrote them
- If none is provided - one will be generated: `e{number}`: `e0`, `e1`, ...
```rust 
spawn!{
    Component{ entity: named_child };
    [named_child][
        // components & methods
    ]
}
``` 

This is 'token based', which means it preserves LSP greatness.
```rust
spawn!{
    // typing . will fire LSPs autocomplete
    .obs // would suggest 'observer'
}
```

## Synergies with [mevy](https://github.com/dekirisu/mevy) 
Using `mevy_core` macro `code!{}`, you can pair it with `code!{}` to write `Color`s and `Val`s neater:
```rust
code!{spawn!{
    BackgroundColor(#ff0000);
    Node{ width:50px, height:10%, margin:[>5px], ..default() }
    [
        // child ui
    ]
}}
```

Using `mevy_ui` macro `ui!{}`, it's a bit like html/css:
```rust
spawn!{
    ui!((
        size: 5px;
        box_shadow: 1px 2px 3px 4px #ff0000;
        background: cyan;
    ));
    [inner_box][ui!((
        size: 80%;
        background: green;
    ))]
}
```




