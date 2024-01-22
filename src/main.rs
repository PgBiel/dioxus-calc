#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let input = use_state::<i64>(cx, || 0);
    cx.render(rsx! {
        div {
            p {
                "Type it"
            }
            input {
                value: "{input}",
                prevent_default: "onkeypress",
                prevent_default: "onkeydown",
                onkeydown: move |evt| {
                    if let Key::Character(key) = evt.key() {
                        if let Ok(digit) = key.parse::<u8>() {
                            if digit < 10 {
                                // append digit
                                let current = input.get();
                                input.set(current * 10 + if *current == 0 { 1 } else { current.signum() } * (digit as i64));
                            }
                        } else if key == "-" {
                            if *input.get() == 0 {
                                // negative number
                                input.set(-1)
                            }
                        }
                    } else if let Key::Backspace = evt.key() {
                        // remove a digit from the end
                        let current = input.get();
                        input.set(current / 10);
                    }
                },
            }
            div {
                display: "grid",
                grid_template_columns: "repeat(3, 2rem)",
                grid_auto_flow: "row",
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 1)
                    },
                    "1"
                },
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 2)
                    },
                    "2"
                },
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 3)
                    },
                    "3"
                },
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 4)
                    },
                    "4"
                },
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 5)
                    },
                    "5"
                },
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 6)
                    },
                    "6"
                },
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 7)
                    },
                    "7"
                },
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 8)
                    },
                    "8"
                },
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 9)
                    },
                    "9"
                },
                button {
                    onclick: move |_| {
                        input.set(input.get() / 10);
                    },
                    "R"
                },
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 0)
                    },
                    "0"
                },
                button {
                    "="
                },
                button {
                    "+"
                },
                button {
                    "-"
                },
                button {
                    "*"
                },
            }
        }
    })
}
