use sycamore::prelude::*;

#[component]
pub fn Counter<G: Html>(ctx: ScopeRef) -> View<G> {
    let counter = ctx.create_signal(0);

    let decrement = |_| counter.set(*counter.get() - 1);
    let increment = |_| counter.set(*counter.get() + 1);

    view! { ctx,
        div(class="counters") {
            button(class="decrement", on:click=decrement) { "-" }
            span(class="value") { (counter.get()) }
            button(class="increment", on:click=increment) { "+" }
        }
    }
}
