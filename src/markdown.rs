use markup5ever::Attribute;
use markup5ever_rcdom::{Handle, NodeData};
use tendril::StrTendril;

use std::collections::LinkedList;

use crate::parse::{parse_stdin, parse_string};

pub trait HtmlConverter {
    fn convert_html(&mut self, handle: Handle) -> String;
}

pub fn convert_stdin() -> String {
    let dom = parse_stdin();
    convert_html(dom.document)
}

pub fn convert_string(s: &str) -> String {
    let dom = parse_string(s);
    convert_html(dom.document)
}

pub fn convert_html(handle: Handle) -> String {
    let mut converter = MarkdownConverter::new();
    converter.convert_html(handle)
}

pub struct MarkdownConverter<'a> {
    buf: String,
    prefix: LinkedList<&'a str>,
    list_markers: Vec<Option<usize>>,
}

impl<'a> Default for MarkdownConverter<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> MarkdownConverter<'a> {
    pub fn new() -> MarkdownConverter<'a> {
        MarkdownConverter {
            buf: String::new(),
            prefix: LinkedList::new(),
            list_markers: Vec::new(),
        }
    }

    fn convert_html_into_buffer(&mut self, handle: &Handle) {
        match &handle.data {
            NodeData::Comment { .. } => {}
            NodeData::Doctype { .. } => {}
            NodeData::Text { contents } => {
                let text = contents.borrow();
                convert_text(&text, &mut self.buf, &mut self.prefix);
            }
            NodeData::Element { name, attrs, .. } => {
                let attrs_borrowed = attrs.borrow();
                self.handle_element(name, &attrs_borrowed, handle);
            }
            NodeData::Document => {
                let children = handle.children.borrow();
                for child in children.iter() {
                    self.convert_html_into_buffer(child);
                }
            }
            NodeData::ProcessingInstruction { .. } => {}
        }
    }

    fn handle_element(
        &mut self,
        name: &markup5ever::QualName,
        attrs: &[Attribute],
        handle: &Handle,
    ) {
        let tag: &str = &name.local.to_ascii_lowercase();

        match tag {
            "head" | "style" | "script" => {
                // ignore these
            }
            _ => {
                // start element
                self.element_start(tag, attrs);
                // do contents
                let children = handle.children.borrow();
                for child in children.iter() {
                    self.convert_html_into_buffer(child);
                }
                // end element
                self.element_end(tag, attrs);
            }
        }
    }

    fn element_start(&mut self, name: &str, attrs: &[Attribute]) {
        match name {
            "b" | "strong" => bold_start(&mut self.buf),
            "i" | "em" => emphasize_start(&mut self.buf),
            "p" | "div" => self.p_start(),
            "blockquote" => blockquote_start(&mut self.buf, &mut self.prefix),
            "br" => self.br_start(),
            "a" => link_start(&mut self.buf),
            "img" => img_start(&mut self.buf, attrs),
            "ul" => ul_start(&mut self.buf, &mut self.list_markers),
            "ol" => ol_start(&mut self.buf, &mut self.list_markers),
            "li" => li_start(&mut self.buf, &self.list_markers),
            _ => {}
        }
    }

    fn element_end(&mut self, name: &str, attrs: &[Attribute]) {
        match name {
            "b" | "strong" => bold_end(&mut self.buf),
            "i" | "em" => emphasize_end(&mut self.buf),
            "blockquote" => blockquote_end(&mut self.buf, &mut self.prefix),
            "a" => link_end(&mut self.buf, attrs),
            "ul" | "ol" => list_end(&mut self.buf, &mut self.list_markers),
            "li" => {
                li_end(&mut self.buf, &mut self.list_markers);
            }
            _ => {}
        }
    }

    fn p_start(&mut self) {
        if let Some(prefix) = prefix(&self.list_markers) {
            if self.buf.ends_with(&prefix) {
                return;
            }
        }
        ensure_double_newline(&mut self.buf);
        prefix_with_necessary_spaces(&mut self.buf, &self.list_markers);
    }

    fn br_start(&mut self) {
        if let Some(prefix) = prefix(&self.list_markers) {
            if self.buf.ends_with(&prefix) {
                return;
            }
        }
        ensure_newline(&mut self.buf);
        prefix_with_necessary_spaces(&mut self.buf, &self.list_markers);
    }
}

impl<'a> HtmlConverter for MarkdownConverter<'a> {
    fn convert_html(&mut self, handle: Handle) -> String {
        self.convert_html_into_buffer(&handle);
        self.buf.clone()
    }
}

fn convert_text(text: &StrTendril, buf: &mut String, prefix: &mut LinkedList<&str>) {
    // Start with prefixes
    for p in prefix.iter() {
        buf.push_str(p);
    }

    // Separate prefix(es) from actual text with one space
    if !prefix.is_empty() {
        buf.push(' ');
    }

    // True if previous is whitespace
    let mut prev = buf.is_empty() || buf.ends_with(' ') || buf.ends_with('\n');
    for c in text.chars() {
        match c {
            // Stick to a single space
            ' ' | '\n' => {
                if !prev {
                    prev = true;
                    buf.push(' ');
                }
            }
            _ => {
                prev = false;
                buf.push(c);
            }
        }
    }
}

fn bold_start(buf: &mut String) {
    buf.push_str("**")
}

fn bold_end(buf: &mut String) {
    bold_start(buf)
}

fn emphasize_start(buf: &mut String) {
    buf.push('*')
}

fn emphasize_end(buf: &mut String) {
    emphasize_start(buf)
}

fn trim_ending_whitespace(buf: &mut String) {
    while buf.ends_with(' ') || buf.ends_with('\t') {
        let end = buf.len() - 1;
        buf.remove(end);
    }
}

fn prefix(list_markers: &[Option<usize>]) -> Option<String> {
    if let Some(mark) = list_markers.last() {
        match *mark {
            Some(i) => Some(format!("{}. ", i)),
            None => Some("* ".to_string()),
        }
    } else {
        None
    }
}

fn prefix_with_necessary_spaces(buf: &mut String, list_markers: &[Option<usize>]) {
    let count = list_markers.iter().fold(0, |sum, mark| match *mark {
        Some(_) => sum + 3, // '1. ' = three characters
        None => sum + 2,    // '* ' = two characters
    });

    buf.push_str(&" ".repeat(count));
}

fn ensure_double_newline(buf: &mut String) {
    trim_ending_whitespace(buf);
    if buf.ends_with("\n\n") {
        // Nothing to do
    } else if buf.ends_with('\n') {
        buf.push('\n')
    } else if !buf.is_empty() {
        buf.push_str("\n\n")
    }
}

fn ensure_newline(buf: &mut String) {
    trim_ending_whitespace(buf);
    if buf.ends_with('\n') {
        // Nothing to do
    } else if !buf.is_empty() {
        buf.push('\n')
    }
}

fn blockquote_start(buf: &mut String, prefix: &mut LinkedList<&str>) {
    ensure_newline(buf);
    prefix.push_back(">")
}

fn blockquote_end(buf: &mut String, prefix: &mut LinkedList<&str>) {
    prefix.pop_back();
    ensure_newline(buf)
}

fn link_start(buf: &mut String) {
    buf.push('[')
}

fn link_end(buf: &mut String, attrs: &[Attribute]) {
    let mut url = "";

    for attr in attrs {
        let attr_name: &str = &attr.name.local.to_ascii_lowercase();
        if attr_name == "href" {
            url = &attr.value;
        }
    }

    buf.push_str("](");
    buf.push_str(url);
    buf.push(')')
}

fn img_start(buf: &mut String, attrs: &[Attribute]) {
    let mut src = "";
    let mut alt = "no alt text";

    for attr in attrs {
        let attr_name: &str = &attr.name.local.to_ascii_lowercase();
        match attr_name {
            "src" => {
                src = &attr.value;
            }
            "alt" => {
                alt = &attr.value;
            }
            _ => {}
        }
    }

    buf.push_str("![");
    buf.push_str(alt);
    buf.push_str("](");
    buf.push_str(src);
    buf.push(')')
}

fn ul_start(buf: &mut String, list_markers: &mut Vec<Option<usize>>) {
    ensure_double_newline(buf);
    list_markers.push(None);
}

fn list_end(buf: &mut String, list_markers: &mut Vec<Option<usize>>) {
    ensure_double_newline(buf);
    list_markers.pop();
    prefix_with_necessary_spaces(buf, list_markers);
}

fn ol_start(buf: &mut String, list_markers: &mut Vec<Option<usize>>) {
    ensure_double_newline(buf);
    list_markers.push(Some(1));
}

fn li_start(buf: &mut String, list_markers: &[Option<usize>]) {
    if !list_markers.is_empty() {
        let last_index = list_markers.len() - 1;
        prefix_with_necessary_spaces(buf, list_markers.split_at(last_index).0);
        if let Some(prefix) = prefix(list_markers) {
            buf.push_str(&prefix);
        }
    }
}

fn li_end(buf: &mut String, list_markers: &mut Vec<Option<usize>>) {
    if let Some(mark) = list_markers.pop() {
        ensure_newline(buf);
        match mark {
            Some(i) => list_markers.push(Some(i + 1)),
            None => list_markers.push(mark),
        }
    }
}

#[test]
fn test_prefix_with_necessary_spaces() {
    let mut buf = String::new();
    prefix_with_necessary_spaces(&mut buf, &[]);
    assert_eq!("", &buf);

    let mut buf = String::new();
    prefix_with_necessary_spaces(&mut buf, &[None]);
    assert_eq!("  ", &buf);

    let mut buf = String::new();
    prefix_with_necessary_spaces(&mut buf, &[Some(3)]);
    assert_eq!("   ", &buf);

    let mut buf = String::new();
    prefix_with_necessary_spaces(&mut buf, &[Some(1), None, Some(2)]);
    assert_eq!("        ", &buf);

    let mut buf = String::new();
    prefix_with_necessary_spaces(&mut buf, &[Some(1), None, Some(2)].split_at(2).0);
    assert_eq!("     ", &buf);
}
