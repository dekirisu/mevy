# Entity Queries

Learn to query and modify entities with `entity!{}`.

## Basic Query

Target all entities with a component:

```rust
fn cleanup(mut cmd: Commands) {
    entity!{
        <world|#Dead>
        .despawn();
    }
}
```

## Chained Queries

Chain multiple selectors to drill down:

```rust
fn update_children(mut cmd: Commands) {
    entity!{
        <world|#Parent>              // 1. All entities with Parent
        <Children.iter()>            // 2. Their children
        <Children.iter()>            // 3. Grandchildren
        BackgroundColor(#ff0000);    // 4. Color them red
    }
}
```

## Conditional Query

Target a specific entity from a component:

```rust
fn update_target(mut cmd: Commands) {
    entity!{
        <world|#Player.get()>        // Single entity from Player component
        Transform {
            translation: Vec3::ZERO,
            !
        };
    }
}
```

## Resource-based Query

Target entities stored in a resource:

```rust
fn update_from_resource(mut cmd: Commands) {
    entity!{
        <world|@GameEntities.all()>  // Safe: does nothing if resource missing
        [children][
            Visibility::Visible;
        ]
    }
}

fn update_from_resource_risky(mut cmd: Commands) {
    entity!{
        <world|!@GameEntities.0>     // Risky: panics if None
        [children][
            Visibility::Visible;
        ]
    }
}
```

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

## World-based Queries

Use `World` directly for synchronous queries:

```rust
fn update_with_world(mut world: World) {
    entity!{
        <+world|>
        <|#OldComponent>
        <Children.iter()>
        BackgroundColor(#00ff00);
    }
}
```

## Deferring to Commands

When using `World`, results are queued to `Commands`:

```rust
fn update_deferred(mut world: World) {
    entity!{
        <+world|>
        <|#Target>
        .despawn();
        // This is queued to Commands automatically
    }
}
```

## `!` Default Shorthand

Use `!` to insert `Default::default()`:

```rust
entity!{
    <world|#Target>
    Transform!;                    // = Transform::default()
    Node{ padding:10px, ! };      // = Node{ padding:10px, ..default() }
    BackgroundColor(#ff0000);
}
```

## `try` Conditional Insertion

```rust
entity!{
    <world|#Target>
    try SomeBundle;  // Only inserts if SomeBundle is Some
}
```

## Free Code Block

Use `{..}` for custom logic with `this: EntityCommands`:

```rust
entity!{
    <world|#Target>
    {
        this.insert(Component);
        this.observe(on_click);
    }
}
```

## Safe vs Risky Selectors

| Selector | Behavior on None |
|---|---|
| `<|@Resource.get()>` | Safe: does nothing |
| `<|!@Resource.0>` | Risky: panics |
| `<|#Marker>` | Always works (may be empty) |
| `<|!#Marker>` | Risky: panics if none/multiple |
