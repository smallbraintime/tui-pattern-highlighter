use ratatui::{
    style::Style,
    text::{Line, Span, Text},
};
use regex::Regex;

/// Creates a `Line` from the given `line` argument and adds `highlight_style` to `Spans` that match the pattern.
///
/// # Arguments
///
/// * `line` - A string slice that holds the line of text to be highlighted.
/// * `pattern` - A regular expression pattern to match the text that needs to be highlighted.
/// * `highlight_style` - The style to be applied to the matching text.
///
/// # Example
///
/// ```
/// use tui_pattern_highlighter::highlight_line;
/// use ratatui::{
///     style::{Color, Style},
///     text::{Line, Span},
/// };
///
/// let line = "Hi @buddy";
/// let pattern = r"@\w+";
/// let highlight_style = Style::new().bg(Color::Blue);
///
/// let expected_line = Line::from(vec![
///     Span::from("Hi "),
///     Span::from("@buddy").style(Style::new().bg(Color::Blue)),
/// ]);
///
/// assert_eq!(highlight_line(line, pattern, highlight_style), expected_line);
/// ```
///
/// # Panics
///
/// The function may panic if the provided pattern is an invalid regular expression.
pub fn highlight_line<'a>(line: &'a str, pattern: &str, highlight_style: Style) -> Line<'a> {
    let mut highlighted_line = Line::default();

    let reg = Regex::new(pattern).unwrap();
    let mut last_index = 0;

    let find_iter = reg.find_iter(&line);

    if find_iter.last().is_some() {
        for m in reg.find_iter(&line) {
            if m.start() != 0 {
                highlighted_line.push_span(Span::from(&line[last_index..m.start()]));
            }
            highlighted_line.push_span(Span::from(m.as_str()).style(highlight_style));
            last_index = m.end();
        }
    }

    if line.len() > last_index {
        highlighted_line.push_span(Span::from(&line[last_index..]));
    }

    highlighted_line
}

/// Creates `Text` from the given `line` argument and adds `highlight_style` to `Spans` that match the pattern.
/// When the '\n' character is encountered, a new `Line` begins.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to be highlighted.
/// * `pattern` - A regular expression pattern to match the text that needs to be highlighted.
/// * `highlight_style` - The style to be applied to the matching text.
///
/// # Example
///
/// ```
/// use tui_pattern_highlighter::highlight_text;
/// use ratatui::{
///     style::{Color, Style},
///     text::{Line, Span, Text},
/// };
///
/// let text = "Hi @buddy\n@stranger hello";
/// let pattern = r"@\w+";
/// let highlight_style = Style::new().bg(Color::Blue);
///
/// let expected_text = Text::from(vec![
///     Line::from(vec![
///         Span::from("Hi "),
///         Span::from("@buddy").style(Style::new().bg(Color::Blue)),
///     ]),
///     Line::from(vec![
///         Span::from("@stranger").style(Style::new().bg(Color::Blue)),
///         Span::from(" hello"),
///     ]),
/// ]);
///
/// assert_eq!(highlight_text(text, pattern, highlight_style), expected_text);
/// ```
///
/// # Panics
///
/// The function may panic if the provided pattern is an invalid regular expression.
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

        assert_eq!(returned_text, text);
    }
}
