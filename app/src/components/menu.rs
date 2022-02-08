use std::cell::RefCell;
use std::rc::Rc;

use yew::{classes, html, html::Scope, Component, Context, Html, MouseEvent};

#[derive(PartialEq, Clone, Debug)]
enum MenuProp {
    Label(Rc<RefCell<MenuNode>>),
    Fold(Rc<RefCell<MenuNode>>),
    Item(Rc<RefCell<MenuNode>>),
}

impl MenuProp {
    fn new(menu_type: &str, id: u32, parent_id: u32, text: &str) -> Self {
        let text = text.to_string();
        let menu_node = Rc::new(RefCell::new(MenuNode {
            id,
            parent_id,
            text,
            expanded: true,
            active: false,
            children: vec![],
        }));

        match menu_type.to_lowercase().as_str() {
            "label" => MenuProp::Label(menu_node),
            "fold" => MenuProp::Fold(menu_node),
            _ => MenuProp::Item(menu_node),
        }
    }

    fn add(&self, prop: MenuProp) {
        match self {
            MenuProp::Label(node) | MenuProp::Fold(node) | MenuProp::Item(node) => {
                node.borrow_mut().children.push(prop);
            }
        }
    }

    fn get_node(&self) -> Rc<RefCell<MenuNode>> {
        match self {
            MenuProp::Label(node) | MenuProp::Fold(node) | MenuProp::Item(node) => Rc::clone(node),
        }
    }

    fn view(&self, link: &Scope<Menu>) -> Html {
        let current = self.get_node();
        let node = Rc::clone(&current);
        let onclick = link.callback(move |_: MouseEvent| Msg::Clicked(current.clone()));

        let node = node.borrow();
        let is_active = node.active.then(|| "is-active");

        match self {
            MenuProp::Label(_) => html! {
                <>
                <div class="menu-label" onclick={onclick}> {&*node.text} </div>
                if node.expanded {
                     <ul class="menu-list">
                        { node.children.iter().map(|child| child.view(link)).collect::<Html>() }
                    </ul>
                }
                </>
            },
            MenuProp::Fold(_) => html! {
                <li>
                    <a onclick={onclick}>
                        {&*node.text}
                    </a>
                    <ul>
                    if node.expanded {
                        { node.children.iter().map(|child| child.view(link)).collect::<Html>() }
                    }
                    </ul>
                </li>
            },
            MenuProp::Item(_) => html! {
                <li>
                    <a class={classes!(is_active)} onclick={onclick}>
                        {&*node.text}
                    </a>
                </li>
            },
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct MenuNode {
    pub id: u32,
    pub parent_id: u32,
    pub text: String,
    pub expanded: bool,
    pub active: bool,
    children: Vec<MenuProp>,
}

#[derive(Debug)]
pub enum Msg {
    Clicked(Rc<RefCell<MenuNode>>),
}

#[derive(Default)]
pub struct Menu {
    nodes: Vec<MenuProp>,
    current: Option<Rc<RefCell<MenuNode>>>,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let label_1 = MenuProp::new("label", 1, 0, "label_1");
        label_1.add(MenuProp::new("item", 2, 1, "Dashboard"));
        label_1.add(MenuProp::new("item", 3, 1, "Customers"));
        let label_4 = MenuProp::new("label", 4, 0, "Administration");
        label_4.add(MenuProp::new("item", 5, 4, "Team Settings"));
        let fold_6 = MenuProp::new("fold", 6, 4, "Manage Your Team");
        fold_6.add(MenuProp::new("item", 7, 6, "Projects"));
        fold_6.add(MenuProp::new("item", 8, 6, "Members"));
        let fold_9 = MenuProp::new("fold", 9, 6, "Manage Your Company");
        fold_9.add(MenuProp::new("item", 10, 9, "Settings"));
        fold_9.add(MenuProp::new("item", 11, 9, "Custom Fields"));
        fold_9.add(MenuProp::new("item", 12, 9, "Payments"));
        fold_6.add(fold_9);
        label_4.add(fold_6);
        let nodes = vec![label_1, label_4];

        Self {
            nodes,
            ..Default::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::debug!("{:?}", msg);
        match msg {
            Msg::Clicked(node) => {
                if let Some(current) = &mut self.current {
                    if !Rc::ptr_eq(current, &node) {
                        current.borrow_mut().active = false;
                    }
                }

                let mut menu_node = node.borrow_mut();
                menu_node.active = !menu_node.active;
                menu_node.expanded = !menu_node.expanded;
                self.current = Some(Rc::clone(&node));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <aside class="menu">
                {for self.nodes.iter().map(|node| node.view(ctx.link()))}
            </aside>
        };
    }
}
