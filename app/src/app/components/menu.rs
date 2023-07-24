use std::collections::HashMap;
use std::str::FromStr;

use indextree::{Arena, NodeId};
use yew::{classes, html, html::Scope, Component, Context, Html, MouseEvent};
use yew_router::prelude::RouterScopeExt;

use crate::app::components::FnRoute;
use crate::app::context::{ContextExt, Module};
use crate::app::msg::Msg;

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub enum MenuType {
    #[default]
    Label,
    Fold,
    Item,
}

impl FromStr for MenuType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Label" => Ok(MenuType::Label),
            "Fold" => Ok(MenuType::Fold),
            "Item" => Ok(MenuType::Item),
            _ => {
                let e = format!("{} is not a valid MenuType", s);
                log::error!("{}", e);
                Err(e)
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct MenuNode {
    pub id: u32,
    pub parent_id: u32,
    pub menu_type: MenuType,
    pub text: String,
    pub expanded: bool,
    pub active: bool,
    pub func_id: String,
}

impl MenuNode {
    fn new(menu_type: &str, id: u32, parent_id: u32, text: String, func_id: String) -> Self {
        let menu_type = MenuType::from_str(menu_type).unwrap_or_default();
        MenuNode {
            id,
            parent_id,
            menu_type,
            text,
            expanded: true,
            active: false,
            func_id,
        }
    }
}

pub struct Menu {
    nodes: Arena<MenuNode>,
    node_map: HashMap<u32, NodeId>,
    activated: u32,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.insert_scope(Module::Menu);
        let mock_menu = vec![
            ("Label", 1_u32, 0_u32, "Label_1", 0_u32),
            ("Label", 4, 0, "Administration", 0),
            ("Item", 2, 1, "Dashboard1", 1001),
            ("Item", 3, 1, "Customers2", 1002),
            ("Item", 5, 4, "Products1", 1001),
            ("Item", 6, 4, "Reports2", 1002),
            ("Item", 7, 4, "Settings", 5),
            ("Item", 8, 4, "Something", 6),
            ("Fold", 9, 4, "Fold_1", 0),
            ("Item", 10, 9, "Dashboard", 7),
            ("Item", 11, 9, "Customers", 8),
            ("Item", 12, 9, "Orders", 9),
            ("Item", 13, 9, "Products", 10),
            ("Item", 14, 9, "Reports", 11),
            ("Item", 15, 9, "Settings", 12),
            ("Item", 16, 9, "Something", 13),
            ("Fold", 17, 9, "Fold_2", 0),
            ("Item", 18, 17, "Dashboard", 14),
            ("Item", 19, 17, "Customers", 15),
            ("Item", 20, 17, "Orders", 16),
            ("Item", 21, 17, "Products", 17),
            ("Item", 22, 17, "Reports", 18),
            ("Item", 23, 17, "Settings", 19),
        ];

        let mut nodes = Arena::new();
        let mut node_map = HashMap::new();
        let root = nodes.new_node(MenuNode::default());
        node_map.insert(0, root);
        mock_menu
            .iter()
            .for_each(|(menu_type, id, parent_id, text, func_id)| {
                node_map.insert(
                    *id,
                    nodes.new_node(MenuNode::new(
                        menu_type,
                        *id,
                        *parent_id,
                        text.to_string(),
                        format!("Fn{}", *func_id),
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
            nodes,
            node_map,
            activated: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MenuClicked(id) => {
                if id != self.activated {
                    if let Some(node_id) = self.node_map.get(&self.activated) {
                        if let Some(node) = self.nodes.get_mut(*node_id) {
                            node.get_mut().active = false;
                        }
                    }
                }

                let clicked = match self.node_map.get(&id) {
                    Some(node_id) => match self.nodes.get_mut(*node_id) {
                        Some(node) => node.get_mut(),
                        None => return false,
                    },
                    None => return false,
                };

                clicked.active = !clicked.active;
                clicked.expanded = !clicked.expanded;
                self.activated = id;

                if !clicked.func_id.is_empty() {
                    if let Some(navigator) = ctx.link().navigator() {
                        if let Ok(fn_route) = FnRoute::from_str(&clicked.func_id) {
                            navigator.push(&fn_route);
                        }
                    }
                }

                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let root = match self.node_map.get(&0) {
            Some(id) => id,
            None => return html! {},
        };

        html! {
            <aside class="menu">
                {for root.children(&self.nodes).map(|child| self.nodes.view(child, ctx.link()))}
            </aside>
        }
    }
}

trait MenuView {
    fn view(&self, node_id: NodeId, link: &Scope<Menu>) -> Html;
}

impl MenuView for Arena<MenuNode> {
    fn view(&self, node_id: NodeId, link: &Scope<Menu>) -> Html {
        let node = match self.get(node_id) {
            Some(node) => node.get(),
            None => return html! {},
        };

        let id = node.id;
        let onclick = link.callback(move |_: MouseEvent| Msg::MenuClicked(id));
        let is_active = node.active.then_some("is-active");

        match node.menu_type {
            MenuType::Label => html! {
                <>
                <div class="menu-label" onclick={onclick}> {&*node.text} </div>
                if node.expanded {
                     <ul class="menu-list">
                        { for node_id.children(self).map(|child| self.view(child, link)) }
                    </ul>
                }
                </>
            },
            MenuType::Fold => html! {
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
            MenuType::Item => html! {
                <li>
                    <a class={classes!(is_active)} onclick={onclick}>
                        {&*node.text}
                    </a>
                </li>
            },
        }
    }
}
