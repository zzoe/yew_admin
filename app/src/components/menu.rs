use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

use crate::util::{neq_assign, WeakComponentLink};

#[derive(PartialEq, Clone, Debug)]
pub enum MenuNode {
    Label(MenuLabel),
    Fold(MenuFold),
    Item(MenuItem),
}

impl MenuNode {
    pub fn inactive(&mut self) {
        match self {
            MenuNode::Item(i) => i.is_active = false,
            MenuNode::Fold(f) => f.is_active = false,
            _ => (),
        }
    }
}

impl Default for MenuNode {
    fn default() -> Self {
        Self::Label(MenuLabel::default())
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct MenuLabel {
    pub id: i32,
    pub label_text: String,
    pub expanded: bool,
    pub nodes: Vec<Rc<RefCell<MenuNode>>>,
}

impl MenuLabel {
    fn view(&self, active_link: &WeakComponentLink<Node>, click_cb: Callback<MouseEvent>) -> Html {
        let views = if self.expanded {
            let views = self
                .nodes
                .iter()
                .map(|node| {
                    html! {
                        <Node node=node active_link=active_link/>
                    }
                })
                .collect::<Html>();

            html!(
            <ul class="menu-list">
                { views }
            </ul>
            )
        } else {
            html! {}
        };

        html! {
            <>
            <p class="menu-label" onclick=click_cb>{&*self.label_text}</p>
                {views}
            </>
        }
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct MenuFold {
    pub id: i32,
    pub is_active: bool,
    pub expanded: bool,
    pub fold_text: String,
    pub nodes: Vec<Rc<RefCell<MenuNode>>>,
}

impl MenuFold {
    fn view(&self, active_link: &WeakComponentLink<Node>, click_cb: Callback<MouseEvent>) -> Html {
        let views = if self.expanded {
            self.nodes
                .iter()
                .map(|node| {
                    html! {
                        <Node node=node active_link=active_link />
                    }
                })
                .collect::<Html>()
        } else {
            html! {}
        };

        let is_active = self.is_active.then(|| "is-active");
        html! {
            <li>
                <a class=classes!(is_active) onclick=click_cb>
                    {&*self.fold_text}
                </a>
                <ul>
                    { views }
                </ul>
            </li>
        }
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct MenuItem {
    pub id: i32,
    pub is_active: bool,
    pub item_text: String,
}

impl MenuItem {
    fn view(&self, click_cb: Callback<MouseEvent>) -> Html {
        let is_active = self.is_active.then(|| "is-active");
        html! {
            <li>
                <a class=classes!(is_active) onclick=click_cb>
                    {&*self.item_text}
                </a>
            </li>
        }
    }
}

pub enum Msg {
    Clicked,
    Inactive,
}

pub struct Menu {
    pub props: MenuProps,
}

#[derive(Properties, PartialEq, Clone)]
pub struct MenuProps {
    #[prop_or_default]
    pub nodes: Rc<Vec<Rc<RefCell<MenuNode>>>>,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = MenuProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        neq_assign(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let link = &WeakComponentLink::new(None);
        let views = self
            .props
            .nodes
            .iter()
            .map(|node| {
                html! {
                    <Node node=node active_link=link/>
                }
            })
            .collect::<Html>();

        html! {
        <aside class="menu">
            {views}
        </aside>
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub props: NodeProps,
    pub link: WeakComponentLink<Self>,
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct NodeProps {
    #[prop_or_default]
    pub node: Rc<RefCell<MenuNode>>,
    #[prop_or_default]
    pub active_link: WeakComponentLink<Node>,
}

impl Component for Node {
    type Message = Msg;
    type Properties = NodeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link = WeakComponentLink::new(Some(link));
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => match &mut *self.props.node.borrow_mut() {
                MenuNode::Label(label) => {
                    label.expanded = !label.expanded;
                    self.inactive();
                    true
                }
                MenuNode::Fold(fold) => {
                    fold.expanded = !fold.expanded;
                    if !std::mem::replace(&mut fold.is_active, true) {
                        self.inactive();
                        self.active();
                    }
                    true
                }
                MenuNode::Item(item) => {
                    let inactive = !std::mem::replace(&mut item.is_active, true);
                    if inactive {
                        self.inactive();
                        self.active();
                    }
                    inactive
                }
            },
            Msg::Inactive => {
                self.props.node.borrow_mut().inactive();
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        neq_assign(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let node = &*self.props.node.borrow();
        let click_cb = self
            .link
            .borrow()
            .as_ref()
            .unwrap()
            .callback(|_| Msg::Clicked);

        let views = match node {
            MenuNode::Label(label) => label.view(&self.props.active_link, click_cb),
            MenuNode::Fold(fold) => fold.view(&self.props.active_link, click_cb),
            MenuNode::Item(item) => item.view(click_cb),
        };

        html! { {views} }
    }
}

impl Node {
    fn active(&self) {
        *self.props.active_link.borrow_mut() = self.link.borrow().clone();
    }

    fn inactive(&self) {
        self.props
            .active_link
            .borrow_mut()
            .as_ref()
            .and_then(|node| Some(node.send_message(Msg::Inactive)));
    }
}
