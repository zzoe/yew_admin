use yew::prelude::*;

pub struct Home {
    pub props: HomeProps,
    pub link: ComponentLink<Self>,
    pub label: String,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HomeProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Home {
    type Message = ();
    type Properties = HomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            label: "Menu label".to_owned(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="columns">
              <div class="column is-narrow">
                <div class="box">
                    <aside class="menu">
                      <p class="menu-label">
                        {"General"}
                      </p>
                      <ul class="menu-list">
                        <li><a>{"Dashboard"}</a></li>
                        <li><a>{"Customers"}</a></li>
                      </ul>
                      <p class="menu-label">
                        {"Administration"}
                      </p>
                      <ul class="menu-list">
                        <li><a>{"Team Settings"}</a></li>
                        <li>
                          <a class="is-active">{"Manage Your Team"}</a>
                          <ul>
                            <li><a>{"Members"}</a></li>
                            <li><a>{"Plugins"}</a></li>
                            <li><a>{"Add a member"}</a></li>
                          </ul>
                        </li>
                        <li><a>{"Invitations"}</a></li>
                      </ul>
                      <p class="menu-label">
                        {"Transactions"}
                      </p>
                      <ul class="menu-list">
                        <li><a>{"Payments"}</a></li>
                        <li><a>{"Transfers"}</a></li>
                        <li><a>{"Balance"}</a></li>
                      </ul>
                    </aside>
                </div>
              </div>
              <div class="column">
                <div class="box">
                    // <nav class="panel is-fullwidth">
                        <div class="tabs is-toggle is-fullwidth">
                            <ul>
                                <li class="is-active"><a>{"Pictures"}</a></li>
                                <li><a>{"Music"}</a></li>
                                <li><a>{"Videos"}</a></li>
                                <li><a>{"Documents"}</a></li>
                            </ul>
                        </div>
                        // <div class="panel-block">
                        // </div>
                    // </nav>
                </div>
              </div>
            </div>
        }
    }
}
