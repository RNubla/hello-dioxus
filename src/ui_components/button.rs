#![allow(non_snake_case)]
use dioxus::prelude::*;
use tracing::{info, Level};

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {..props.attributes,
            { props.children}
        }
    }
}
