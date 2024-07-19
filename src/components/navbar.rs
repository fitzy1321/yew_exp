use yew::prelude::*;

#[function_component]
pub fn Navbar() -> Html {
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
