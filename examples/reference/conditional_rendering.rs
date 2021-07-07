//! Example: Conditional Rendering
//! ------------------------------
//!
//! This example shows how to hide or show elements using conditional rendering.
//!
//! Often times you might want to display a different UI given some sort of condition. This is called "conditonal rendering".
//! In Dioxus, you can perform conditional rendering with optionals or matching.
//!
//! The rsx! and html! macro accepts anything that is `IntoIter<Item = impl IntoVnode>`. Options and Results both implement
//! IntoIter for impl VNode, so you can use option/result for conditional rendering.

use dioxus::prelude::*;

fn main() {}

// Convert a boolean conditional into a hide/show
#[derive(PartialEq, Props)]
struct MyProps {
    should_show: bool,
}
static Example: FC<MyProps> = |cx| {
    cx.render(rsx! {
        div {
            {cx.should_show.then(|| rsx!{
                h1 { "showing the title!" }
            })}
        }
    })
};

// Convert a boolean conditional into an either/or
// Because rsx! is lazy (produces a closure), we cannot use it in two branch arms. To use it in matching/branching, we
// must render it.
//
// Dioxus will let you render any `LazyNodes` into a `VNode` with `cx.render`. `rsx!` also supports the `in cx` syntax
// which will do essentially the same thing as `cx.render`.
//
// In short:
// `rsx!(in cx, ...)` is shorthand for `cx.render(rsx!(...))`
#[derive(PartialEq, Props)]
struct MyProps1 {
    should_show: bool,
}
static Example1: FC<MyProps1> = |cx| {
    cx.render(rsx! {
        div {
            // With matching
            {match cx.should_show {
                true => cx.render(rsx!(div {"it is true!"})),
                false => rsx!(in cx, div {"it is false!"}),
            }}

            // or with just regular conditions
            {if cx.should_show {
                rsx!(in cx, div {"it is true!"})
            } else {
                rsx!(in cx, div {"it is false!"})
            }}

            // or with optional chaining
            {
                cx.should_show
                .then(|| rsx!(in cx, div {"it is false!"}))
                .unwrap_or_else(|| rsx!(in cx, div {"it is false!"}))
            }
        }
    })
};

/// Matching can be expanded

#[derive(PartialEq)]
enum Color {
    Green,
    Yellow,
    Red,
}
#[derive(PartialEq, Props)]
struct MyProps2 {
    color: Color,
}
static Example2: FC<MyProps2> = |cx| {
    cx.render(rsx! {
        div {
            {match cx.color {
                Color::Green => rsx!(in cx, div {"it is Green!"}),
                Color::Yellow => rsx!(in cx, div {"it is Yellow!"}),
                Color::Red => rsx!(in cx, div {"it is Red!"}),
            }}
        }
    })
};