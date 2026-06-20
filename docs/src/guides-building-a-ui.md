# Building a UI

Learn to compose complex UI layouts with `ui!{}` and `entity!{}`.

## Basic Panel

```rust
fn spawn_panel(mut cmd: Commands) {
    entity!{
        <cmd>
        
        // Outer panel
        ui!((
            size: 300px 200px;
            background: #1a1a2e;
            border: 2px #ee6677;
            border_radius: 12px;
            padding: 16px;
            gap: 8px;
            flex_direction: column;
        ));
        
        // Title
        [title][
            ui!((
                size: 100% auto;
                justify_content: center;
            ));
            Text::new("My Panel");
        ]
        
        // Content area
        [content][
            ui!((
                size: 100% 1fr;
                background: #16213e;
                border_radius: 8px;
                justify_content: center;
                align_items: center;
            ));
            Text::new("Content goes here");
        ]
        
        // Footer with buttons
        [footer][
            ui!((
                size: 100% auto;
                justify_content: center;
                gap: 8px;
            ));
            
            [ok_button][
                ui!((
                    size: 80px 32px;
                    background: #ee6677;
                    border_radius: 6px;
                    justify_content: center;
                    align_items: center;
                ));
                Text::new("OK");
                > Pointer<Click> {
                    this.insert(BackgroundColor(#ff8899));
                };
            ]
            
            [cancel_button][
                ui!((
                    size: 80px 32px;
                    background: #444;
                    border_radius: 6px;
                    justify_content: center;
                    align_items: center;
                ));
                Text::new("Cancel");
            ]
        ]
    }
}
```

## Grid Layout

```rust
fn spawn_grid(mut cmd: Commands) {
    entity!{
        <cmd>
        
        ui!((
            size: 100% 100%;
            display: grid;
            grid_template_columns: 1fr 2fr 1fr;
            grid_template_rows: auto 1fr auto;
            gap: 8px;
            padding: 16px;
        ));
        
        [header][
            ui!((
                grid_column: span 3;
                size: 100% 40px;
                background: #ee6677;
                border_radius: 8px;
                justify_content: center;
                align_items: center;
            ));
            Text::new("Header");
        ]
        
        [sidebar][
            ui!((
                grid_row: span 2;
                size: 100% 100%;
                background: #16213e;
                border_radius: 8px;
            ));
            Text::new("Sidebar");
        ]
        
        [main][
            ui!((
                grid_column: span 2;
                grid_row: span 2;
                size: 100% 100%;
                background: #1a1a2e;
                border_radius: 8px;
            ));
            Text::new("Main Content");
        ]
        
        [footer][
            ui!((
                grid_column: span 3;
                size: 100% 30px;
                background: #333;
                border_radius: 8px;
                justify_content: center;
                align_items: center;
            ));
            Text::new("Footer");
        ]
    }
}
```

## Reusable Components

Define reusable components with `ui!{}` function mode:

```rust
// Define a button prefab
ui!{
    mevy_button(
        size: 120px 40px;
        background: #ee6677;
        border_radius: 6px;
        justify_content: center;
        align_items: center;
    )?
}

// Define a card prefab
ui!{
    mevy_card(
        size: 200px 150px;
        background: #1a1a2e;
        border: 2px #333;
        border_radius: 12px;
        padding: 12px;
        gap: 8px;
        flex_direction: column;
    )?
}

// Use them
fn spawn_with_prefabs(mut cmd: Commands) {
    entity!{
        [card][
            mevy_card();
            Text::new("Card Title");
            [btn][
                mevy_button();
                Text::new("Click");
            ]
        ]
    }
}
```

## Hover Effects

```rust
entity!{
    [hoverable][
        ui!((
            size: 100px 100px;
            background: #ee6677;
            border_radius: 8px;
        ));
        
        > Pointer<Over> {
            this.insert(ui!((
                background: #ff8899;
                box_shadow: 0px 0px 10px #ee6677;
            )));
        };
        
        > Pointer<Out> {
            this.insert(ui!((
                background: #ee6677;
                box_shadow: 0px 0px 0px #000;
            )));
        };
    ]
}
```
