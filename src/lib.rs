use ratatui::{
    style::Style,
    text::{Line, Span, Text},
};
use regex::Regex;

pub fn highlight_line<'a>(line: &'a str, pattern: &str, highlight_style: Style) -> Line<'a> {
    let mut highlighted_line = Line::default();

    let reg = Regex::new(pattern).unwrap();
    let mut last_index = 0;

    let find_iter = reg.find_iter(&line);

    if find_iter.last().is_some() {
        for m in reg.find_iter(&line) {
            highlighted_line.push_span(Span::from(&line[last_index..m.start()]));
            highlighted_line.push_span(Span::from(m.as_str()).style(highlight_style));
            last_index = m.end();
        }
    }

    if line.len() > last_index {
        highlighted_line.push_span(Span::from(&line[last_index..]));
    }

    highlighted_line
}

pub fn highlight_text<'a>(line: &'a str, pattern: &str, highlight_style: Style) -> Text<'a> {
    let mut highlighted_text = Text::default();

    let mut last_index = 0;

    for (i, _) in line.match_indices('\n') {
        highlighted_text.push_line(highlight_line(
            &line[last_index..i],
            pattern,
            highlight_style,
        ));
        last_index = i + 1;
    }

    if line.len() > last_index {
        highlighted_text.push_line(highlight_line(
            &line[last_index..],
            pattern,
            highlight_style,
        ));
    }

    highlighted_text
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    const STYLE: Style = Style::new().bg(Color::Blue);
    const TEXT: &str =
        "Hello @Henry. Why are you named @nobody\nBecause yes, and you @John. Btw Where @Bill is ?";

    #[test]
    fn highlighting_line_test() {
        let returned_line = highlight_line(&TEXT[0..39], r"@\w+", STYLE);

        let line = Line::from(vec![
            Span::from("Hello "),
            Span::from("@Henry").style(STYLE),
            Span::from(". Why are you named "),
            Span::from("@nobody").style(STYLE),
        ]);

        assert_eq!(returned_line, line);
    }

    #[test]
    fn highlighting_text_text() {
        let returned_text = highlight_text(TEXT, r"@\w+", STYLE);
        let text = Text::from(vec![
            Line::from(vec![
                Span::from("Hello "),
                Span::from("@Henry").style(STYLE),
                Span::from(". Why are you named "),
                Span::from("@nobody").style(STYLE),
            ]),
            Line::from(vec![
                Span::from("Because yes, and you "),
                Span::from("@John").style(STYLE),
                Span::from(". Btw Where "),
                Span::from("@Bill").style(STYLE),
                Span::from(" is ?"),
            ]),
        ]);

        assert_eq!(returned_text.lines[1], text.lines[1]);

        assert_eq!(returned_text, text);
    }
}
