use sycamore::prelude::*;

#[component]
pub fn Counter<G: Html>(cx: Scope) -> View<G> {
    let counter = create_signal(cx, 0);

    let decrement = |_| counter.set(*counter.get() - 1);
    let increment = |_| counter.set(*counter.get() + 1);

    view! { cx,
        div(class="counters") {
            button(class="decrement", on:click=decrement) { "-" }
            span(class="value") { (counter.get()) }
            button(class="increment", on:click=increment) { "+" }
        }
    }
}
