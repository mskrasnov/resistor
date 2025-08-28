use cursive::align;
use cursive::event::Key;
use cursive::menu;
use cursive::theme;
use cursive::Cursive;

use cursive::traits::Nameable;
use cursive::utils::markup::StyledString;

use cursive::views::Button;
use cursive::views::Dialog;
use cursive::views::DummyView;
use cursive::views::LinearLayout;
use cursive::views::ListView;
use cursive::views::Panel;
use cursive::views::RadioGroup;
use cursive::views::TextView;

use crate::colors::Color;

fn about_window(scr: &mut Cursive) {
    let prog_name = StyledString::styled(
        format!("{}-v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        theme::Style::from(theme::Color::Dark(theme::BaseColor::Blue)),
    );

    let summary = Panel::new(
        LinearLayout::vertical()
            .child(TextView::new(prog_name))
            .child(
                ListView::new()
                    .child(
                        "Description:",
                        TextView::new(
                            "A program for calculating the resistance of\n\
                      resistors based on their color marking",
                        ),
                    )
                    .child(
                        "Program perository:",
                        TextView::new("https://github.com/mskrasnov/resistor"),
                    ),
            ),
    )
    .title("Summary");
    let copyright = Panel::new(TextView::new(format!(
        "Copyright (C) 2023-2025 Michail Krasnov <mskrasnov07@ya.ru>"
    )))
    .title("Copyright");

    let donation = Panel::new(TextView::new(StyledString::styled(
        "If you are from Russia, you can send me a donation:\n\
        2202 2062 5233 5406 (Сбербанк)",
        theme::Style::from(theme::Color::Dark(theme::BaseColor::Red)).combine(theme::Effect::Bold),
    )))
    .title("Support me!");

    let about = LinearLayout::vertical()
        .child(summary)
        .child(copyright)
        .child(donation);

    let win = Dialog::around(about)
        .title("About program")
        .button("OK", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}

fn units_window(scr: &mut Cursive) {
    let layout = LinearLayout::vertical()
        .child(TextView::new(format!(
            "{} supports followed units:",
            env!("CARGO_PKG_NAME")
        )))
        .child(Panel::new(TextView::new(
            "Ohm (Ω) — calculating unit\n\
	                  kΩ, Kilo-Ohm — 10^3 Ω\n\
	                  MΩ, Mega-Ohm — 10^6 Ω\n\
	                  GΩ, Giga-Ohm — 10^9 Ω\n\n\
	                  Conversion to the specified\n\
	                  units is automatic.",
        )));
    let win = Dialog::around(layout)
        .title("Supported units")
        .button("OK", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}

fn feedback_window(scr: &mut Cursive) {
    let layout = LinearLayout::vertical()
        .child(TextView::new(
            "If you see an error or want to offer new functionality,\n\
                      visit the address below, fill in all the specified\n\
                      information and send an issue.",
        ))
        .child(Panel::new(TextView::new(
            "https://github.com/mskrasnov/resistor/issues/new",
        )))
        .child(
            TextView::new(StyledString::styled(
                "GitHub registration needed!",
                theme::Style::from(theme::Color::Dark(theme::BaseColor::Red))
                    .combine(theme::Effect::Blink)
                    .combine(theme::Effect::Bold),
            ))
            .align(align::Align::center()),
        );
    let win = Dialog::around(layout).title("Feedback").button("OK", |s| {
        s.pop_layer();
    });
    scr.add_layer(win);
}

pub fn five_colors_resistor_window(scr: &mut Cursive) {
    let mut colors1 = RadioGroup::new();
    let mut colors2 = RadioGroup::new();
    let mut colors3 = RadioGroup::new();
    let mut colors4 = RadioGroup::new();

    let work_layout = LinearLayout::horizontal()
        .child(
            Panel::new(
                LinearLayout::vertical()
                    .child(colors1.button(Color::Black, Color::Black.to_str()))
                    .child(colors1.button(Color::Brown, Color::Brown.to_str()))
                    .child(colors1.button(Color::Red, Color::Red.to_str()))
                    .child(colors1.button(Color::Orange, Color::Orange.to_str()))
                    .child(colors1.button(Color::Yellow, Color::Yellow.to_str()))
                    .child(colors1.button(Color::Green, Color::Green.to_str()))
                    .child(colors1.button(Color::Blue, Color::Blue.to_str()))
                    .child(colors1.button(Color::Purple, Color::Purple.to_str()))
                    .child(colors1.button(Color::Gray, Color::Gray.to_str()))
                    .child(colors1.button(Color::White, Color::White.to_str())),
            )
            .title("Line №1")
            .title_position(align::HAlign::Left),
        )
        .child(
            Panel::new(
                LinearLayout::vertical()
                    .child(colors2.button(Color::Black, Color::Black.to_str()))
                    .child(colors2.button(Color::Brown, Color::Brown.to_str()))
                    .child(colors2.button(Color::Red, Color::Red.to_str()))
                    .child(colors2.button(Color::Orange, Color::Orange.to_str()))
                    .child(colors2.button(Color::Yellow, Color::Yellow.to_str()))
                    .child(colors2.button(Color::Green, Color::Green.to_str()))
                    .child(colors2.button(Color::Blue, Color::Blue.to_str()))
                    .child(colors2.button(Color::Purple, Color::Purple.to_str()))
                    .child(colors2.button(Color::Gray, Color::Gray.to_str()))
                    .child(colors2.button(Color::White, Color::White.to_str()))
                    .child(colors2.button(Color::Gold, Color::Gold.to_str())),
            )
            .title("Line №2")
            .title_position(align::HAlign::Left),
        )
        .child(
            Panel::new(
                LinearLayout::vertical()
                    .child(colors3.button(Color::Black, Color::Black.to_str()))
                    .child(colors3.button(Color::Brown, Color::Brown.to_str()))
                    .child(colors3.button(Color::Red, Color::Red.to_str()))
                    .child(colors3.button(Color::Orange, Color::Orange.to_str()))
                    .child(colors3.button(Color::Yellow, Color::Yellow.to_str()))
                    .child(colors3.button(Color::Green, Color::Green.to_str()))
                    .child(colors3.button(Color::Blue, Color::Blue.to_str()))
                    .child(colors3.button(Color::Purple, Color::Purple.to_str()))
                    .child(colors3.button(Color::Gray, Color::Gray.to_str()))
                    .child(colors3.button(Color::White, Color::White.to_str()))
                    .child(colors3.button(Color::Gold, Color::Gold.to_str()))
                    .child(colors3.button(Color::Silver, Color::Silver.to_str())),
            )
            .title("Line №3")
            .title_position(align::HAlign::Left),
        )
        .child(
            Panel::new(
                LinearLayout::vertical()
                    .child(colors4.button(Color::Silver, Color::Silver.to_str()))
                    .child(colors4.button(Color::Gold, Color::Gold.to_str()))
                    .child(colors4.button(Color::Brown, Color::Brown.to_str()))
                    .child(colors4.button(Color::Red, Color::Red.to_str()))
                    .child(colors4.button(Color::Green, Color::Green.to_str()))
                    .child(colors4.button(Color::Blue, Color::Blue.to_str()))
                    .child(colors4.button(Color::Purple, Color::Purple.to_str()))
                    .child(colors4.button(Color::Gray, Color::Gray.to_str()))
                    .child(colors4.button(Color::Black, "other")),
            )
            .title("Line №4")
            .title_position(align::HAlign::Left),
        );

    let status_bar = LinearLayout::horizontal()
        .child(Panel::new(
            LinearLayout::horizontal()
                .child(Button::new("Calculate", move |s| {
                    let color1 = colors1.selection().to_usize();
                    let color2 = colors2.selection().to_usize();
                    let color3 = colors3.selection().to_multiplier();
                    let color4 = colors4.selection().to_deviation();

                    let deviation = match color4 {
                        Some(d) => format!(" ±{d}%"),
                        None => String::new(),
                    };

                    let color: f32 = format!("{color1}{color2}").trim().parse().unwrap();

                    let mut value = color * color3;
                    let mut suffix = "Ω";

                    if (1000. ..1_000_000.).contains(&value) {
                        value /= 1000.;
                        suffix = "KΩ";
                    } else if (1_000_000. ..1_000_000_000.).contains(&value) {
                        value /= 1_000_000.;
                        suffix = "MΩ";
                    } else if value >= 1_000_000_000. {
                        value /= 1_000_000_000.;
                        suffix = "GΩ";
                    }

                    s.call_on_name("result", |txt: &mut TextView| {
                        txt.set_content(format!("Resistance: {} {}{}", value, suffix, deviation,));
                    });
                }))
                .child(DummyView)
                .child(Button::new("Quit", |s| s.quit())),
        ))
        .child(Panel::new(
            TextView::new("the calculation results will appear here").with_name("result"),
        ));

    let win = Dialog::around(
        LinearLayout::vertical()
            .child(work_layout)
            .child(status_bar),
    )
    .title(env!("CARGO_PKG_NAME"));

    scr.add_layer(win);
}

pub fn main_window(scr: &mut Cursive) {
    scr.menubar()
        .add_subtree("File", menu::Tree::new().leaf("Quit", |s| s.quit()))
        .add_subtree(
            "Help",
            menu::Tree::new()
                .leaf("Supported units", units_window)
                .leaf("Feedback", feedback_window)
                .leaf("About", about_window),
        );

    scr.add_global_callback(Key::F1, |s| s.select_menubar());
    scr.add_global_callback(Key::Esc, |s| s.quit());
    scr.set_autohide_menu(false);

    five_colors_resistor_window(scr);
}
