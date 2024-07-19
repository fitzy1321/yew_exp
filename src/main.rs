use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn Navbar() -> Html {
    html! {
    <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
        <div class="container">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">
                    <strong>{"My Yew Site"}</strong>
                </a>
                <a role="button" class="navbar-burger burger" aria-label="menu" aria-expanded="false" data-target="navbarMenu">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>
            <div id="navbarMenu" class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item" href="/">{"Home"}</a>
                    <a class="navbar-item" href="/contact">{"Contact"}</a>
                    <div class="navbar-item has-dropdown is-hoverable">
                        <a class="navbar-link">{"More"}</a>
                        <div class="navbar-dropdown">
                            <a class="navbar-item" href="#">{"Careers"}</a>
                            <hr class="navbar-divider" />
                            <a class="navbar-item" href="#">{"Report an issue"}</a>
                        </div>
                    </div>
                </div>
                <div class="navbar-end">
                    <div class="navbar-item">
                        <div class="buttons">
                            <a class="button is-light" href="#">{"Log in"}</a>
                            <a class="button is-primary" href="#">{"Sign up"}</a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </nav>
    }
}

#[function_component]
fn Home() -> Html {
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

#[function_component]
fn Contact() -> Html {
    html! {
    <div>
        <Navbar />
        {"Under Construction"}
    </div>
    }
}

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
