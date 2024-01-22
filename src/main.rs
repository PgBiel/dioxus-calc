#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    /// Applies this operation on the two operands, returning the operation's result.
    fn apply(self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs.saturating_add(rhs),
            Self::Sub => lhs.saturating_sub(rhs),
            Self::Mul => lhs.saturating_mul(rhs),
            Self::Div => lhs.saturating_div(rhs),
        }
    }
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    // Currently visible input.
    let input = use_state::<i64>(cx, || 0);
    // Currently selected operation.
    let operation = use_state::<Option<Op>>(cx, || None);
    // When an operation is in progress, this temporarily stores the LHS. The RHS will be taken from 'input'.
    let lhs = use_state::<Option<i64>>(cx, || None);
    // Push the current input to LHS.
    let push_lhs = || lhs.set(Some(*input.get()));
    // Cancels the currently selected operation.
    let reset_op = || {
        operation.set(None);
        lhs.set(None);
    };
    // Applies the current operation between the current LHS and the input as RHS, if possible.
    let apply_op = move || {
        if let Some((op, lhs)) = (*operation.get()).zip(*lhs.get()) {
            let rhs = *input.get();
            input.set(op.apply(lhs, rhs));
            reset_op();
        }
    };
    let push_op = move |op: Op| {
        apply_op();
        // Use the output from the previous operation (if any) as the next's LHS.
        // Otherwise, use the current input.
        push_lhs();
        // reset input (temp solution)
        input.set(0);
        operation.set(Some(op));
    };
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
                        // Remove last digit
                        input.set(input.get() / 10);
                    },
                    "D"
                },
                button {
                    onclick: move |_| {
                        let current = *input.get();
                        input.set(current * 10 + if current == 0 { 1 } else { current.signum() } * 0)
                    },
                    "0"
                },
                button {
                    onclick: move |_| {
                        apply_op();
                    },
                    "="
                },
                button {
                    onclick: move |_| {
                        reset_op();
                    },
                    "C"
                },
                button {
                    onclick: move |_| {
                        push_op(Op::Add);
                    },
                    "+"
                },
                button {
                    onclick: move |_| {
                        push_op(Op::Sub);
                    },
                    "-"
                },
                button {
                    onclick: move |_| {
                        push_op(Op::Mul);
                    },
                    "*"
                },
                button {
                    onclick: move |_| {
                        push_op(Op::Div);
                    },
                    "/"
                },
                button {
                    " "
                }
            }
        }
    })
}
