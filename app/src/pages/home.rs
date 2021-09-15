use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

use crate::components::menu::{Menu, MenuFold, MenuItem, MenuLabel, MenuNode};
use crate::util::neq_assign;

pub struct Home {
    pub props: HomeProps,
    pub link: ComponentLink<Self>,
    pub labels: Rc<Vec<Rc<RefCell<MenuLabel>>>>,
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
        let labels = Rc::new(vec![
            Rc::new(RefCell::new(MenuLabel {
                id: 1,
                label_text: "label_1".to_string(),
                expanded: true,
                menu_list: vec![
                    Rc::new(RefCell::new(MenuNode::Item(MenuItem {
                        id: 2,
                        is_active: false,
                        item_text: "Dashboard".to_string(),
                    }))),
                    Rc::new(RefCell::new(MenuNode::Item(MenuItem {
                        id: 3,
                        is_active: false,
                        item_text: "Customers".to_string(),
                    }))),
                ],
            })),
            Rc::new(RefCell::new(MenuLabel {
                id: 4,
                label_text: "Administration".to_string(),
                expanded: true,
                menu_list: vec![
                    Rc::new(RefCell::new(MenuNode::Item(MenuItem {
                        id: 5,
                        is_active: false,
                        item_text: "Team Settings".to_string(),
                    }))),
                    Rc::new(RefCell::new(MenuNode::Fold(MenuFold {
                        id: 6,
                        is_active: false,
                        expanded: true,
                        fold_text: "Manage Your Team".to_string(),
                        nodes: vec![
                            Rc::new(RefCell::new(MenuNode::Item(MenuItem {
                                id: 7,
                                is_active: false,
                                item_text: "Projects".to_string(),
                            }))),
                            Rc::new(RefCell::new(MenuNode::Item(MenuItem {
                                id: 8,
                                is_active: false,
                                item_text: "Members".to_string(),
                            }))),
                            Rc::new(RefCell::new(MenuNode::Fold(MenuFold {
                                id: 9,
                                is_active: false,
                                expanded: true,
                                fold_text: "Manage Your Team".to_string(),
                                nodes: vec![
                                    Rc::new(RefCell::new(MenuNode::Item(MenuItem {
                                        id: 10,
                                        is_active: false,
                                        item_text: "Projects".to_string(),
                                    }))),
                                    Rc::new(RefCell::new(MenuNode::Item(MenuItem {
                                        id: 11,
                                        is_active: false,
                                        item_text: "Members".to_string(),
                                    }))),
                                    Rc::new(RefCell::new(MenuNode::Fold(MenuFold {
                                        id: 12,
                                        is_active: false,
                                        expanded: true,
                                        fold_text: "Manage Your Team".to_string(),
                                        nodes: vec![
                                            Rc::new(RefCell::new(MenuNode::Item(MenuItem {
                                                id: 13,
                                                is_active: false,
                                                item_text: "Projects".to_string(),
                                            }))),
                                            Rc::new(RefCell::new(MenuNode::Item(MenuItem {
                                                id: 14,
                                                is_active: false,
                                                item_text: "Members".to_string(),
                                            }))),
                                        ],
                                    }))),
                                ],
                            }))),
                        ],
                    }))),
                ],
            })),
        ]);

        Self {
            props,
            link,
            labels,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        neq_assign(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let labels = Rc::clone(&self.labels);
        html! {
            <div class="columns">
                <div class="column is-narrow">
                    <div class="box">
                        <Menu labels=labels />
                    </div>
                </div>
                <div class="column">
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
                </div>
            </div>
        }
    }
}
