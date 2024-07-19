use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component]
pub fn Home() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
    <div>
        <Navbar />
        <section class="section">
            <div class="container is-centered has-text-centered">
                <h1 class="title">{ "My example Yew App!!!" }</h1>

                <hr class="is-divider" />

                <div class="columns is-centered">
                    <div class="column has-text-centered">
                        <button class="button is-primary is-outlined" {onclick}>{ "+1" }</button>
                    </div>
                    <div class="column has-text-centered">
                        <p class="has-text-white is-5">{ *counter }</p>
                    </div>
                </div>
            </div>
        </section>
    </div>
    }
}
