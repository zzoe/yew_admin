use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;
use yew::services::ConsoleService;

use crate::util::{neq_assign, WeakComponentLink};

#[derive(Default, PartialEq, Clone, Debug)]
pub struct MenuLabel {
    pub id: i32,
    pub label_text: String,
    pub expanded: bool,
    pub menu_list: Vec<Rc<RefCell<MenuNode>>>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum MenuNode {
    Item(MenuItem),
    Fold(MenuFold),
}

impl MenuNode {
    pub fn inactive(&mut self) {
        match self {
            MenuNode::Item(i) => i.is_active = false,
            MenuNode::Fold(f) => f.is_active = false,
        }
    }
}

impl Default for MenuNode {
    fn default() -> Self {
        Self::Item(MenuItem::default())
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

#[derive(Default, PartialEq, Clone, Debug)]
pub struct MenuItem {
    pub id: i32,
    pub is_active: bool,
    pub item_text: String,
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
    pub labels: Rc<Vec<Rc<RefCell<MenuLabel>>>>,
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
            .labels
            .iter()
            .map(|label| {
                html! {
                    <Label label=label active_link=link/>
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

pub struct Label {
    pub props: LabelProps,
    pub link: ComponentLink<Self>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct LabelProps {
    #[prop_or_default]
    pub label: Rc<RefCell<MenuLabel>>,
    #[prop_or_default]
    pub active_link: WeakComponentLink<Node>,
}

impl Component for Label {
    type Message = Msg;
    type Properties = LabelProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                let mut label = self.props.label.borrow_mut();
                label.expanded = !label.expanded;
                self.props
                    .active_link
                    .borrow_mut()
                    .as_ref()
                    .and_then(|node| Some(node.send_message(Msg::Inactive)));
                self.props.active_link.replace(None);
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        neq_assign(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let label = self.props.label.borrow();
        let link = &self.props.active_link.clone();
        let views = if label.expanded {
            let views = label
                .menu_list
                .iter()
                .map(|node| {
                    html! {
                        <Node node=node active_link=link/>
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

        let click_cb = self.link.callback(|_| Msg::Clicked);

        html! {
        <>
        <p class="menu-label" onclick=click_cb>{&*label.label_text}</p>
            {views}
        </>
        }
    }
}

pub struct Node {
    pub props: NodeProps,
    pub link: WeakComponentLink<Self>,
}

#[derive(Properties, PartialEq, Clone)]
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
            Msg::Clicked => {
                match &mut *self.props.node.borrow_mut() {
                    MenuNode::Item(item) => {
                        let active_link = &mut *self.props.active_link.borrow_mut();
                        if !item.is_active {
                            if active_link.is_some() {
                                let node = active_link.as_ref().unwrap();
                                node.send_message(Msg::Inactive);
                            }
                            *active_link = self.link.borrow().clone();
                            item.is_active = true;
                            ConsoleService::info(&*format!("第一次点"));
                        } else {
                            ConsoleService::info(&*format!("重复点"));
                            return false;
                        }
                    }
                    MenuNode::Fold(fold) => {
                        self.props
                            .active_link
                            .borrow_mut()
                            .as_ref()
                            .and_then(|node| Some(node.send_message(Msg::Inactive)));
                        self.props.active_link.replace(None);
                        fold.expanded = !fold.expanded
                    }
                }
                true
            }
            Msg::Inactive => {
                self.props.node.borrow_mut().inactive();
                ConsoleService::info(&*format!("恢复: {:?}", &*self.props.node.borrow()));
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
            MenuNode::Item(item) => html! {
                <li>
                    <a class=classes!(item.is_active.then(||"is-active")) onclick=click_cb>
                        {&*item.item_text}
                    </a>
                </li>
            },
            MenuNode::Fold(fold) => {
                let link = &self.props.active_link.clone();
                let views = if fold.expanded {
                    fold.nodes
                        .iter()
                        .map(|node| {
                            html! {
                                <Node node=node active_link=link />
                            }
                        })
                        .collect::<Html>()
                } else {
                    html! {}
                };

                html! {
                    <li>
                        <a class=classes!(fold.is_active.then(||"is-active")) onclick=click_cb>
                            {&*fold.fold_text}
                        </a>
                        <ul>
                            { views }
                        </ul>
                    </li>
                }
            }
        };

        html! { {views} }
    }
}
