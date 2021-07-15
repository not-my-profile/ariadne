use ariadne::{Report, ReportKind, Label, Source, Config, Color};

fn main() {
    Report::build(ReportKind::Error, "stresstest.tao", 13)
        .with_code(3)
        .with_message(format!("Incompatible types"))
        .with_label(Label::new(("stresstest.tao", 18..19)).with_message("This is of type Nat").with_color(Color::Fixed(166)))
        .with_label(Label::new(("stresstest.tao", 13..16)).with_message("This is of type Str").with_color(Color::Fixed(23)))
        .with_label(Label::new(("stresstest.tao", 40..41)).with_message("This is of type Nat").with_color(Color::Fixed(48)))
        .with_label(Label::new(("stresstest.tao", 43..47)).with_message("This is of type Bool").with_color(Color::Fixed(86)))
        .with_label(Label::new(("stresstest.tao", 49..51)).with_message("This is of type ()").with_color(Color::Fixed(99)))
        .with_label(Label::new(("stresstest.tao", 53..55)).with_message("This is of type [_]").with_color(Color::Fixed(130)))
        .with_label(Label::new(("stresstest.tao", 75..78)).with_message("This is of type Str").with_color(Color::Fixed(145)))
        .with_label(Label::new(("stresstest.tao", 81..134)).with_message("This is of type Nat").with_color(Color::Fixed(178)))
        .with_label(Label::new(("stresstest.tao", 100..126)).with_message("This is an inner multi-line").with_color(Color::Fixed(185)))
        .with_label(Label::new(("stresstest.tao", 106..120)).with_message("This is another inner multi-line").with_color(Color::Fixed(197)))
        .with_label(Label::new(("stresstest.tao", 108..122)).with_message("This is *really* nested multi-line").with_color(Color::Fixed(210)))
        .with_label(Label::new(("stresstest.tao", 110..111)).with_message("This is an inline within the nesting!").with_color(Color::Fixed(43)))
        .with_label(Label::new(("stresstest.tao", 111..112)).with_message("And another!").with_color(Color::Fixed(71)))
        .with_label(Label::new(("stresstest.tao", 103..123)).with_message("This is *really* nested multi-line").with_color(Color::Fixed(101)))
        .with_label(Label::new(("stresstest.tao", 105..125)).with_message("This is *really* nested multi-line").with_color(Color::Fixed(140)))
        .with_label(Label::new(("stresstest.tao", 112..116)).with_message("This is *really* nested multi-line").with_color(Color::Fixed(120)))
        .with_label(Label::new(("stresstest.tao", 83..117)).with_message("Hahaha!").with_color(Color::Fixed(75)))
        .with_label(Label::new(("stresstest.tao", 85..110)).with_message("Oh god, no more 1").with_color(Color::Fixed(33)))
        .with_label(Label::new(("stresstest.tao", 84..114)).with_message("Oh god, no more 2").with_color(Color::Fixed(52)))
        .with_label(Label::new(("stresstest.tao", 89..113)).with_message("Oh god, no more 3").with_color(Color::Fixed(167)))
        .with_config(Config::default()
            .with_cross_gap(false)
            .with_compact(true)
            .with_underlines(true)
            .with_tab_width(4))
        .finish()
        .print(("stresstest.tao", Source::from(include_str!("stresstest.tao"))))
        .unwrap();
}