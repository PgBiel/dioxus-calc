#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_desktop::use_window;

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
    Pow,
}

impl Op {
    /// Applies this operation on the two operands, returning the operation's result.
    fn apply(self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs.saturating_add(rhs),
            Self::Sub => lhs.saturating_sub(rhs),
            Self::Mul => lhs.saturating_mul(rhs),
            Self::Div => lhs.saturating_div(rhs),
            Self::Pow => match u32::try_from(rhs) {
                Ok(exp) => lhs.saturating_pow(exp),
                Err(_) => 0,
            },
        }
    }
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let window = use_window(cx);
    window.set_title("Calculator");
    // Currently visible input.
    let input = use_state::<i64>(cx, || 0);
    // If input should be cleared on the next digit.
    // Toggled on when an operation is selected (the LHS still displays, but RHS will begin with the next digit).
    let input_should_clear = use_state::<bool>(cx, || false);
    // Get input or clear it if needed.
    let get_input_or_clear = move || {
        if *input_should_clear.get() {
            input_should_clear.set(false);
            input.set(0);
            0
        } else {
            *input.get()
        }
    };
    // Append a digit at the end of the input.
    let push_digit = move |digit: u8| {
        assert!(digit < 10);
        let current = get_input_or_clear();
        let sign = if current < 0 { -1 } else { 1 };
        input.set(current * 10 + sign * (digit as i64));
    };
    // Remove a digit from the input.
    let pop_digit = move || {
        let current = get_input_or_clear();
        input.set(current / 10);
    };
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
        input_should_clear.set(false);
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
        // Ensure the user will type the RHS afterwards.
        // Otherwise, use the LHS (still displayed if the user doesn't type) as RHS.
        input_should_clear.set(true);
        operation.set(Some(op));
    };
    cx.render(rsx! {
        div {
            h1 { "Calculator" }
            p {
                "Type it"
            }
            input {
                value: "{input}",
                prevent_default: "onkeydown",
                onkeydown: move |evt| {
                    if let Key::Character(key) = evt.key() {
                        if let Ok(digit) = key.parse::<u8>() {
                            if digit < 10 {
                                push_digit(digit);
                            }
                        } else if key == "-" {
                            if get_input_or_clear() == 0 {
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
                // 3 columns, N rows grid
                grid_template_columns: "repeat(3, 2rem)",
                grid_auto_flow: "row",
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_digit(1),
                    "1"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_digit(2),
                    "2"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_digit(3),
                    "3"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_digit(4),
                    "4"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_digit(5),
                    "5"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_digit(6),
                    "6"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_digit(7),
                    "7"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_digit(8),
                    "8"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_digit(9),
                    "9"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| pop_digit(),
                    "D"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_digit(0),
                    "0"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| {
                        apply_op();
                    },
                    "="
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| {
                        reset_op();
                    },
                    "C"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_op(Op::Add),
                    "+"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_op(Op::Sub),
                    "-"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_op(Op::Mul),
                    "*"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_op(Op::Div),
                    "/"
                },
                button {
                    background_color: "blue",
                    color: "white",
                    onclick: move |_| push_op(Op::Pow),
                    "**"
                }
            }
        }
    })
}
