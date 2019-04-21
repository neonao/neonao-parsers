use pulldown_cmark::Parser;
use serde::Serialize;
use std::convert::From;
use std::ops::Range;
use wasm_bindgen::prelude::*;

// https://docs.rs/pulldown-cmark/0.4.1/pulldown_cmark/enum.LinkType.html
#[derive(Serialize)]
pub enum LinkType {
    Inline,
    Autolink,
    Email,
    Unsupported,
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
#[derive(Serialize)]
pub enum Tag {
    Paragraph,
    Emphasis,
    Strong,
    Code,
    Unsupported,
    Link(LinkType, String, String),
    // Image(LinkType, CowStr<'a>, CowStr<'a>),
}

impl<'a> From<pulldown_cmark::Tag<'a>> for Tag {
    fn from(tag: pulldown_cmark::Tag) -> Tag {
        match tag {
            pulldown_cmark::Tag::Paragraph => Tag::Paragraph,
            pulldown_cmark::Tag::Emphasis => Tag::Emphasis,
            pulldown_cmark::Tag::Strong => Tag::Strong,
            pulldown_cmark::Tag::Code => Tag::Code,
            pulldown_cmark::Tag::Link(link_type, url, title) => {
                Tag::Link(link_type.into(), url.to_string(), title.to_string())
            }
            _ => Tag::Unsupported,
        }
    }
}

#[derive(Serialize)]
pub enum Event {
    Start(Tag),
    End(Tag),
    Text(String),
    // Html(String),
    // InlineHtml(String),
    // FootnoteReference(String),
    // SoftBreak,
    // HardBreak,
    // TaskListMarker(bool),
}

#[derive(Serialize)]
pub struct Segment {
    event: Event,
    range: Range<usize>,
}

#[wasm_bindgen]
pub fn markdown(source: &str) -> JsValue {
    use pulldown_cmark::Event::*;
    let mut segments: Vec<Segment> = vec![];
    for (event, range) in Parser::new(source).into_offset_iter() {
        match event {
            Start(tag) => {
                let event = Event::Start(tag.into());
                segments.push(Segment { event, range })
            }
            End(tag) => {
                let event = Event::End(tag.into());
                segments.push(Segment { event, range })
            }
            Text(text) => {
                let event = Event::Text(text.to_string());
                segments.push(Segment { event, range })
            }
            _ => (),
        }
    }
    JsValue::from_serde(&segments).unwrap()
}
