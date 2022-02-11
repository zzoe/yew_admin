use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::menu::Menu;
use crate::Msg;

#[derive(Clone, Routable, PartialEq)]
pub enum MenuRoute {
    #[at("/home/fn1")]
    Fn1,
    #[at("/home/fn2")]
    Fn2,
}

impl MenuRoute {
    pub fn from_str(fn_name: impl AsRef<str>) -> Self {
        match fn_name.as_ref().to_ascii_lowercase().as_str() {
            "fn1" => MenuRoute::Fn1,
            _ => MenuRoute::Fn2,
        }
    }
}

fn switch_menu(routes: &MenuRoute) -> Html {
    match routes {
        MenuRoute::Fn1 => html! {
            <p class="has-text-centered is-size-1">{"Fn1!"}</p>
        },
        MenuRoute::Fn2 => html! {
            <div class="box">
                <div class="tabs is-toggle">
                    <ul>
                        <li class="is-active"><a>{"Pictures"}</a></li>
                        <li><a>{"Music"}</a></li>
                        <li><a>{"Videos"}</a></li>
                        <li><a>{"Documents"}</a></li>
                    </ul>
                </div>
                <div class="field is-grouped is-grouped-multiline">
                    <p class="control has-icons-left">
                        <input class="input" type="text" placeholder="Name" />
                        <span class="icon is-small is-left">
                            <i class="fas fa-user"></i>
                        </span>
                    </p>
                    <p class="control has-icons-left has-icons-right">
                        <input class="input is-success" type="email" placeholder="Email" value="zoe211@163.com" />
                        <span class="icon is-small is-left">
                            <i class="fas fa-envelope"></i>
                        </span>
                        <span class="icon is-small is-right">
                            <i class="fas fa-check"></i>
                        </span>
                    </p>
                    <p class="control">
                        <a class="button is-primary">{"Search"}</a>
                    </p>
                </div>
                <div class="table-container">
                    <table class="table is-bordered is-striped is-narrow is-hoverable is-fullwidth">
                        <thead>
                          <tr>
                            <th><abbr title="Position">{"Pos"}</abbr></th>
                            <th>{"Team"}</th>
                            <th><abbr title="Played">{"Pld"}</abbr></th>
                            <th><abbr title="Won">{"W"}</abbr></th>
                            <th><abbr title="Drawn">{"D"}</abbr></th>
                            <th><abbr title="Lost">{"L"}</abbr></th>
                            <th><abbr title="Goals for">{"GF"}</abbr></th>
                            <th><abbr title="Goals against">{"GA"}</abbr></th>
                            <th><abbr title="Goal difference">{"GD"}</abbr></th>
                            <th><abbr title="Points">{"Pts"}</abbr></th>
                            <th>{"Qualification or relegation"}</th>
                          </tr>
                        </thead>
                        <tbody>
                          <tr>
                            <td>{"1"}</td>
                            <td><a href="https://en.wikipedia.org/wiki/Leicester_City_F.C." title="Leicester City F.C.">{"Leicester City"}</a></td>
                            <td>{"38"}</td>
                            <td>{"23"}</td>
                            <td>{"12"}</td>
                            <td>{"3"}</td>
                            <td>{"68"}</td>
                            <td>{"36"}</td>
                            <td>{"+32"}</td>
                            <td>{"81"}</td>
                            <td>{"Qualification for the "}<a href="https://en.wikipedia.org/wiki/2016%E2%80%9317_UEFA_Champions_League#Group_stage" title="2016–17 UEFA Champions League">{"Champions League group stage"}</a></td>
                          </tr>
                        </tbody>
                    </table>
                </div>
                <nav class="level">
                    <div class="level-left" />
                    <div class="level-right">
                        <div class="level-item">
                            <div class="buttons has-addons">
                                <button class="button is-small">{"|<-"}</button>
                                <button class="button is-small">{"<<<"}</button>
                                <button class="button is-small">{">>>"}</button>
                                <button class="button is-small">{"->|"}</button>
                            </div>
                        </div>
                        <div class="level-item">
                            <div class="field has-addons">
                                <div class="control">
                                    <input class="input is-small" width="1rem" type="text" placeholder="Page No." />
                                </div>
                                <div class="control">
                                    <a class="button is-info is-small">{"Go"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="level-item">
                            <div class="field has-addons">
                                <div class="control">
                                    <input class="input is-small" type="text" placeholder="Rows/Page" />
                                </div>
                                <div class="control">
                                    <a class="button is-info is-small">{"Set"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </nav>
            </div>
        },
    }
}

pub struct Home;

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        return html! {
            <div class="columns">
                <div class="column is-narrow">
                    <div class="box">
                        <Menu />
                    </div>
                </div>
                <div class="column">
                    <Switch<MenuRoute> render={Switch::render(switch_menu)} />
                </div>
            </div>
        };
    }
}
