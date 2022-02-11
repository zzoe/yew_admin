use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use indextree::{Arena, NodeId};
use yew::{classes, html, html::Scope, Component, Context, Html, MouseEvent};
use yew_router::prelude::{History, RouterScopeExt};

use crate::msg::Msg;
use crate::pages::home::MenuRoute;

#[derive(PartialEq, Clone, Debug)]
pub enum MenuEnum {
    Label(Rc<RefCell<MenuNode>>),
    Fold(Rc<RefCell<MenuNode>>),
    Item(Rc<RefCell<MenuNode>>),
}

impl Default for MenuEnum {
    fn default() -> Self {
        MenuEnum::Label(Rc::new(RefCell::new(MenuNode::default())))
    }
}

impl MenuEnum {
    fn new(menu_type: &str, id: u32, parent_id: u32, text: String, func_name: String) -> Self {
        let menu_node = Rc::new(RefCell::new(MenuNode {
            id,
            parent_id,
            text,
            expanded: true,
            active: false,
            func_name,
        }));

        match menu_type.to_lowercase().as_str() {
            "label" => MenuEnum::Label(menu_node),
            "fold" => MenuEnum::Fold(menu_node),
            _ => MenuEnum::Item(menu_node),
        }
    }

    fn get_node(&self) -> Rc<RefCell<MenuNode>> {
        match self {
            MenuEnum::Label(node) | MenuEnum::Fold(node) | MenuEnum::Item(node) => Rc::clone(node),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct MenuNode {
    pub id: u32,
    pub parent_id: u32,
    pub text: String,
    pub expanded: bool,
    pub active: bool,
    pub func_name: String,
}

pub struct Menu {
    root: NodeId,
    nodes: Arena<MenuEnum>,
    activated: Option<Rc<RefCell<MenuNode>>>,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mock_menu = vec![
            ("Label", 1_u32, 0_u32, "Label_1", ""),
            ("Label", 4, 0, "Administration", ""),
            ("Item", 2, 1, "Dashboard", "fn1"),
            ("Item", 3, 1, "Customers", "fn2"),
            ("Item", 5, 4, "Products", "fn3"),
            ("Item", 6, 4, "Reports", "fn4"),
            ("Item", 7, 4, "Settings", "fn5"),
            ("Item", 8, 4, "Something", "fn6"),
            ("Fold", 9, 4, "Fold_1", ""),
            ("Item", 10, 9, "Dashboard", "fn7"),
            ("Item", 11, 9, "Customers", "fn8"),
            ("Item", 12, 9, "Orders", "fn9"),
            ("Item", 13, 9, "Products", "fn10"),
            ("Item", 14, 9, "Reports", "fn11"),
            ("Item", 15, 9, "Settings", "fn12"),
            ("Item", 16, 9, "Something", "fn13"),
            ("Fold", 17, 9, "Fold_2", ""),
            ("Item", 18, 17, "Dashboard", "fn14"),
            ("Item", 19, 17, "Customers", "fn15"),
            ("Item", 20, 17, "Orders", "fn16"),
            ("Item", 21, 17, "Products", "fn17"),
            ("Item", 22, 17, "Reports", "fn18"),
            ("Item", 23, 17, "Settings", "fn19"),
        ];

        let mut nodes = Arena::new();
        let mut node_map = HashMap::new();
        let root = nodes.new_node(MenuEnum::default());
        node_map.insert(0, root);
        mock_menu
            .iter()
            .for_each(|(menu_type, id, parent_id, text, func_name)| {
                node_map.insert(
                    *id,
                    nodes.new_node(MenuEnum::new(
                        menu_type,
                        *id,
                        *parent_id,
                        text.to_string(),
                        func_name.to_string(),
                    )),
                );
            });

        mock_menu.iter().for_each(|(_, id, parent_id, _, _)| {
            if let Some(parent) = node_map.get(parent_id) {
                if let Some(child) = node_map.get(id) {
                    parent.append(*child, &mut nodes);
                }
            }
        });

        Self {
            root,
            nodes,
            activated: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MenuClicked(node) => {
                if let Some(current) = &mut self.activated {
                    if !Rc::ptr_eq(current, &node) {
                        current.borrow_mut().active = false;
                    }
                }

                let mut menu_node = node.borrow_mut();
                menu_node.active = !menu_node.active;
                menu_node.expanded = !menu_node.expanded;
                self.activated = Some(Rc::clone(&node));

                if !menu_node.func_name.is_empty() {
                    ctx.link()
                        .history()
                        .unwrap()
                        .push(MenuRoute::from_str(&*menu_node.func_name));
                }

                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <aside class="menu">
                {for self.root.children(&self.nodes).map(|child| self.nodes.view(child, ctx.link()))}
            </aside>
        };
    }
}

trait MenuView {
    fn view(&self, node_id: NodeId, link: &Scope<Menu>) -> Html;
}

impl MenuView for Arena<MenuEnum> {
    fn view(&self, node_id: NodeId, link: &Scope<Menu>) -> Html {
        let arena_node_opt = self.get(node_id);
        if arena_node_opt.is_none() {
            return html! {};
        }

        let arena_node = arena_node_opt.unwrap();
        let menu_enum = arena_node.get();
        let current = menu_enum.get_node();
        let node_ref = Rc::clone(&current);
        let onclick = link.callback(move |_: MouseEvent| Msg::MenuClicked(current.clone()));

        let node = node_ref.borrow();
        let is_active = node.active.then(|| "is-active");

        match menu_enum {
            MenuEnum::Label(_) => html! {
                <>
                <div class="menu-label" onclick={onclick}> {&*node.text} </div>
                if node.expanded {
                     <ul class="menu-list">
                        { for node_id.children(self).map(|child| self.view(child, link)) }
                    </ul>
                }
                </>
            },
            MenuEnum::Fold(_) => html! {
                <li>
                    <a onclick={onclick}>
                        {&*node.text}
                    </a>
                    <ul>
                    if node.expanded {
                        { for node_id.children(self).map(|child| self.view(child, link)) }
                    }
                    </ul>
                </li>
            },
            MenuEnum::Item(_) => html! {
                <li>
                    <a class={classes!(is_active)} onclick={onclick}>
                        {&*node.text}
                    </a>
                </li>
            },
        }
    }
}
