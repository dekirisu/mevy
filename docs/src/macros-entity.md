# `entity!{}` — Entity Spawning & Modification

The `entity!{}` macro lets you spawn entities, build hierarchies, and run queries in a single expression. It replaces Bevy's `.spawn().with_children()` callback pattern and `Commands::spawn()` boilerplate.

## The Mental Model

Think of `entity!{}` as a tree builder. The first entry `<...>` selects **where** to operate (which world, which entity). Everything after that operates on the selected target. Children are created with `[name][...]` blocks, which nest recursively.

## Basic Spawning

```rust
entity!{
    Transform!;
    Name("Hello");
}
```

This spawns a new entity with `Transform::default()` and `Name("Hello")`. The `!` shorthand expands to `::default()` — more on that below.

## Hierarchy with Children

Use `[name][...]` to create child entities:

```rust
entity!{
    Node { size: 100px 100px; ! }
    BackgroundColor(#ff0000);
    
    [child1][
        Node { size: 50px 50px; ! }
        BackgroundColor(#00ff00);
    ]
    
    [named_child][
        Node { size: 50px 50px; ! }
        BackgroundColor(#0000ff);
    ]
}
```

Children can be referenced by name **anywhere** — even before they're defined. This is useful when you want to set a component on a child from a parent:

```rust
entity!{
    Node { entity: named_child; }  // reference before definition
    [named_child][
        // ...
    ]
}
```

## World/Entity Selection

The first entry `<...>` determines **where** to operate and **what** to target:

### Spawning (creating new entities)

| Selector | World Type | Entity |
|---|---|---|
| *(none)* | `Commands` | new (spawned) |
| `world` | `Commands` (named) | new |
| `commands` | `Commands` (named) | new |
| `+world` | `World` | new |
| `-world` | `DeferredWorld` | new |
| `*this` | `EntityCommands` | new |
| `^cbuild` | `ChildBuilder` | new |
| `+*this` | `EntityWorldMut` | new |

### Modifying (targeting existing entities)

| Selector | World Type | Entity |
|---|---|---|
| `commands\|entity` | `Commands` | specific Entity |
| `\|entity` | `Commands` | specific Entity |
| `commands\|` | `Commands` | `me: Entity` |
| `\|` | `Commands` | `me: Entity` |
| `+world\|entity` | `World` | specific Entity |
| `-world\|entity` | `DeferredWorld` | specific Entity |
| `*this\|entity` | `EntityCommands` | specific Entity |
| `^cbuild\|entity` | `ChildBuilder` | specific Entity |
| `+*this\|entity` | `EntityWorldMut` | specific Entity |

### Component Query Selectors

For targeting entities by component:

| Selector | Behavior |
|---|---|
| `!#Marker` | Single entity (panics if none/multiple) |
| `!#Marker.0` | Entity field on component (panics if None) |
| `#*Comp.all()` | Iterator over entities from component |

> [!WARNING]
> Plain `#Marker` (without `!` or `*`) is **not** supported. Use `!#Marker` for a single entity or `#*Comp.all()` for an iterator.

### Resource Selectors

For targeting entities stored in a resource:

| Selector | Behavior |
|---|---|
| `@Resource.get()` | Safe: does nothing if resource or value is `None` |
| `@*Resource.all()` | Iterate over entity iterator from resource |
| `!@Resource.0` | Risky: panics if resource or value is `None` |

> [!WARNING]
> Resource/component selectors (`@`, `#`) only leak child entities, not the root. If you use `<|!#Marker>` to select a specific entity, spawned children are still available outside the macro, but the selected entity itself is not.

See [Entity Selectors API](api-entity-selectors.md) for the complete selector reference.

## Entity Redirection

After the initial selector, you can chain **redirections** to drill down through entities:

```rust
entity!{
    <world|#*Parent.all()>       // 1. Start with every entity with Parent
    <Children.iter()>            // 2. Redirect to their children
    <Children.iter()>            // 3. Redirect to grandchildren
    BackgroundColor(#ff0000);    // 4. Color them red
    .despawn();                  // 5. Then despawn them
}
```

Each `<...>` in the chain redirects the target. The final `BackgroundColor(#ff0000)` applies to all entities selected by the last redirection.

## Leaking & Returning

By default, the spawned entity variable `me` is scoped to the macro block. Named children are always available. Symbols at the **end** of the macro control additional visibility:

| Symbol | Behavior |
|---|---|
| `>` | Make `me` available outside the macro block |
| `<` | Return the root entity |
| `@` | Wrap the entire body in a closure `|mut world: Commands|` |

### Leaking `me`

```rust
entity!{
    Bundle::new(5);
    [child][]
>}
// me;      // the root entity (now available outside)
// child;   // the named child entity (always available)
```

### Returning

```rust
let enty = entity!{
    Bundle::new(5);
<};
```

### Closure

```rust
entity!{
    Bundle::new(5);
@};
// The entire body is wrapped in: |mut world: Commands| { ... }
// Useful for system parameters that need a `Commands` argument.
```

> [!NOTE]
> Resource/component selectors (`@`, `#`) only leak child entities, not the root.

## Quick Observer

Attach observers directly on entities without separate registration:

```rust
entity!{
    // Single '>' — 'this' = EntityCommands
    > Pointer<Click> {
        this.despawn();
    }
    
    // Double '>>' — 'entity' = Entity, 'world' = &mut World (inside auto-wrapped queue)
    >> Pointer<Click> {
        // 'entity' and 'world' are available here automatically
        // The block is wrapped in world.queue(...) by the macro
    }
}
```

The single `>` variant gives you `this: EntityCommands` inside the block. The double `>>` variant gives you `world: &mut World` and `entity: Entity`, letting you queue commands to be executed later. Note that the user's block is automatically wrapped in `world.queue(...)`, so you don't need to write it yourself.

## Shorthand Notations

Inside `entity!{}`, mevy recognizes several shorthand patterns:

| Syntax | Expands to |
|---|---|
| `Bundle!` | `Bundle::default()` |
| `Bundle{a:3,!}` | `Bundle{a:3, ..Default::default()}` |
| `Bundle: 3` | `Bundle::new(3)` |
| `bundle_fn: 3` | `bundle_fn(3)` |
| `macro!: 3, 4` | `macro!{3, 4}` |
| `.method: 5` | `.method(5)` |

The `!` at the end of a type calls `::default()`. Inside a struct, `!` inserts `..Default::default()`. The `:` after a type calls its `new()` method. These are all syntactic sugar — the macro expands them to regular Rust.

> [!NOTE]
> Some of these shorthands (`!`, `:`) are also recognized inside `code!{}`. The behavior is the same, but the context is different (inline values vs entity commands).

## `try` and `{..}`

```rust
entity!{
    try SomeBundle;  // Only inserts if SomeBundle is Some
    
    {
        this.insert(Component);
        this.observe(on_click);
    }
}
```

`try` conditionally inserts a bundle if it's `Some`. The `{..}` block gives you direct access to `this: EntityCommands` for arbitrary operations.

> [!NOTE]
> `try` is `entity!{}`-only. The `!` shorthand (for `::default()`) is shared with `code!{}` but behaves slightly differently in each context.

## Ancestors Array

Inside nested children, `ancestors[]` gives you parent entities:

```rust
[[[[[
    Component{ entity: ancestors[3] };
]]]]]
```

- `ancestors[0]` = direct parent
- `ancestors[n]` = root entity of the current macro call

This is useful when you need to reference a specific ancestor from a deeply nested child.

## Complete Example

```rust
fn startup(mut world: Commands) {
    entity!{
        Camera2d;
        BackgroundColor(#0a0a0a);
        
        ui!((
            size: 200px 150px;
            background: #1a1a2e;
            border: 3px #ee6677;
            border_radius: 8px;
            justify_content: center;
            align_items: center;
        ));
        
        [button][
            ui!((
                size: 120px 40px;
                background: #ee6677;
                border_radius: 6px;
                justify_content: center;
                align_items: center;
            ));
            Text::new("Click me");
            > Pointer<Click> {
                this.insert(BackgroundColor(#ff4455));
            };
        ]
    }
}
```

## See Also

- [Entity Selectors API](api-entity-selectors.md) — Full selector reference
- [Entity Queries Guide](guides-entity-queries.md) — Query and modify patterns
- [Migration Guide](migration.md) — Version-specific changes
