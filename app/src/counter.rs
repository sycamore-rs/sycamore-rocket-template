use sycamore::prelude::*;

#[component(Counter<G>)]
pub fn counter() -> View<G> {
    let counter = Signal::new(0);

    let decrement = cloned!((counter) => move |_| counter.set(*counter.get() - 1));
    let increment = cloned!((counter) => move |_| counter.set(*counter.get() + 1));

    view! {
        div(class="counters") {
            button(class="decrement", on:click=decrement) { "-" }
            span(class="value") { (counter.get()) }
            button(class="increment", on:click=increment) { "+" }
        }
    }
}
