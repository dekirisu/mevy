# Entity Queries

This guide shows how to query and modify entities with `entity!{}`. Entity queries let you target and modify multiple entities at once, without writing separate query systems.

## The Mental Model

Entity queries work by **selecting** entities and then **applying** operations to them:

1. **Select** — the first `<...>` selector identifies the starting point (world type + entity target)
2. **Redirect** — additional `<...>` selectors drill down through children or components
3. **Apply** — operations after the selectors target whatever the last selector selected

Think of it like a pipeline: data flows through each selector, and the final operations apply to whatever emerges.

## Basic Query

Target all entities with a component:

```rust
fn cleanup(mut cmd: Commands) {
    entity!{
        <world|#*Dead.all()>
        .despawn();
    }
}
```

The `<world|#*Dead.all()>` selector targets every entity with the `Dead` component. The `.despawn()` call removes them all.

> [!NOTE]
> Use `#*Comp.all()` for an iterator over all entities with a component. Use `!#Comp` for a single entity (panics if none/multiple). Plain `#Comp` is not supported.

## Chained Queries

Chain multiple selectors to drill down through entities:

```rust
fn update_children(mut cmd: Commands) {
    entity!{
        <world|#*Parent.all()>      // 1. All entities with Parent
        <Children.iter()>            // 2. Their children
        <Children.iter()>            // 3. Grandchildren
        BackgroundColor(#ff0000);    // 4. Color them red
    }
}
```

Each `<...>` in the chain redirects the target. The final `BackgroundColor(#ff0000)` applies to all entities selected by the last redirection.

## Conditional Query

Target a specific entity from a component:

```rust
fn update_target(mut cmd: Commands) {
    entity!{
        <world|!#Player>             // Single entity from Player component
        Transform {
            translation: Vec3::ZERO,
            !
        };
    }
}
```

`!#Player` returns a single entity (panics if none/multiple). The `!` in the struct inserts `..Default::default()`.

## Resource-based Query

Target entities stored in a resource:

```rust
fn update_from_resource(mut cmd: Commands) {
    entity!{
        <world|@GameEntities.get()>  // Safe: does nothing if resource or value is None
        [children][
            Visibility::Visible;
        ]
    }
}

fn update_from_resource_risky(mut cmd: Commands) {
    entity!{
        <world|!@GameEntities.0>     // Risky: panics if resource or value is None
        [children][
            Visibility::Visible;
        ]
    }
}
```

The `@` prefix means "get entities from a resource". Safe selectors (`@Resource.get()`) silently skip if the resource or value is missing. Risky selectors (`!@Resource.0`) panic.

## Modify with Specific Entity

```rust
fn modify_specific(mut cmd: Commands, player: Entity) {
    entity!{
        <world|player>
        Transform {
            translation: Vec3::new(10.0, 0.0, 0.0),
            !
        };
        [child][
            Name("Child of player");
        ]
    }
}
```

The `|player` part selects a specific entity to modify. Everything after that operates on that entity.

## World-based Queries

Use `World` directly for synchronous queries:

```rust
fn update_with_world(mut world: World) {
    entity!{
        <+world|>
        <|#*OldComponent.all()>
        <Children.iter()>
        BackgroundColor(#00ff00);
    }
}
```

The `<+world|>` selector uses `World` directly instead of `Commands`. This is useful when you need to query the world synchronously.

## Deferring to Commands

When using `World`, results are queued to `Commands`:

```rust
fn update_deferred(mut world: World) {
    entity!{
        <+world|>
        <|#*Target.all()>
        .despawn();
        // This is queued to Commands automatically
    }
}
```

Operations on `World` are deferred to `Commands` automatically, so you don't need to worry about borrow checker conflicts.

## `!` Default Shorthand

Use `!` to insert `Default::default()`:

```rust
entity!{
    <world|#*Marker.all()>
    Transform!;                    // = Transform::default()
    Node{ padding:10px, ! };      // = Node{ padding:10px, ..default() }
    BackgroundColor(#ff0000);
}
```

The `!` at the end of a type calls `::default()`. Inside a struct, `!` inserts `..Default::default()`.

## `try` Conditional Insertion

```rust
entity!{
    <world|#*Target.all()>
    try SomeBundle;  // Only inserts if SomeBundle is Some
}
```

`try` conditionally inserts a bundle if it's `Some`. Useful for optional components that may or may not be present.

## Free Code Block

Use `{..}` for custom logic with `this: EntityCommands`:

```rust
entity!{
    <world|#*Target.all()>
    {
        this.insert(Component);
        this.observe(on_click);
    }
}
```

The `{..}` block gives you direct access to `this: EntityCommands` for arbitrary operations that don't fit the shorthand syntax.

## Safe vs Risky Selectors

| Selector | Behavior on None |
|---|---|
| `<\|@Resource.get()>` | Safe: does nothing |
| `<\|!@Resource.0>` | Risky: panics |
| `<\|#*Comp.all()>` | Always works (may be empty) |
| `<\|!#Marker>` | Risky: panics if none/multiple |

Use safe selectors when the entity might not exist (error handling). Use risky selectors when you're certain the entity exists (performance and clarity).

## See Also

- [Entity Selectors API](api-entity-selectors.md) — Full selector reference
- [entity!{} Documentation](macros-entity.md) — Complete macro reference
- [Migration Guide](migration.md) — Version-specific query behavior
