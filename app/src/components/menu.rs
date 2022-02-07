use yew::html::Scope;
use yew::{html, Component, Context, Html, Properties};

pub enum MenuProp {
    Label(MenuNode),
    Fold(MenuNode),
    Item(MenuNode),
}

impl MenuProp {
    fn view(&self, link: &Scope<Menu>) -> Html {
        let onclick = link.callback(|_| Msg::Clicked);
        match self {
            MenuProp::Label(node) => {
                let views = if node.expanded {
                    html! {
                    <ul class="menu-list">
                        { node.children.iter().map(|child| child.view(link)).collect::<Html>() }
                    </ul>
                    }
                } else {
                    html! {}
                };

                html! {
                    <>
                    <div class="menu-label" {onclick}> {&*node.text} </div>
                    {views}
                    </>
                }
            }
            MenuProp::Fold(node) => {
                let is_active = node.active.then(|| "is-active");
                let views = if node.expanded {
                    html! {
                        { node.children.iter().map(|child| child.view(link)).collect::<Html>() }
                    }
                } else {
                    html! {}
                };

                html! {
                    <li>
                        <a class=classes!(is_active) {onclick}>
                            {&*node.text}
                        </a>
                        <ul>
                            { views }
                        </ul>
                    </li>
                }
            }
            MenuProp::Item(node) => {
                let is_active = node.active.then(|| "is-active");
                html! {
                    <li>
                        <a class=classes!(is_active) {onclick}>
                            {&*node.text}
                        </a>
                    </li>
                }
            }
        }
    }
}

pub struct MenuNode {
    pub id: u32,
    pub parent_id: u32,
    pub text: String,
    pub expanded: bool,
    pub active: bool,
    pub children: Vec<MenuProp>,
}

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    pub nodes: Vec<MenuProp>,
}

enum Msg {
    Clicked,
}

struct Menu;

impl Component for Menu {
    type Message = Msg;
    type Properties = MenuProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <aside class="menu">
                {for ctx.props().nodes.map(|node| node.view(ctx.link()))}
            </aside>
        }
    }
}
