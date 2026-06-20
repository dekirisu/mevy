# Experimental Helpers

> [!WARNING]
> Experimental features are **unstable**. The API may change between versions without notice. Do not rely on them in production code.

## Enable the Feature

```toml
mevy = { version = "0.3", features = ["0.18", "experimental"] }
```

These helpers are useful for prototyping and daily development. They solve common patterns with minimal syntax, but since they're experimental, their behavior may change.

## Alternative Entity Macros

Shorthand versions of `entity!{}` for specific world types. These are **always available** (no feature flag needed):

### `cen![]` — Commands Entity

```rust
cen![..]     // spawn a `me: Entity`
cen![&..]    // edit a `me: Entity`
cen![*..]    // edit a `world: EntityCommands`
cen![#Marker|..]  // entity target becomes 'Marker' (named entity, not component query)
```

### `den![]` — DeferredWorld Entity

```rust
den![..]     // spawn a `me: Entity`
den![&..]    // edit a `me: Entity`
den![*..]    // edit a `world: EntityCommands`
den![#Marker|..]  // entity target becomes 'Marker' (named entity, not component query)
```

### `wen![]` — World Entity

```rust
wen![..]     // spawn a `me: Entity`
wen![&..]    // edit a `me: Entity`
wen![*..]    // edit a `world: EntityWorldMut`
wen![#Marker|..]  // entity target becomes 'Marker' (named entity, not component query)
```

The prefix indicates the world type: `c` = Commands, `d` = DeferredWorld, `w` = World. The `&` prefix before the content selects a specific entity to modify.

> [!WARNING]
> The `#` prefix in `cen![#Marker|..]` is consumed by the macro and does **not** trigger a component query. The entity target becomes the named entity `Marker`, not a component selector.

## `modify!{}` — Entity Modification Shortcut

`modify!{...}` is a shorthand for `entity!{<|> ...}` — it always targets a specific entity via `me: Entity`:

```rust
modify!{
    Transform!;
    BackgroundColor(#ff0000);
}
```

Use this when you have an `Entity` variable named `me` and want to modify it quickly.

## Get Resource — `gere![]`

> [!NOTE]
> Requires the `experimental` feature flag.

```rust
// Get (immutable)
let time = gere![Time].unwrap();

// Get (mutable)
let mut time = gere![mut Time].unwrap();

// With auto .unwrap()
let time = gere![Time!];
```

The `mut` keyword before the type requests a mutable reference. The `!` suffix (after the type) adds `.unwrap()` automatically.

## Edit Resource — `edre![]`

> [!NOTE]
> Requires the `experimental` feature flag.

Quickly edit a resource field:

```rust
edre![Time.delta = 0.016];
```

This is equivalent to:
```rust
if let Some(mut data) = world.get_resource_mut::<Time>() {
    data.delta = 0.016;
}
```

## Get Component — `geco![]`

> [!NOTE]
> Requires the `experimental` feature flag.

```rust
// Get (immutable)
let val = geco![MyComponent].unwrap();

// Get (mutable)
let mut val = geco![mut MyComponent].unwrap();

// Get (cloned)
let val = geco![MyComponent*].unwrap();

// Check existence
if geco![MyComponent?] {
    // component exists
}

// With auto .unwrap()
let val = geco![MyComponent!];
```

The suffixes modify behavior:
- `mut` before the type → mutable reference
- `*` after the type → cloned value
- `?` after the type → returns `bool` (true if component exists)
- `!` after the type → adds `.unwrap()` automatically

## Edit Component — `edco![]`

> [!NOTE]
> Requires the `experimental` feature flag.

Quickly edit a component field:

```rust
edco![MyComponent.field = 100];
```

This is equivalent to:
```rust
if let Some(mut data) = world.get_mut::<MyComponent>(me) {
    data.field = 100;
}
```

### Deref Component

Prefix with `*` to dereference:

```rust
edco![*MyComponent.field = 100];
```

This dereferences the component before accessing the field, useful for `Option<T>` or `Box<T>` components.
