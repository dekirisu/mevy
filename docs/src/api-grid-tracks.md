# Grid Track Syntax

mevy supports an extended syntax for CSS Grid track sizing in `ui!{}`. Grid tracks define the columns and rows of a grid layout, controlling their size and behavior.

## How Grid Tracks Work

CSS Grid uses **tracks** (rows and columns) to define the structure of a grid. Each track has a size (like `100px` or `1fr`) and may have a minimum and maximum size. mevy translates CSS-like track syntax into Bevy's `GridTrack` types.

## Basic Tracks

These are the simplest track sizing options — a single value that defines the track size:

| Syntax | Result |
|---|---|
| `1px` | `1px` fixed track |
| `3%` | `3%` percentage track |
| `10fr` | `10` fraction track |
| `10!` | `10px` (exclamation = px) |

Fixed tracks have an exact pixel size. Percentage tracks are relative to the container. Fraction (`fr`) tracks divide remaining space proportionally.

## Named Functions

Instead of raw values, mevy also accepts named sizing functions that describe the track's behavior:

| Syntax | Result |
|---|---|
| `auto` | `GridTrack::Auto` |
| `min_content` | `GridTrack::MinContent` |
| `max_content` | `GridTrack::MaxContent` |
| `min: 100px` | `GridTrack::min_content(100px)` |
| `max: 200px` | `GridTrack::max_content(200px)` |

`auto` sizes to the content. `min_content` sizes to the minimum needed. `max_content` sizes to the maximum needed. The `min:` and `max:` variants let you set a minimum or maximum size constraint.

## Minmax

The `minmax(min max)` function creates a track with both a minimum and maximum size constraint:

```
minmax(100px 200px)
// -> min: 100px, max: 200px

minmax(auto 1fr)
// -> min: auto, max: 1fr

minmax(min_content max_content)
// -> min: min_content, max: max_content
```

`minmax(min max)` creates a track that can grow and shrink between the two values. The minimum is the smallest the track can be; the maximum is how large it can grow.

## Repeated Tracks

Tracks can be repeated using the `N: track` syntax or named repetition keywords.

### Fixed Repetition

Use `N: track` to create `N` identical tracks:

```
10: 1px
// -> 10 tracks of 1px

3: 3%
// -> 3 tracks of 3%
```

The `N: track` syntax repeats a track `N` times. Useful for creating evenly-sized columns or rows.

### Auto Fill / Auto Fit

These keywords create tracks dynamically based on available space:

```
auto_fill: 1px
auto_fill: 10fr
auto_fit: 200px
auto_fit: min_content
```

`auto_fill` creates as many tracks as will fit, including empty ones. `auto_fit` creates tracks only if there's content for them — empty tracks are removed. Both are useful for responsive layouts.

### Fit Content

`fit_content` creates tracks sized to their content, with a minimum size constraint:

```
10fit_px    -> FitContentPx(10)
10fit!      -> FitContentPx(10)
10fit%      -> FitContentPercent(10)
```

`fit_content` creates a track sized to its content, with a minimum size. The `fit` suffix is attached to a number; `fit_px` or `fit!` uses pixel values, `fit%` uses percentages.

## Grid Placement

`grid_row` and `grid_column` use `GridPlacement` to position items within the grid. No value defaults to `GridPlacement::DEFAULT`:

### Span

```rust
grid_row: span 3;      // span 3 tracks starting from current position
grid_column: span 2;   // span 2 tracks starting from current position
```

### Start Only

```rust
grid_row: start 1;     // start at track 1, span 1 track
grid_column: start 2;  // start at track 2, span 1 track
```

### Start + End

```rust
grid_row: start 1 end 3;   // start at track 1, end at track 3
grid_column: start 2 end 5; // start at track 2, end at track 5
```

### End Only

```rust
grid_row: end 3;         // end at track 3, span 1 track
grid_column: end 5;      // end at track 5, span 1 track
```

## Usage Examples

Here are practical examples of each track type in context:

### Auto Rows

Use `grid_auto_rows` to define how rows are created when items don't specify a row:

```rust
ui!((
    grid_auto_rows: 1px 3% 10fr min_content;
));
```

### Auto Columns

Similar to auto rows, but for columns:

```rust
ui!((
    grid_auto_columns: auto_fill: 1px;
));
```

### Template Rows

Define a fixed grid structure with `grid_template_rows`. The `N:` prefix repeats a track `N` times:

```rust
ui!((
    grid_template_rows: 10: 1px 3: 3%;
));
```

### Grid Placement

Use `grid_row` and `grid_column` to control which tracks an item spans:

```rust
// Span
grid_row: span 3;
grid_column: span 2;

// Start/End
grid_row: start 1 end 3;
grid_column: start 2 end 5;
```
