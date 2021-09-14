use std::cell::RefCell;
use std::rc::Rc;

use yew::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Menu {
    pub props: MenuProps,
    pub link: ComponentLink<Self>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct MenuProps {
    #[prop_or_default]
    pub labels: Rc<RefCell<Vec<Rc<RefCell<MenuLabel>>>>>,
}

#[derive(Default, PartialEq, Clone)]
pub struct MenuLabel {
    pub label_text: String,
    pub menu_list: Vec<Rc<RefCell<MenuNode>>>,
}

#[derive(PartialEq, Clone)]
pub enum MenuNode {
    Item(MenuItem),
    Fold(MenuFold),
}

impl Default for MenuNode {
    fn default() -> Self {
        Self::Item(MenuItem::default())
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct MenuFold {
    pub is_active: bool,
    pub expanded: bool,
    pub fold_text: String,
    pub nodes: Vec<Rc<RefCell<MenuNode>>>,
}

#[derive(Default, PartialEq, Clone)]
pub struct MenuItem {
    pub is_active: bool,
    pub item_text: String,
}

pub enum Msg {
    MenuItemClicked,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = MenuProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MenuItemClicked => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let labels = self.props.labels.borrow();
        let views = labels
            .iter()
            .map(|label| {
                html! {
                    <Label label=label />
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

#[derive(Properties, PartialEq, Clone)]
pub struct LabelProps {
    #[prop_or_default]
    pub label: Rc<RefCell<MenuLabel>>,
}

pub struct Label {
    pub props: LabelProps,
    pub link: ComponentLink<Self>,
}

impl Component for Label {
    type Message = Msg;
    type Properties = LabelProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MenuItemClicked => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.ne(&props)
    }

    fn view(&self) -> Html {
        let label = self.props.label.borrow();
        let views = label
            .menu_list
            .iter()
            .map(|node| {
                html! {
                    <Node node=node />
                }
            })
            .collect::<Html>();
        html! {
        <>
        <a class="menu-label">{&*label.label_text}</a>
            <ul class="menu-list">
                { views }
            </ul>
        </>
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NodeProps {
    #[prop_or_default]
    pub node: Rc<RefCell<MenuNode>>,
}

pub struct Node {
    pub props: NodeProps,
    pub link: ComponentLink<Self>,
}

impl Component for Node {
    type Message = Msg;
    type Properties = NodeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MenuItemClicked => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.ne(&props)
    }

    fn view(&self) -> Html {
        let node = &*self.props.node.borrow();
        let views = match node {
            MenuNode::Item(item) => html! {
                <li><a class=classes!(item.is_active.then(||"is-active"))>{&*item.item_text}</a></li>
            },
            MenuNode::Fold(fold) => {
                let views = fold
                    .nodes
                    .iter()
                    .map(|node| {
                        html! {
                            <Node node=node />
                        }
                    })
                    .collect::<Html>();
                html! {
                    <li>
                        <a class=classes!(fold.is_active.then(||"is-active"))>{&*fold.fold_text}</a>
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
