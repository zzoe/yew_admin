use ybc::NavbarFixed::Top;
use yew::prelude::*;

use home::Home;

pub mod home;

enum Msg {
    AddOne,
}

struct App {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <nav class="navbar" role="navigation" aria-label="main navigation">
  <div class="navbar-brand">
    <a class="navbar-item" href="https://bulma.io">
      <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28" />
    </a>

    <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
      <span aria-hidden="true"></span>
    </a>
  </div>

  <div id="navbarBasicExample" class="navbar-menu">
    <div class="navbar-start">
      <a class="navbar-item">
        {"Home"}
      </a>

      <a class="navbar-item">
        {"Documentation"}
      </a>

      <div class="navbar-item has-dropdown is-hoverable">
        <a class="navbar-link">
          {"More"}
        </a>

        <div class="navbar-dropdown">
          <a class="navbar-item">
          {"About"}
          </a>
          <a class="navbar-item">
          {"Jobs"}
          </a>
          <a class="navbar-item">
          {"Contact"}
          </a>
          <hr class="navbar-divider" />
          <a class="navbar-item">
          {"Report an issue"}
          </a>
        </div>
      </div>
    </div>

    <div class="navbar-end">
      <div class="navbar-item">
        <div class="buttons">
          <a class="button is-primary">
            <strong>{"Sign up"}</strong>
          </a>
          <a class="button is-light">
          {"Log in"}
          </a>
        </div>
      </div>
    </div>
  </div>
</nav>
            <ybc::Navbar fixed=Top />
            <Home />
            <ybc::Footer />
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
