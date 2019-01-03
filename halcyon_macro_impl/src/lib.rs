extern crate proc_macro;

use proc_macro::{Group, Ident, Literal, Punct, TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;

#[derive(Debug)]
struct Element {
    tag: String,
    children: Vec<ElementChild>,
    attributes: Vec<Attribute>,
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
    mut tokens_iter: std::iter::Peekable<proc_macro::token_stream::IntoIter>,
) -> (
    std::iter::Peekable<proc_macro::token_stream::IntoIter>,
    Attribute,
) {
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
    mut tokens_iter: std::iter::Peekable<proc_macro::token_stream::IntoIter>,
) -> Result<
    (
        std::iter::Peekable<proc_macro::token_stream::IntoIter>,
        Element,
    ),
    Box<std::error::Error>,
> {
    if let Some(TokenTree::Ident(tag)) = tokens_iter.next() {
        let mut attributes: Vec<Attribute> = vec![];
        loop {
            println!("{:?}", tokens_iter.peek());
            if let Some(TokenTree::Punct(next_token)) = tokens_iter.peek() {
                if next_token.to_string() == ">" {
                    break;
                } else {
                    panic!("unexpected end of element")
                }
            } else if let Some(TokenTree::Ident(next_token)) = tokens_iter.peek() {
                let result = parse_attribute(tokens_iter);
                tokens_iter = result.0;
                attributes.push(result.1);
            } else {
                panic!("unexpected member of element")
            }
        }
        println!("!! {:?}", tokens_iter.peek());
        if let Some(TokenTree::Punct(t)) = tokens_iter.next() {
            if t.to_string() == ">" {
                let mut children: Vec<ElementChild> = vec![];
                loop {
                    let next = tokens_iter.next();
                    if let Some(TokenTree::Punct(t)) = next {
                        println!("checking if next tag is new child or end tag");
                        if t.to_string() == "<" {
                            if let Some(TokenTree::Punct(t)) = tokens_iter.peek() {
                                if t.to_string() == "/" {
                                    println!("end tag");
                                    // if we are at end tag
                                    // fast forward
                                    tokens_iter.next(); // /
                                    tokens_iter.next(); // <tag>
                                    tokens_iter.next(); // >
                                    break;
                                }
                            } else if let Some(TokenTree::Ident(t)) = tokens_iter.peek() {
                                println!("new child");
                                // if we might be at new child
                                let result = parse_element(tokens_iter)?;
                                tokens_iter = result.0;
                                println!("{:?}", result.1);
                                children.push(ElementChild::Element(result.1));
                            } else {
                                panic!("unexpected child");
                            }
                        }
                    } else if let Some(TokenTree::Group(t)) = next {
                        children.push(ElementChild::Code(t));
                    } else {
                        println!("{:?}", next);
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

#[proc_macro_hack]
pub fn html(input: TokenStream) -> TokenStream {
    let mut tokens_iter = input.into_iter().peekable();
    if let Some(TokenTree::Punct(t)) = tokens_iter.next() {
        if t.to_string() == "<" {
            let result = parse_element(tokens_iter).unwrap();
            tokens_iter = result.0;
            println!("hey! {:?}", result.1);
        } else {
            panic!("html! macro contents did not start with an element tag")
        }
    }
    TokenTree::Literal(Literal::u32_suffixed(42)).into()
}
