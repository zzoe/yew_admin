use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use indextree::{Arena, NodeId};
use yew::{classes, html, html::Scope, Component, Context, Html, MouseEvent};

#[derive(PartialEq, Clone, Debug)]
enum MenuProp {
    Label(Rc<RefCell<MenuNode>>),
    Fold(Rc<RefCell<MenuNode>>),
    Item(Rc<RefCell<MenuNode>>),
}

impl Default for MenuProp {
    fn default() -> Self {
        MenuProp::Label(Rc::new(RefCell::new(MenuNode::default())))
    }
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
        }));

        match menu_type.to_lowercase().as_str() {
            "label" => MenuProp::Label(menu_node),
            "fold" => MenuProp::Fold(menu_node),
            _ => MenuProp::Item(menu_node),
        }
    }

    fn get_node(&self) -> Rc<RefCell<MenuNode>> {
        match self {
            MenuProp::Label(node) | MenuProp::Fold(node) | MenuProp::Item(node) => Rc::clone(node),
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
}

#[derive(Debug)]
pub enum Msg {
    Clicked(Rc<RefCell<MenuNode>>),
}

pub struct Menu {
    root: NodeId,
    nodes: Arena<MenuProp>,
    activated: Option<Rc<RefCell<MenuNode>>>,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mock_menu = vec![
            ("Label", 1_u32, 0_u32, "Label_1"),
            ("Label", 4, 0, "Administration"),
            ("Item", 2, 1, "Dashboard"),
            ("Item", 3, 1, "Customers"),
            ("Item", 5, 4, "Products"),
            ("Item", 6, 4, "Reports"),
            ("Item", 7, 4, "Settings"),
            ("Item", 8, 4, "Something"),
            ("Fold", 9, 4, "Fold_1"),
            ("Item", 10, 9, "Dashboard"),
            ("Item", 11, 9, "Customers"),
            ("Item", 12, 9, "Orders"),
            ("Item", 13, 9, "Products"),
            ("Item", 14, 9, "Reports"),
            ("Item", 15, 9, "Settings"),
            ("Item", 16, 9, "Something"),
            ("Fold", 17, 9, "Fold_2"),
            ("Item", 18, 17, "Dashboard"),
            ("Item", 19, 17, "Customers"),
            ("Item", 20, 17, "Orders"),
            ("Item", 21, 17, "Products"),
            ("Item", 22, 17, "Reports"),
            ("Item", 23, 17, "Settings"),
        ];

        let mut nodes = Arena::new();
        let mut node_map = HashMap::new();
        let root = nodes.new_node(MenuProp::default());
        node_map.insert(0, root);
        mock_menu
            .iter()
            .for_each(|(menu_type, id, parent_id, text)| {
                node_map.insert(
                    *id,
                    nodes.new_node(MenuProp::new(menu_type, *id, *parent_id, text)),
                );
            });

        mock_menu.iter().for_each(|(_, id, parent_id, _)| {
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(node) => {
                if let Some(current) = &mut self.activated {
                    if !Rc::ptr_eq(current, &node) {
                        current.borrow_mut().active = false;
                    }
                }

                let mut menu_node = node.borrow_mut();
                menu_node.active = !menu_node.active;
                menu_node.expanded = !menu_node.expanded;
                self.activated = Some(Rc::clone(&node));
                true
            }
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

impl MenuView for Arena<MenuProp> {
    fn view(&self, node_id: NodeId, link: &Scope<Menu>) -> Html {
        let arena_node_opt = self.get(node_id);
        if arena_node_opt.is_none() {
            return html! {};
        }

        let arena_node = arena_node_opt.unwrap();
        let menu_prop = arena_node.get();
        let current = menu_prop.get_node();
        let node_ref = Rc::clone(&current);
        let onclick = link.callback(move |_: MouseEvent| Msg::Clicked(current.clone()));

        let node = node_ref.borrow();
        let is_active = node.active.then(|| "is-active");

        match menu_prop {
            MenuProp::Label(_) => html! {
                <>
                <div class="menu-label" onclick={onclick}> {&*node.text} </div>
                if node.expanded {
                     <ul class="menu-list">
                        { for node_id.children(self).map(|child| self.view(child, link)) }
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
                        { for node_id.children(self).map(|child| self.view(child, link)) }
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
