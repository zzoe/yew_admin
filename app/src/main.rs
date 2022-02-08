use yew::prelude::*;
use yew_router::prelude::*;

use pages::home::Home;
use pages::settings::Settings;

mod components;
mod pages;

enum Msg {
    BuggerClick,
}

struct App {
    bugger_switch: bool,
}

#[derive(Clone, Routable, PartialEq)]
enum AppRoute {
    #[at("/home")]
    Home,
    #[at("/settings")]
    Settings,
    #[at("/")]
    Welcome,
}

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! {
            <Home />
        },
        AppRoute::Settings => html! {
            <Settings />
        },
        AppRoute::Welcome => html! {
            <p class="has-text-centered is-size-1">{"Welcome!"}</p>
        },
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            bugger_switch: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::BuggerClick => {
                self.bugger_switch = !self.bugger_switch;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let navbar_class = if self.bugger_switch { "is-active" } else { "" };
        let onclick = ctx.link().callback(|_| Msg::BuggerClick);

        html! {
            <BrowserRouter>
            <nav class="navbar px-6 py-5" role="navigation" aria-label="main navigation">
              <div class="navbar-brand">
                <Link<AppRoute> classes={"navbar-item"} to={AppRoute::Welcome}>
                  <img src="resource/rustacean-flat-happy.svg" width="112" height="28" />
                </Link<AppRoute>>

                <a role="button" class={classes!("navbar-burger", navbar_class)} {onclick}
                    aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                </a>
              </div>

              <div id="navbarBasicExample" class={classes!("navbar-menu", navbar_class)}>
                <div class="navbar-start">
                    <Link<AppRoute> classes={"navbar-item"} to={AppRoute::Home}>
                        { "Home" }
                    </Link<AppRoute>>
                    <Link<AppRoute> classes={"navbar-item"} to={AppRoute::Settings}>
                        {"Settings"}
                    </Link<AppRoute>>

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

            <main>
                    <Switch<AppRoute> render={Switch::render(switch)} />
            </main>

            <footer class="footer px-6 py-5">
                <div class="content has-text-centered">
                  <p>{"zoe © www.zoe.zz 鄂ICP备19880211号"}</p>
                </div>
            </footer>
            </BrowserRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
