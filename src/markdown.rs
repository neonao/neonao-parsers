use pulldown_cmark::Parser;
use serde::Serialize;
use std::convert::From;
use std::ops::Range;
use typescript_definitions::TypescriptDefinition;
use wasm_bindgen::prelude::*;

#[derive(Serialize, TypescriptDefinition)]
pub enum LinkType {
    Inline,
    Autolink,
    Email,
    Reference,
    ReferenceUnknown,
    Collapsed,
    CollapsedUnknown,
    Shortcut,
    ShortcutUnknown,
    // https://github.com/arabidopsis/typescript-definitions/issues/2
    // WorkAround(()),
}

impl From<pulldown_cmark::LinkType> for LinkType {
    fn from(link_type: pulldown_cmark::LinkType) -> LinkType {
        use pulldown_cmark::LinkType::*;
        match link_type {
            Inline => LinkType::Inline,
            Autolink => LinkType::Autolink,
            Email => LinkType::Email,
            Reference => LinkType::Reference,
            ReferenceUnknown => LinkType::ReferenceUnknown,
            Collapsed => LinkType::Collapsed,
            CollapsedUnknown => LinkType::CollapsedUnknown,
            Shortcut => LinkType::Shortcut,
            ShortcutUnknown => LinkType::ShortcutUnknown,
        }
    }
}

#[derive(Serialize, TypescriptDefinition)]
pub enum Alignment {
    None,
    Left,
    Center,
    Right,
    // https://github.com/arabidopsis/typescript-definitions/issues/2
    // WorkAround(()),
}

impl From<pulldown_cmark::Alignment> for Alignment {
    fn from(link_type: pulldown_cmark::Alignment) -> Alignment {
        use pulldown_cmark::Alignment::*;
        match link_type {
            None => Alignment::None,
            Left => Alignment::Left,
            Center => Alignment::Center,
            Right => Alignment::Right,
        }
    }
}

#[derive(Serialize, TypescriptDefinition)]
#[serde(tag = "name")]
pub enum Tag {
    Paragraph,
    Emphasis,
    Strong,
    CodeBlock {
        language: String,
    },
    BlockQuote,
    Strikethrough,
    Table {
        alignments: Vec<Alignment>,
    },
    TableHead,
    TableRow,
    TableCell,
    FootnoteDefinition {
        text: String,
    },
    HtmlBlock,
    Rule,
    Item,
    Header {
        level: i32,
    },
    List {
        first: Option<usize>,
    },
    Link {
        kind: LinkType,
        url: String,
        title: String,
    },
    Image {
        kind: LinkType,
        url: String,
        title: String,
    },
}

impl<'a> From<pulldown_cmark::Tag<'a>> for Tag {
    fn from(tag: pulldown_cmark::Tag) -> Tag {
        use pulldown_cmark::Tag::*;
        match tag {
            Paragraph => Tag::Paragraph,
            Emphasis => Tag::Emphasis,
            Strong => Tag::Strong,
            Link(link_type, url, title) => Tag::Link {
                kind: link_type.into(),
                url: url.to_string(),
                title: title.to_string(),
            },
            Image(link_type, url, title) => Tag::Image {
                kind: link_type.into(),
                url: url.to_string(),
                title: title.to_string(),
            },
            Rule => Tag::Rule,
            Header(level) => Tag::Header { level },
            Strikethrough => Tag::Strikethrough,
            List(first) => Tag::List { first },
            Table(alignments) => Tag::Table {
                alignments: alignments.into_iter().map(Into::into).collect(),
            },
            TableCell => Tag::TableCell,
            TableHead => Tag::TableHead,
            TableRow => Tag::TableRow,
            BlockQuote => Tag::BlockQuote,
            FootnoteDefinition(text) => Tag::FootnoteDefinition {
                text: text.to_string(),
            },
            CodeBlock(language) => Tag::CodeBlock {
                language: language.to_string(),
            },
            Item => Tag::Item,
            HtmlBlock => Tag::HtmlBlock,
        }
    }
}

#[derive(Serialize, TypescriptDefinition)]
#[serde(tag = "kind")]
pub enum Event {
    Start { tag: Tag },
    End { tag: Tag },
    Text { text: String },
    SoftBreak,
    Code { code: String },
    HardBreak,
    TaskListMarker { checked: bool },
    Html { html: String },
    InlineHtml { html: String },
    FootnoteReference { content: String },
}

impl<'a> From<pulldown_cmark::Event<'a>> for Event {
    fn from(event: pulldown_cmark::Event) -> Event {
        use pulldown_cmark::Event::*;
        match event {
            Start(tag) => Event::Start { tag: tag.into() },
            End(tag) => Event::End { tag: tag.into() },
            Text(text) => Event::Text {
                text: text.to_string(),
            },
            SoftBreak => Event::SoftBreak,
            HardBreak => Event::HardBreak,
            Code(code) => Event::Code {
                code: code.to_string(),
            },
            TaskListMarker(checked) => Event::TaskListMarker { checked },
            Html(html) => Event::Html {
                html: html.to_string(),
            },
            InlineHtml(html) => Event::InlineHtml {
                html: html.to_string(),
            },
            FootnoteReference(content) => Event::FootnoteReference {
                content: content.to_string(),
            },
        }
    }
}

#[derive(Serialize, TypescriptDefinition)]
pub struct Segment {
    event: Event,
    range: (usize, usize),
}

impl Segment {
    fn new(event: pulldown_cmark::Event, range: Range<usize>) -> Segment {
        let range: (usize, usize) = (range.start, range.end);
        let event = event.into();
        Segment { event, range }
    }
}

#[wasm_bindgen]
pub fn markdown(source: &str) -> JsValue {
    use pulldown_cmark::Options;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    let segments: Vec<Segment> = Parser::new(source)
        .into_offset_iter()
        .map(|(event, range)| Segment::new(event, range))
        .collect();
    JsValue::from_serde(&segments).unwrap()
}
