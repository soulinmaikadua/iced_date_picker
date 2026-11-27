# iced_date_picker

A lightweight, dependency-minimal date picker widget for [iced](https://github.com/iced-rs/iced) 0.14.

It renders a trigger button that shows the selected date. Clicking it opens an inline month calendar where you can navigate between months, pick a day, jump to today, or clear the selection.

## Features

- Month grid with previous/next navigation
- Highlights today and the selected day
- Today and Clear shortcuts
- Monday-first weeks
- Follows the active iced theme
- Uses [`chrono`](https://crates.io/crates/chrono) `NaiveDate` for values

## Installation

```toml
[dependencies]
iced_date_picker = "0.1"
iced = "0.14"
chrono = "0.4"
```

> Replace `iced_date_picker` with the actual crate name if you publish under a different one.

## Usage

The picker follows the usual iced pattern: it owns its state, emits its own messages, and you map those into your app's messages.

```rust
use iced::widget::{column, text};
use iced::Element;
use iced_date_picker::{self as date_picker, DatePicker};

#[derive(Default)]
struct App {
    picker: DatePicker,
}

#[derive(Debug, Clone)]
enum Message {
    Picker(date_picker::Message),
}

fn update(app: &mut App, message: Message) {
    match message {
        Message::Picker(m) => app.picker.update(m),
    }
}

fn view(app: &App) -> Element<'_, Message> {
    let chosen = match app.picker.selected() {
        Some(d) => format!("Selected: {}", d.format("%A, %B %-d, %Y")),
        None => "Nothing selected".to_string(),
    };

    column![text(chosen), app.picker.view().map(Message::Picker)]
        .spacing(12)
        .padding(20)
        .into()
}

fn main() -> iced::Result {
    iced::run(update, view)
}
```

Run the bundled example with:

```sh
cargo run --example basic
```

## API

| Item                          | Description                                                                       |
| ----------------------------- | --------------------------------------------------------------------------------- |
| `DatePicker::default()`       | Creates a closed picker showing the current month with no date selected           |
| `DatePicker::update(message)` | Applies a picker `Message` to the state                                           |
| `DatePicker::view()`          | Returns an `Element<'_, date_picker::Message>`; map it into your own message type |
| `DatePicker::selected()`      | Returns the chosen date as `Option<NaiveDate>`                                    |

## Customizing

- **Week start:** change `num_days_from_monday` to `num_days_from_sunday` and reorder the weekday labels.
- **Cell size and spacing:** adjust the `CELL` and `GAP` constants.
- **Floating popup:** the calendar is laid out inline below the trigger. For an overlay, wrap the trigger and calendar in an iced `stack`.

## Compatibility

| iced | This crate |
| ---- | ---------- |
| 0.14 | 0.1.x      |

Iced makes breaking changes between releases, so each iced version needs a matching release of this crate.

## License

Licensed under either of

- Apache License, Version 2.0
- MIT license
