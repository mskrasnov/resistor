use iced::Length;
use iced::Pixels;

use iced::widget::column;
use iced::widget::horizontal_space;
use iced::widget::row;
use iced::widget::scrollable;
use iced::widget::vertical_space;

use iced::Element;
use iced::Sandbox;
use iced::Settings;

use sapphire_ui::widgets::button;
use sapphire_ui::widgets::container;
use sapphire_ui::widgets::radio;
use sapphire_ui::widgets::small_text;
use sapphire_ui::widgets::text;
use sapphire_ui::widgets::widget_group;
use sapphire_ui::widgets::title_text;

use sapphire_ui::theme::text::*;

use crate::colors::Color;

pub fn ui() -> iced::Result {
    let settings = Settings {
        window: iced::window::Settings {
            size: iced::Size {
                width: 525.,
                height: 510.,
            },
            resizable: false,
            ..Default::default()
        },
        id: Some("resistor".to_string()),
        antialiasing: true,
        default_text_size: Pixels(15.),
        ..Settings::default()
    };
    App::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    Line1Changed(Color),
    Line2Changed(Color),
    Line3Changed(Color),
    Line4Changed(Color),
    CalculateButtonPressed,
    DonateButtonPressed,
    AboutButtonPressed,
}

pub struct App {
    header: String,
    subheader: Option<String>,
    line1: Color,
    line2: Color,
    line3: Color,
    line4: Color,
}

impl Default for App {
    fn default() -> Self {
        Self {
            header: "Укажите полосы и нажмите «Вычислить»".to_string(),
            subheader: Some("Значения вычислений появятся здесь".to_string()),
            line1: Color::Black,
            line2: Color::Black,
            line3: Color::Black,
            line4: Color::Black,
        }
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        format!(
            "{} ver.{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let header = {
            let mut data = column![title_text(&self.header),];
            if let Some(subtext) = &self.subheader {
                // data = data.push(small_text(subtext));
                data = column![text(&self.header).size(16.), small_text(subtext)];
            }
            widget_group(data).width(Length::Fill)
        };

        let line1_color_choice = Some(self.line1);
        let line1_colors = column![
            text("Полоса №1").variant(TextVariant::Dimmed),
            radio(
                Color::Black.to_str(),
                Color::Black,
                line1_color_choice,
                Message::Line1Changed
            ),
            radio(
                Color::Brown.to_str(),
                Color::Brown,
                line1_color_choice,
                Message::Line1Changed
            ),
            radio(
                Color::Red.to_str(),
                Color::Red,
                line1_color_choice,
                Message::Line1Changed,
            ),
            radio(
                Color::Orange.to_str(),
                Color::Orange,
                line1_color_choice,
                Message::Line1Changed,
            ),
            radio(
                Color::Yellow.to_str(),
                Color::Yellow,
                line1_color_choice,
                Message::Line1Changed,
            ),
            radio(
                Color::Green.to_str(),
                Color::Green,
                line1_color_choice,
                Message::Line1Changed,
            ),
            radio(
                Color::Blue.to_str(),
                Color::Blue,
                line1_color_choice,
                Message::Line1Changed,
            ),
            radio(
                Color::Purple.to_str(),
                Color::Purple,
                line1_color_choice,
                Message::Line1Changed,
            ),
            radio(
                Color::Gray.to_str(),
                Color::Gray,
                line1_color_choice,
                Message::Line1Changed,
            ),
            radio(
                Color::White.to_str(),
                Color::White,
                line1_color_choice,
                Message::Line1Changed,
            ),
        ]
        .spacing(5);

        let line2_color_choice = Some(self.line2);
        let line2_colors = column![
            text("Полоса №2").variant(TextVariant::Dimmed),
            radio(
                Color::Black.to_str(),
                Color::Black,
                line2_color_choice,
                Message::Line2Changed
            ),
            radio(
                Color::Brown.to_str(),
                Color::Brown,
                line2_color_choice,
                Message::Line2Changed
            ),
            radio(
                Color::Red.to_str(),
                Color::Red,
                line2_color_choice,
                Message::Line2Changed,
            ),
            radio(
                Color::Orange.to_str(),
                Color::Orange,
                line2_color_choice,
                Message::Line2Changed,
            ),
            radio(
                Color::Yellow.to_str(),
                Color::Yellow,
                line2_color_choice,
                Message::Line2Changed,
            ),
            radio(
                Color::Green.to_str(),
                Color::Green,
                line2_color_choice,
                Message::Line2Changed,
            ),
            radio(
                Color::Blue.to_str(),
                Color::Blue,
                line2_color_choice,
                Message::Line2Changed,
            ),
            radio(
                Color::Purple.to_str(),
                Color::Purple,
                line2_color_choice,
                Message::Line2Changed,
            ),
            radio(
                Color::Gray.to_str(),
                Color::Gray,
                line2_color_choice,
                Message::Line2Changed,
            ),
            radio(
                Color::White.to_str(),
                Color::White,
                line2_color_choice,
                Message::Line2Changed,
            ),
            radio(
                Color::Gold.to_str(),
                Color::Gold,
                line2_color_choice,
                Message::Line2Changed,
            ),
        ]
        .spacing(5);

        let line3_color_choice = Some(self.line3);
        let line3_colors = column![
            text("Полоса №3").variant(TextVariant::Dimmed),
            radio(
                Color::Black.to_str(),
                Color::Black,
                line3_color_choice,
                Message::Line3Changed
            ),
            radio(
                Color::Brown.to_str(),
                Color::Brown,
                line3_color_choice,
                Message::Line3Changed
            ),
            radio(
                Color::Red.to_str(),
                Color::Red,
                line3_color_choice,
                Message::Line3Changed,
            ),
            radio(
                Color::Orange.to_str(),
                Color::Orange,
                line3_color_choice,
                Message::Line3Changed,
            ),
            radio(
                Color::Yellow.to_str(),
                Color::Yellow,
                line3_color_choice,
                Message::Line3Changed,
            ),
            radio(
                Color::Green.to_str(),
                Color::Green,
                line3_color_choice,
                Message::Line3Changed,
            ),
            radio(
                Color::Blue.to_str(),
                Color::Blue,
                line3_color_choice,
                Message::Line3Changed,
            ),
            radio(
                Color::Purple.to_str(),
                Color::Purple,
                line3_color_choice,
                Message::Line3Changed,
            ),
            radio(
                Color::Gray.to_str(),
                Color::Gray,
                line3_color_choice,
                Message::Line3Changed,
            ),
            radio(
                Color::White.to_str(),
                Color::White,
                line3_color_choice,
                Message::Line3Changed,
            ),
            radio(
                Color::Gold.to_str(),
                Color::Gold,
                line3_color_choice,
                Message::Line3Changed,
            ),
            radio(
                Color::Silver.to_str(),
                Color::Silver,
                line3_color_choice,
                Message::Line3Changed,
            ),
        ]
        .spacing(5);

        let line4_colors_choice = Some(self.line4);
        let line4_colors = column![
            text("Полоса №4").variant(TextVariant::Dimmed),
            radio(
                Color::Silver.to_str(),
                Color::Silver,
                line4_colors_choice,
                Message::Line4Changed,
            ),
            radio(
                Color::Gold.to_str(),
                Color::Gold,
                line4_colors_choice,
                Message::Line4Changed,
            ),
            radio(
                Color::Brown.to_str(),
                Color::Brown,
                line4_colors_choice,
                Message::Line4Changed,
            ),
            radio(
                Color::Red.to_str(),
                Color::Red,
                line4_colors_choice,
                Message::Line4Changed,
            ),
            radio(
                Color::Green.to_str(),
                Color::Green,
                line4_colors_choice,
                Message::Line4Changed,
            ),
            radio(
                Color::Blue.to_str(),
                Color::Blue,
                line4_colors_choice,
                Message::Line4Changed,
            ),
            radio(
                Color::Purple.to_str(),
                Color::Purple,
                line4_colors_choice,
                Message::Line4Changed,
            ),
            radio(
                Color::Gray.to_str(),
                Color::Gray,
                line4_colors_choice,
                Message::Line4Changed,
            ),
            radio(
                "другой",
                Color::Black,
                line4_colors_choice,
                Message::Line4Changed,
            ),
        ]
        .spacing(5);

        let colors_table =
            scrollable(row![line1_colors, line2_colors, line3_colors, line4_colors,].spacing(5))
                .width(Length::Fill);

        let calculate_button = button("Вычислить").on_press(Message::CalculateButtonPressed);
        let donate_button = button("Донат").on_press(Message::DonateButtonPressed);
        let about_button = button("О программе").on_press(Message::AboutButtonPressed);

        let body = container(
            column![
                colors_table,
                vertical_space(),
                row![
                    donate_button,
                    about_button,
                    horizontal_space(),
                    calculate_button,
                ]
                .spacing(10),
            ], // .spacing(20),
        )
        .padding(20);

        let contents = column![header, body].spacing(5);

        container(contents).into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Line1Changed(color) => {
                self.line1 = color;
            }
            Message::Line2Changed(color) => {
                self.line2 = color;
            }
            Message::Line3Changed(color) => {
                self.line3 = color;
            }
            Message::Line4Changed(color) => {
                self.line4 = color;
            }
            Message::CalculateButtonPressed => {
                let color1 = self.line1.to_usize();
                let color2 = self.line2.to_usize();
                let color3 = self.line3.to_multiplier();
                let color4 = self.line4.to_deviation();

                let deviation = match color4 {
                    Some(d) => format!(" ±{d}%"),
                    None => String::new(),
                };

                let color: f32 = format!("{color1}{color2}").trim().parse().unwrap();

                let mut value = color * color3;
                let mut suffix = "Ом";

                if (1000. ..1_000_000.).contains(&value) {
                    value /= 1000.;
                    suffix = "кОм";
                } else if (1_000_000. ..1_000_000_000.).contains(&value) {
                    value /= 1_000_000.;
                    suffix = "МОм";
                } else if value >= 1_000_000_000. {
                    value /= 1_000_000_000.;
                    suffix = "ГОм";
                }

                self.header = format!("{value} {suffix} {deviation}");
                self.subheader = None;
            }
            Message::DonateButtonPressed => {
                self.header = "Мой Boosty: https://boosty.to/linux-for-arm".to_string();
                self.subheader = Some("Карта (Сбербанк): 2202206252335406".to_string());
            }
            Message::AboutButtonPressed => {
                self.header = format!(
                    "{} ver.{}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                );
                self.subheader = Some("Узнать об обновлениях: https://t.me/svalka07".to_string());
            }
        }
    }
}
