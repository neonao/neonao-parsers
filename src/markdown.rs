use pulldown_cmark::Parser;
use serde::Serialize;
use std::convert::From;
use std::ops::Range;
use typescript_definitions::TypescriptDefinition;
use wasm_bindgen::prelude::*;

// https://docs.rs/pulldown-cmark/0.4.1/pulldown_cmark/enum.LinkType.html
#[derive(Serialize, TypescriptDefinition)]
pub enum LinkType {
    Inline,
    Autolink,
    Email,
    Unsupported,
    // https://github.com/arabidopsis/typescript-definitions/issues/2
    // WorkAround(()),
}

impl From<pulldown_cmark::LinkType> for LinkType {
    fn from(link_type: pulldown_cmark::LinkType) -> LinkType {
        match link_type {
            pulldown_cmark::LinkType::Inline => LinkType::Inline,
            pulldown_cmark::LinkType::Autolink => LinkType::Autolink,
            pulldown_cmark::LinkType::Email => LinkType::Email,
            _ => LinkType::Unsupported,
        }
    }
}

// https://docs.rs/pulldown-cmark/0.4.1/pulldown_cmark/enum.Tag.html
#[derive(Serialize, TypescriptDefinition)]
pub enum Tag {
    Paragraph,
    Emphasis,
    Strong,
    Code,
    Unsupported,
    Link {
        kind: LinkType,
        url: String,
        title: String,
    },
    // Image(LinkType, CowStr<'a>, CowStr<'a>),
}

impl<'a> From<pulldown_cmark::Tag<'a>> for Tag {
    fn from(tag: pulldown_cmark::Tag) -> Tag {
        match tag {
            pulldown_cmark::Tag::Paragraph => Tag::Paragraph,
            pulldown_cmark::Tag::Emphasis => Tag::Emphasis,
            pulldown_cmark::Tag::Strong => Tag::Strong,
            pulldown_cmark::Tag::Code => Tag::Code,
            pulldown_cmark::Tag::Link(link_type, url, title) => Tag::Link {
                kind: link_type.into(),
                url: url.to_string(),
                title: title.to_string(),
            },
            _ => Tag::Unsupported,
        }
    }
}

#[derive(Serialize, TypescriptDefinition)]
pub enum Event {
    Start { tag: Tag },
    End { tag: Tag },
    Text { text: String },
    Unsupported,
}

#[derive(Serialize, TypescriptDefinition)]
pub struct Segment {
    event: Event,
    range: (usize, usize),
}

impl Segment {
    fn new(event: pulldown_cmark::Event, range: Range<usize>) -> Segment {
        use pulldown_cmark::Event::*;
        let range: (usize, usize) = (range.start, range.end);

        match event {
            Start(tag) => {
                let event = Event::Start { tag: tag.into() };
                Segment { event, range }
            }
            End(tag) => {
                let event = Event::End { tag: tag.into() };
                Segment { event, range }
            }
            Text(text) => {
                let event = Event::Text {
                    text: text.to_string(),
                };
                Segment { event, range }
            }
            _ => Segment {
                range,
                event: Event::Unsupported,
            },
        }
    }
}

#[wasm_bindgen]
pub fn markdown(source: &str) -> JsValue {
    let segments: Vec<Segment> = Parser::new(source)
        .into_offset_iter()
        .map(|(event, range)| Segment::new(event, range))
        .collect();
    JsValue::from_serde(&segments).unwrap()
}
