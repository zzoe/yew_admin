use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;
use settings::Settings;

mod home;
mod settings;

enum Msg {
    BuggerClick,
}

struct App {
    link: ComponentLink<Self>,
    bugger_switch: bool,
}

#[derive(Switch, Clone, Debug)]
enum AppRoute {
    #[to = "/home"]
    Home,
    #[to = "/settings"]
    Settings,
    #[to = "/"]
    Welcome,
}

fn switch(routes: AppRoute) -> Html {
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

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            bugger_switch: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::BuggerClick => {
                self.bugger_switch = !self.bugger_switch;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let navbar_class = if self.bugger_switch { "is-active" } else { "" };
        let on_bugger_click = self.link.callback(|_| Msg::BuggerClick);

        html! {
            <>
            <nav class="navbar px-6 py-5" role="navigation" aria-label="main navigation">
              <div class="navbar-brand">
                <RouterAnchor<AppRoute> classes={"navbar-item"} route={AppRoute::Welcome}>
                  <img src="resource/rustacean-flat-happy.svg" width="112" height="28" />
                </RouterAnchor<AppRoute>>

                <a role="button" class=classes!("navbar-burger", navbar_class) onclick=on_bugger_click
                    aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                </a>
              </div>

              <div id="navbarBasicExample" class=classes!("navbar-menu", navbar_class)>
                <div class="navbar-start">
                    <RouterAnchor<AppRoute> classes={"navbar-item"} route={AppRoute::Home}>
                        { "Home" }
                    </RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> classes={"navbar-item"} route={AppRoute::Settings}>
                        {"Settings"}
                    </RouterAnchor<AppRoute>>

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
                <Router<AppRoute, ()> render=Router::render(switch) />
            </main>

            <footer class="footer px-6 py-5">
                <div class="content has-text-centered">
                  <p>{"zoe © www.zoe.zz 鄂ICP备19880211号"}</p>
                </div>
            </footer>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
