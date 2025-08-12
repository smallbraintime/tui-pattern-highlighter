use ratatui::{
    style::Style,
    text::{Line, Span, Text},
};
use regex::Regex;
use std::borrow::Cow;

/// Creates a `Line` from the given `line` argument and adds `highlight_style` to `Spans` that match the pattern.
///
/// # Arguments
///
/// * `line` - A string that holds the line of text to be highlighted.
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
/// let line = String::from("Hi @buddy");
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
pub fn highlight_line<'a, S, T>(line: S, pattern: T, highlight_style: Style) -> Line<'a>
where
    S: Into<String>,
    T: IntoRegexRef,
{
    let line_string = line.into();
    let mut highlighted_line = Line::default();

    let regex_ref = pattern.into_regex_ref();
    let mut last_index = 0;

    for m in regex_ref.find_iter(&line_string).collect::<Vec<_>>() {
        if m.start() > last_index {
            highlighted_line.push_span(Span::from(line_string[last_index..m.start()].to_string()));
        }
        highlighted_line.push_span(Span::from(m.as_str().to_string()).style(highlight_style));
        last_index = m.end();
    }

    if line_string.len() > last_index {
        highlighted_line.push_span(Span::from(line_string[last_index..].to_string()));
    }

    highlighted_line
}

/// Creates `Text` from the given `line` argument and adds `highlight_style` to `Spans` that match the pattern.
/// When the '\n' character is encountered, a new `Line` begins.
///
/// # Arguments
///
/// * `text` - A string that holds the text to be highlighted.
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
/// let text = String::from("Hi @buddy\n@stranger hello");
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
///     ]);
///
/// assert_eq!(highlight_text(text, pattern, highlight_style), expected_text);
/// ```
///
/// # Panics
///
/// The function may panic if the provided pattern is an invalid regular expression.
pub fn highlight_text<'a, S, T>(text: S, pattern: T, highlight_style: Style) -> Text<'a>
where
    S: Into<String>,
    T: IntoRegexRef + Clone,
{
    let text_string = text.into();
    let mut highlighted_text = Text::default();

    let mut last_index = 0;

    for (i, _) in text_string.match_indices('\n') {
        highlighted_text.push_line(highlight_line(
            text_string[last_index..i].to_string(),
            pattern.clone(),
            highlight_style,
        ));
        last_index = i + 1;
    }

    if text_string.len() > last_index {
        highlighted_text.push_line(highlight_line(
            text_string[last_index..].to_string(),
            pattern,
            highlight_style,
        ));
    }

    highlighted_text
}

pub trait IntoRegexRef {
    fn into_regex_ref(self) -> Cow<'static, Regex>;
}

impl IntoRegexRef for Regex {
    fn into_regex_ref(self) -> Cow<'static, Regex> {
        Cow::Owned(self)
    }
}

impl<'a> IntoRegexRef for &'a Regex {
    fn into_regex_ref(self) -> Cow<'static, Regex> {
        Cow::Owned(self.clone())
    }
}

impl IntoRegexRef for String {
    fn into_regex_ref(self) -> Cow<'static, Regex> {
        Cow::Owned(Regex::new(&self).unwrap())
    }
}

impl<'a> IntoRegexRef for &'a String {
    fn into_regex_ref(self) -> Cow<'static, Regex> {
        Cow::Owned(Regex::new(self).unwrap())
    }
}

impl<'a> IntoRegexRef for &'a str {
    fn into_regex_ref(self) -> Cow<'static, Regex> {
        Cow::Owned(Regex::new(self).unwrap())
    }
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

        let regex = Regex::new(r"@\w+").unwrap();
        let returned_line_reg = highlight_line(&TEXT[0..39], &regex, STYLE);

        let line = Line::from(vec![
            Span::from("Hello "),
            Span::from("@Henry").style(STYLE),
            Span::from(". Why are you named "),
            Span::from("@nobody").style(STYLE),
        ]);

        assert_eq!(returned_line, line);
        assert_eq!(returned_line_reg, line);
    }

    #[test]
    fn highlighting_text_test() {
        let returned_text = highlight_text(TEXT, r"@\w+", STYLE);

        let regex = Regex::new(r"@\w+").unwrap();
        let returned_text_reg = highlight_text(TEXT, &regex, STYLE);

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
        assert_eq!(returned_text_reg, text);
    }
}
