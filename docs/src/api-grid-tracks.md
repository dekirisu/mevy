# Grid Track Syntax

mevy supports an extended syntax for CSS Grid track sizing in `ui!{}`.

## Basic Tracks

| Syntax | Result |
|---|---|
| `1px` | `1px` fixed track |
| `3%` | `3%` percentage track |
| `10fr` | `10` fraction track |
| `10!` | `10px` (exclamation = px) |

## Named Functions

| Syntax | Result |
|---|---|
| `auto` | `GridTrack::Auto` |
| `min_content` | `GridTrack::MinContent` |
| `max_content` | `GridTrack::MaxContent` |
| `min: 100px` | `GridTrack::min_content(100px)` |
| `max: 200px` | `GridTrack::max_content(200px)` |

## Minmax

````
minmax(100px 200px)
// -> min: 100px, max: 200px

minmax(auto 1fr)
// -> min: auto, max: 1fr

minmax(min_content max_content)
// -> min: min_content, max: max_content
````

## Repeated Tracks

### Fixed Repetition

````
10: 1px
// -> 10 tracks of 1px

3: 3%
// -> 3 tracks of 3%
````

### Auto Fill / Auto Fit

````
auto_fill: 1px
auto_fill: 10fr
auto_fit: 200px
auto_fit: min_content
````

### Fit Content

````
10fit_px    -> FitContentPx(10)
10fit       -> FitContentPx(10)
10fit!      -> FitContentPx(10)
10fit%      -> FitContentPercent(10)
````

## Usage Examples

### Auto Rows

````
ui!((
    grid_auto_rows: 1px 3% 10fr min_content;
));
````

### Auto Columns

````
ui!((
    grid_auto_columns: auto_fill: 1px;
));
````

### Template Rows

````
ui!((
    grid_template_rows: 10: 1px 3: 3%;
));
````

### Grid Placement

````
// Span
grid_row: span 3;
grid_column: span 2;

// Start/End
grid_row: start 1 end 3;
grid_column: start_span 2 end_end 5;
````
