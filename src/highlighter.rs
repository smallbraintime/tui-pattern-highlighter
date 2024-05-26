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
        highlighted_line.push_span(Span::from(&line[last_index..m.start()]).style(highlight_style));
        highlighted_line.push_span(Span::from(m.as_str()).style(highlight_style));
        last_index = m.end() + 1;
    }

    if line.len() > last_index {
        highlighted_line.push_span(Span::from(&line[last_index..]).style(highlight_style));
    }

    highlighted_line
}
