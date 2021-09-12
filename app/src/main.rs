use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;
use settings::Settings;

mod home;
mod settings;

// enum Msg {
//     AddOne,
// }

struct App {
    // `ComponentLink` is like a reference to a component.
// It can be used to send messages to the component
// link: ComponentLink<Self>,
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
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // match msg {
        //     Msg::AddOne => true,
        // }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // let onkeypress = self.link.batch_callback(|event| {
        //     if event.key() == "Enter" {
        //         Some(Msg::Submit)
        //     } else {
        //         None
        //     }
        // });

        html! {
            <>
            <nav class="navbar px-6 py-5" role="navigation" aria-label="main navigation">
              <div class="navbar-brand">
                <a class="navbar-item" href="http://127.0.0.1:8080">
                  <img src="resource/rustacean-flat-happy.svg" width="112" height="28" />
                </a>

                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                </a>
              </div>

              <div id="navbarBasicExample" class="navbar-menu">
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
