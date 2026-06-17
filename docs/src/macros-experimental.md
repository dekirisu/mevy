# Experimental Helpers

These macros are enabled by the `experimental` feature flag. They're bare-bone and may change quickly.

> **Warning**: Experimental features are **unstable**. The API may change between versions without notice.

## Enable the Feature

```toml
mevy = { version = "0.3", features = ["0.18", "experimental"] }
```

## Alternative Entity Macros

Shorthand versions of `entity!{}` for specific world types:

### `cen![]` — Commands Entity

```rust
cen![..]     // spawn a `me: Entity`
cen![&..]    // edit a `me: Entity`
cen![*..]    // edit a `world: EntityCommands`
cen![#Marker|..]  // edit all Entities with Marker
```

### `den![]` — DeferredWorld Entity

```rust
den![..]     // spawn a `me: Entity`
den![&..]    // edit a `me: Entity`
den![*..]    // edit a `world: EntityCommands`
den![#Marker|..]  // edit all Entities with Marker
```

### `wen![]` — World Entity

```rust
wen![..]     // spawn a `me: Entity`
wen![&..]    // edit a `me: Entity`
wen![*..]    // edit a `world: EntityWorldMut`
wen![#Marker|..]  // edit all Entities with Marker
```

## Get Resource — `gere![]`

```rust
// Get (immutable)
let time = gere![Time].unwrap();

// Get (mutable)
let mut time = gere![mut Time].unwrap();
```

## Edit Resource — `edre![]`

Quickly edit a resource field:

```rust
edre![Time.delta = 0.016];
```

## Get Component — `geco![]`

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
```

## Edit Component — `edco![]`

Quickly edit a component field:

```rust
edco![MyComponent.field = 100];
```

## Deref Component

Prefix with `*` to dereference:

```rust
edco![*MyComponent.field = 100];
```

## Modify Shortcut

`modify!{...}` is a shorthand for `entity!{<|> ...}` — it always targets a specific entity via `me: Entity`:

````
modify!{
    Transform!;
    BackgroundColor(#ff0000);
}
````
