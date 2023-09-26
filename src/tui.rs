use cursive::Cursive;
use cursive::align;
use cursive::event::Key;
use cursive::menu;
use cursive::theme;

use cursive::traits::Nameable;
use cursive::utils::markup::StyledString;

use cursive::views::Dialog;
use cursive::views::DummyView;
use cursive::views::TextView;
use cursive::views::Panel;
use cursive::views::RadioGroup;
use cursive::views::LinearLayout;
use cursive::views::Button;

use libmsg::tui::about::AboutWindowBuilder;
use libmsg::tui::about::Copyright;
use libmsg::tui::about::Donation;

use crate::colors::Color;

fn about_window(scr: &mut Cursive) {
	let about = AboutWindowBuilder::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
		.set_description("Программа для вычисления сопротивления резисторов по\nих цветовой маркировке")
		.set_site("https://github.com/aagrh111/resistor")
		.add_copyright(Copyright::new("2023", "{Tsar}", "michail383krasnov@mail.ru"))
		.add_donation(Donation::new("{Tsar}", "Сбербанк", "2202 2062 5233 5406\nГде мои деньги, Лебовски?"));

	let win = Dialog::around(about.build().window())
		.title("О программе")
		.button("ОК", |s| { s.pop_layer(); });
	scr.add_layer(win);
}

fn donut_window(scr: &mut Cursive) {
	let layout = LinearLayout::vertical()
		.child(TextView::new(
			"Над программой работает голодный человек в одиночку. Если\n\
			          вы хотите отблагодарить его за проделанную работу, то,\n\
			          пожалуйста, отправьте ему на карту донат. Он будет очень рад\n\
			          и обязательно продолжит работу над Resistor :)"
		))
		.child(Panel::new(
			TextView::new("2202 2062 5233 5406").align(align::Align::center()))
		.title("Сбербанк"))
		.child(TextView::new(
			"Resistor — свободное ПО, за которое не платят его\n\
			          разработчику. Обратная связь от пользователей и добровольные\n\
			          пожертвования помогают сохранить мотивацию работать над\n\
			          программой дальше. Также деньги пойдут на покупку нового\n\
			          оборудования (мне нужен новый ноутбук) и на обслуживание уже\n\
			          имеющегося."
		));

	let win = Dialog::around(layout)
		.title("ДОНАТ")
		.button("ОК", |s| { s.pop_layer(); });
	scr.add_layer(win);
}

fn units_window(scr: &mut Cursive) {
	let layout = LinearLayout::vertical()
		.child(TextView::new(format!("{} поддерживает следующие\nединицы измерения:", env!("CARGO_PKG_NAME"))))
		.child(Panel::new(TextView::new(
			"Ом (Ω) — единица измерения\n\
	                  кОм (kΩ, килоом) — 10^3 Ом\n\
	                  МОм (MΩ, мегаом) — 10^6 Ом\n\
	                  ГОм (GΩ, гигаом) — 10^9 Ом\n\n\
			          Приведение к указанным единицам\n\
			          происходит автоматически.",
		)));
	let win = Dialog::around(layout)
		.title("Поддерживаемые единицы")
		.button("OK", |s| { s.pop_layer(); });
	scr.add_layer(win);
}

fn feedback_window(scr: &mut Cursive) {
	let layout = LinearLayout::vertical()
		.child(TextView::new(
			"Если вы увидели ошибку или хотите предложить новый\n\
			          функционал для программы, посетите адрес ниже,\n\
			          заполните все указанные данные и отправьте issue."
		))
		.child(Panel::new(TextView::new(
			"https://github.com/aagrh111/resistor/issues/new"
		)))
		.child(TextView::new(
			StyledString::styled(
				"Необходима регистрация на GitHub!",
				theme::Style::from(
					theme::Color::Dark(theme::BaseColor::Red)
				).combine(theme::Effect::Blink).combine(theme::Effect::Bold)
			)
		).align(align::Align::center()));
	let win = Dialog::around(layout)
		.title("Обратная связь")
		.button("OK", |s| { s.pop_layer(); });
	scr.add_layer(win);
}

pub fn five_colors_resistor_window(scr: &mut Cursive) {
	let mut colors1 = RadioGroup::new();
	let mut colors2 = RadioGroup::new();
	let mut colors3 = RadioGroup::new();
	let mut colors4 = RadioGroup::new();

	let work_layout = LinearLayout::horizontal()
		.child(Panel::new(
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
				.child(colors1.button(Color::White, Color::White.to_str()))
		).title("Полоса №1").title_position(align::HAlign::Left))
		.child(Panel::new(
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
				.child(colors2.button(Color::Gold, Color::Gold.to_str()))
		).title("Полоса №2").title_position(align::HAlign::Left))
		.child(Panel::new(
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
				.child(colors3.button(Color::Silver, Color::Silver.to_str()))
		).title("Полоса №3").title_position(align::HAlign::Left))
		.child(Panel::new(
			LinearLayout::vertical()
				.child(colors4.button(Color::Silver, Color::Silver.to_str()))
				.child(colors4.button(Color::Gold, Color::Gold.to_str()))
				.child(colors4.button(Color::Brown, Color::Brown.to_str()))
				.child(colors4.button(Color::Red, Color::Red.to_str()))
				.child(colors4.button(Color::Green, Color::Green.to_str()))
				.child(colors4.button(Color::Blue, Color::Blue.to_str()))
				.child(colors4.button(Color::Purple, Color::Purple.to_str()))
				.child(colors4.button(Color::Gray, Color::Gray.to_str()))
				.child(colors4.button(Color::Black, "другой"))
		).title("Полоса №4").title_position(align::HAlign::Left));

	let status_bar = LinearLayout::horizontal()
		.child(Panel::new(LinearLayout::horizontal()
			.child(Button::new("Вычислить", move |s| {
				let color1 = colors1
					.selection()
					.to_usize();
				let color2 = colors2
					.selection()
					.to_usize();
				let color3 = colors3
					.selection()
					.to_multiplier();
				let color4 = colors4
					.selection()
					.to_deviation();

				let deviation = match color4 {
					Some(d) => format!(" ±{d}%"),
					None => String::new(),
				};

				let color: f32 = format!("{color1}{color2}")
					.trim()
					.parse()
					.unwrap();

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

				s.call_on_name("result", |txt: &mut TextView| {
					txt.set_content(format!(
						"Сопротивление: {} {}{}", value, suffix, deviation,
					));
				});
			}))
			.child(DummyView)
			.child(Button::new("Выйти", |s| s.quit()))
		))
		.child(Panel::new(
			TextView::new("здесь появятся результаты вычислений")
				.with_name("result")
		));

	let win = Dialog::around(LinearLayout::vertical()
		.child(work_layout)
		.child(status_bar)
	).title(env!("CARGO_PKG_NAME"));

	scr.add_layer(win);
}

pub fn main_window(scr: &mut Cursive) {
	scr.menubar()
		.add_subtree("Файл", menu::Tree::new()
			.leaf("Выход", |s| s.quit())
		)
		.add_subtree("Справка", menu::Tree::new()
			.leaf("ДОНАТ", donut_window)
			.leaf("Поддерживаемые единицы", units_window)
			.leaf("Обратная связь", feedback_window)
			.leaf("О программе", about_window)
		);

	scr.add_global_callback(Key::F1, |s| s.select_menubar());
	scr.add_global_callback(Key::F10, donut_window);
	scr.add_global_callback(Key::Esc, |s| s.quit());
	scr.set_autohide_menu(false);

	five_colors_resistor_window(scr);
}
