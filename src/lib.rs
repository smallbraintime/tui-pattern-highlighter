use ratatui::{
    style::Style,
    text::{Line, Span},
};
use regex::Regex;

pub fn highlight_line<'a>(line: &'a str, pattern: &str, highlight_style: Style) -> Line<'a> {
    let reg = Regex::new(pattern).unwrap();

    let mut highlighted_line = Line::default();

    let mut last_index = 0;

    for m in reg.find_iter(&line) {
        highlighted_line.push_span(Span::from(&line[last_index..m.start()]));
        highlighted_line.push_span(Span::from(m.as_str()).style(highlight_style));
        last_index = m.end();
    }

    if line.len() > last_index {
        highlighted_line.push_span(Span::from(&line[last_index..]).style(highlight_style));
    }

    highlighted_line
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    #[test]
    fn highlighting_text() {
        let returned_line = highlight_line(
            "Welcome @Henry. Why are you named @nobody",
            r"@\w+",
            Style::new().bg(Color::Blue),
        );

        let line = Line::from(vec![
            Span::from("Welcome "),
            Span::from("@Henry").style(Style::new().bg(Color::Blue)),
            Span::from(". Why are you named "),
            Span::from("@nobody").style(Style::new().bg(Color::Blue)),
        ]);

        assert_eq!(returned_line, line);
    }
}
