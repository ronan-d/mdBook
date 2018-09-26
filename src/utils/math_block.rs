use std::borrow::Cow;

use pulldown_cmark::{Event, Tag};

pub fn pipe<'a, I>(events: I) -> impl Iterator<Item = Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let t = handle_block(events);
    handle_inline(t)
}

fn handle_block<'a, I>(mut events: I) -> impl Iterator<Item = Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut new_events = Vec::new();

    const INFO_STRING: &str = "math";
    const OPENING_DELIMITER: &str = r"\[";
    const CLOSING_DELIMITER: &str = r"\]";

    while let Some(event) = events.next() {
        if let Event::Start(Tag::CodeBlock(info_string)) = event {
            if info_string == INFO_STRING {
                let mut new_text = String::from(OPENING_DELIMITER);
                while let Some(Event::Text(text)) = events.next() {
                    new_text.push_str(&text);
                }
                new_text.push_str(CLOSING_DELIMITER);
                let new_text = Cow::from(new_text);

                new_events.push(Event::Start(Tag::Paragraph));
                new_events.push(Event::Text(new_text));
                new_events.push(Event::End(Tag::Paragraph));
            }
        } else {
            new_events.push(event);
        }
    }

    new_events.into_iter()
}

fn handle_inline<'a, I>(mut events: I) -> impl Iterator<Item = Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut new_events = Vec::new();

    const ID: &str = "m";
    const OPENING_DELIMITER: &str = r"\(";
    const CLOSING_DELIMITER: &str = r"\)";

    while let Some(event) = events.next() {
        if let Event::Start(Tag::Code) = event {
            if let Some(event2) = events.next() {
                let new_event = if let Event::Text(text) = event2 {
                    if text.split_whitespace().next() == Some(ID) {
                        let mut new_text = String::from(OPENING_DELIMITER);
                        new_text.push_str(&text[1..].trim());
                        while let Some(Event::Text(text)) = events.next() {
                            new_text.push_str(&text);
                        }
                        new_text.push_str(CLOSING_DELIMITER);
                        let new_event = Event::Text(Cow::from(new_text));
                        new_event
                    } else {
                        new_events.push(event);
                        Event::Text(text)
                    }
                } else {
                    new_events.push(event);
                    event2
                };
                new_events.push(new_event);
            }
        } else {
            new_events.push(event);
        }
    }

    new_events.into_iter()
}
