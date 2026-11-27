mod date_picker {
    use chrono::{Datelike, Local, NaiveDate};
    use iced::widget::{button, container, row, text, Column, Row};
    use iced::{Center, Element, Fill, Length};

    const CELL: f32 = 36.0;
    const GAP: f32 = 2.0;
    const MONTHS: [&str; 12] = [
        "January", "February", "March", "April", "May", "June", "July", "August",
        "September", "October", "November", "December",
    ];
    const WEEKDAYS: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        Toggle,
        PrevMonth,
        NextMonth,
        Pick(NaiveDate),
        Today,
        Clear,
    }

    #[derive(Debug, Clone)]
    pub struct DatePicker {
        open: bool,
        year: i32,
        month: u32,
        selected: Option<NaiveDate>,
    }

    impl Default for DatePicker {
        fn default() -> Self {
            let today = Local::now().date_naive();
            Self {
                open: false,
                year: today.year(),
                month: today.month(),
                selected: None,
            }
        }
    }

    fn days_in_month(year: i32, month: u32) -> u32 {
        let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
        NaiveDate::from_ymd_opt(y, m, 1)
            .and_then(|d| d.pred_opt())
            .map(|d| d.day())
            .unwrap_or(30)
    }

    fn cell_size<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Element<'a, M> {
        container(content)
            .width(Length::Fixed(CELL))
            .height(Length::Fixed(CELL))
            .center_x(Length::Fixed(CELL))
            .center_y(Length::Fixed(CELL))
            .into()
    }

    impl DatePicker {
        pub fn selected(&self) -> Option<NaiveDate> {
            self.selected
        }

        pub fn update(&mut self, message: Message) {
            match message {
                Message::Toggle => self.open = !self.open,
                Message::PrevMonth => {
                    if self.month == 1 {
                        self.month = 12;
                        self.year -= 1;
                    } else {
                        self.month -= 1;
                    }
                }
                Message::NextMonth => {
                    if self.month == 12 {
                        self.month = 1;
                        self.year += 1;
                    } else {
                        self.month += 1;
                    }
                }
                Message::Pick(date) => {
                    self.selected = Some(date);
                    self.open = false;
                }
                Message::Today => {
                    let today = Local::now().date_naive();
                    self.selected = Some(today);
                    self.year = today.year();
                    self.month = today.month();
                    self.open = false;
                }
                Message::Clear => {
                    self.selected = None;
                    self.open = false;
                }
            }
        }

        pub fn view(&self) -> Element<'_, Message> {
            let width = Length::Fixed(CELL * 7.0 + GAP * 6.0 + 20.0);

            let label = match self.selected {
                Some(d) => d.format("%Y-%m-%d").to_string(),
                None => "Select a date".to_string(),
            };
            let trigger = button(
                row![
                    container(text(label)).width(Fill),
                    text(if self.open { "▲" } else { "▼" })
                ]
                .spacing(10),
            )
            .width(width)
            .on_press(Message::Toggle);

            let mut root = Column::new().spacing(6).push(trigger);

            if self.open {
                root = root.push(self.calendar(width));
            }
            root.into()
        }

        fn calendar(&self, width: Length) -> Element<'_, Message> {
            let today = Local::now().date_naive();

            // Header: ‹  Month Year  ›
            let header = row![
                button(text("‹").center())
                    .on_press(Message::PrevMonth)
                    .style(button::text),
                container(text(format!(
                    "{} {}",
                    MONTHS[(self.month - 1) as usize],
                    self.year
                )))
                .width(Fill)
                .center_x(Fill),
                button(text("›").center())
                    .on_press(Message::NextMonth)
                    .style(button::text),
            ]
            .align_y(Center);

            // Weekday labels
            let mut labels = Row::new().spacing(GAP);
            for name in WEEKDAYS {
                labels = labels.push(cell_size(text(name).size(13)));
            }

            // Day grid: 6 weeks x 7 days, Monday first
            let first = NaiveDate::from_ymd_opt(self.year, self.month, 1).expect("valid month");
            let offset = first.weekday().num_days_from_monday() as usize;
            let dim = days_in_month(self.year, self.month) as usize;

            let mut grid = Column::new().spacing(GAP);
            for week in 0..6 {
                let mut r = Row::new().spacing(GAP);
                for col in 0..7 {
                    let idx = week * 7 + col;
                    let cell: Element<'_, Message> = if idx >= offset && idx - offset < dim {
                        let day = (idx - offset + 1) as u32;
                        let date = NaiveDate::from_ymd_opt(self.year, self.month, day).unwrap();
                        let is_selected = self.selected == Some(date);
                        let is_today = date == today;

                        button(text(day.to_string()).center())
                            .width(Length::Fixed(CELL))
                            .height(Length::Fixed(CELL))
                            .padding(0)
                            .on_press(Message::Pick(date))
                            .style(move |theme, status| {
                                if is_selected {
                                    button::primary(theme, status)
                                } else if is_today {
                                    button::secondary(theme, status)
                                } else {
                                    button::text(theme, status)
                                }
                            })
                            .into()
                    } else {
                        cell_size(text(""))
                    };
                    r = r.push(cell);
                }
                grid = grid.push(r);
            }

            let footer = row![
                button(text("Today")).on_press(Message::Today).style(button::text),
                container(text("")).width(Fill),
                button(text("Clear")).on_press(Message::Clear).style(button::text),
            ];

            container(
                Column::new()
                    .spacing(8)
                    .push(header)
                    .push(labels)
                    .push(grid)
                    .push(footer),
            )
            .padding(10)
            .width(width)
            .style(container::bordered_box)
            .into()
        }
    }
}

use date_picker::DatePicker;
use iced::widget::{column, text};
use iced::Element;

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
