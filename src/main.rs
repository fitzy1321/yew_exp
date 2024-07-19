use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use pages::{contact::Contact, home::Home};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={move |routes: Route| {
                match routes {
                    Route::Home => html! { <Home /> },
                    Route::Contact => html! { <Contact /> },
                }}} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
