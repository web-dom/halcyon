extern crate proc_macro;

use proc_macro::{Group, TokenStream, TokenTree};
use std::error::Error;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug)]
struct Element {
    tag: String,
    children: Vec<ElementChild>,
    attributes: Vec<Attribute>,
}

fn to_snake(t:&str) -> String {
    let mut s = String::new();
    for (i,c) in t.chars().enumerate() {
        let new_char = c;
        if i > 0 && c.is_uppercase() {
            s.push_str(&"_");
        }
        s.push_str(&new_char.to_lowercase().to_string());
    }
    s
}

impl Element {
    fn to_token_stream(&self) -> TokenStream {
        let mut children_token_stream = String::from("None");
        if self.children.len() > 0 {
            let children_list_str = self
                .children
                .iter()
                .map(|x| match x {
                    ElementChild::Element(e) => e.to_token_stream().to_string(),
                    ElementChild::Code(c) => {
                        format!(r#"halcyon::VirtualNode::from({})"#, c.stream().to_string())
                    }
                })
                .collect::<Vec<String>>()
                .join(",");
            children_token_stream = String::from(format!("Some(vec![{}])", children_list_str));
        }

        let mut attributes_token_stream = String::from("None");
        if self.attributes.len() > 0 {
            let attributes_list_str = self
                .attributes
                .iter()
                .map(|a| match &a.value {
                    AttributeValue::Text(t) => format!(
                        r#"h.insert("{}".to_string(),halcyon::Prop::from({}.to_string()));"#,
                        a.name, t
                    ),
                    AttributeValue::Code(c) => format!(
                        r#"h.insert("{}".to_string(),halcyon::Prop::from({}));"#,
                        a.name,
                        c.stream().to_string()
                    ),
                })
                .collect::<Vec<String>>()
                .join("\n");
            attributes_token_stream = String::from(format!(
                r#"Some({{
                let mut h = halcyon::Props::new();
                {}
                h
            }})"#,
                attributes_list_str
            ));
        }

        let c = self.tag.chars().next().unwrap();
        if c.is_uppercase() {
            TokenStream::from_str(&format!(
                r#"halcyon::c(halcyon::C::from({}), {}, {})"#,
                to_snake(&self.tag), attributes_token_stream, children_token_stream
            ))
            .expect("invalid token stream")
        } else {
            TokenStream::from_str(&format!(
                r#"halcyon::h("{}", {}, {})"#,
                self.tag, attributes_token_stream, children_token_stream
            ))
            .expect("invalid token stream")
        }
    }
}

#[derive(Debug)]
enum ElementChild {
    Element(Element),
    Code(Group),
}

#[derive(Debug)]
enum AttributeValue {
    Text(String),
    Code(Group),
}

#[derive(Debug)]
struct Attribute {
    name: String,
    value: AttributeValue,
}

fn parse_attribute(
    mut tokens_iter: Peekable<proc_macro::token_stream::IntoIter>,
) -> (Peekable<proc_macro::token_stream::IntoIter>, Attribute) {
    if let Some(TokenTree::Ident(name)) = tokens_iter.next() {
        if let Some(TokenTree::Punct(t)) = tokens_iter.next() {
            if t.to_string() == "=" {
                let next = tokens_iter.next();
                if let Some(TokenTree::Literal(value)) = next {
                    return (
                        tokens_iter,
                        Attribute {
                            name: name.to_string(),
                            value: AttributeValue::Text(value.to_string()),
                        },
                    );
                } else if let Some(TokenTree::Group(value)) = next {
                    return (
                        tokens_iter,
                        Attribute {
                            name: name.to_string(),
                            value: AttributeValue::Code(value),
                        },
                    );
                } else {
                    panic!("did not expect this from attribute");
                }
            }
        }
    }
    panic!("should not be here")
}

fn parse_element(
    mut tokens_iter: Peekable<proc_macro::token_stream::IntoIter>,
) -> Result<(Peekable<proc_macro::token_stream::IntoIter>, Element), Box<Error>> {
    if let Some(TokenTree::Ident(tag)) = tokens_iter.next() {
        let mut attributes: Vec<Attribute> = vec![];
        loop {
            if let Some(TokenTree::Punct(next_token)) = tokens_iter.peek() {
                if next_token.to_string() == ">" {
                    break;
                } else if next_token.to_string() == "/" {
                    //skip the /
                    tokens_iter.next();
                    if let Some(TokenTree::Punct(next_token)) = tokens_iter.peek() {
                        if next_token.to_string() == ">" {
                            return Ok((
                                tokens_iter,
                                Element {
                                    tag: tag.to_string(),
                                    children: vec![],
                                    attributes: attributes,
                                },
                            ));
                        } else {
                            panic!("unexpected short end of element")
                        }
                    }
                } else {
                    panic!("unexpected end of element")
                }
            } else if let Some(TokenTree::Ident(_next_token)) = tokens_iter.peek() {
                let result = parse_attribute(tokens_iter);
                tokens_iter = result.0;
                attributes.push(result.1);
            } else {
                panic!("unexpected member of element")
            }
        }
        if let Some(TokenTree::Punct(t)) = tokens_iter.next() {
            if t.to_string() == ">" {
                let mut children: Vec<ElementChild> = vec![];
                loop {
                    let next = tokens_iter.next();
                    if let Some(TokenTree::Punct(t)) = next {
                        if t.to_string() == "<" {
                            if let Some(TokenTree::Punct(t)) = tokens_iter.peek() {
                                if t.to_string() == "/" {
                                    // if we are at end tag
                                    // fast forward
                                    tokens_iter.next(); // /
                                    tokens_iter.next(); // <tag>
                                    tokens_iter.next(); // >
                                    break;
                                }
                            } else if let Some(TokenTree::Ident(_t)) = tokens_iter.peek() {
                                // if we might be at new child
                                let result = parse_element(tokens_iter)?;
                                tokens_iter = result.0;
                                children.push(ElementChild::Element(result.1));
                            } else {
                                panic!("unexpected child");
                            }
                        }
                    } else if let Some(TokenTree::Group(t)) = next {
                        children.push(ElementChild::Code(t));
                    } else {
                        panic!("unexpected next element")
                    }
                }
                return Ok((
                    tokens_iter,
                    Element {
                        tag: tag.to_string(),
                        children: children,
                        attributes: attributes,
                    },
                ));
            }
        } else {
            panic!("unxpected end of attributes")
        }
    }
    panic!("html tag did not start with an identifier")
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let mut tokens_iter = input.into_iter().peekable();
    if let Some(TokenTree::Punct(t)) = tokens_iter.next() {
        if t.to_string() == "<" {
            let result = parse_element(tokens_iter).unwrap();
            return result.1.to_token_stream();
        } else {
            panic!("html! macro contents did not start with an element tag")
        }
    }
    panic!("html! macro did not start with punct")
}
