use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;

use yew::html::{AnyScope, Scope};
use yew::scheduler::Shared;
use yew::Callback;
use yew::{Component, Context};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Module {
    Home,
    Menu,
    Fn1,
}

#[derive(Clone)]
pub struct AppScope {
    module: Module,
    scope: AnyScope,
}

impl PartialEq for AppScope {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module
    }
}

impl AppScope {
    fn new(module: Module, scope: &Scope<impl Component>) -> Self {
        AppScope {
            module,
            scope: scope.clone().into(),
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct AppContext {
    pub scopes: HashMap<Module, AppScope>,
}

pub(crate) trait ContextExt {
    fn insert_scope(&self, module: Module);
    fn send<DST: Component>(&self, module: Module, msg: DST::Message);
}

impl<COMP: Component> ContextExt for Context<COMP> {
    fn insert_scope(&self, module: Module) {
        if let Some((c, _)) = self.link().context::<Shared<AppContext>>(Callback::noop()) {
            c.borrow_mut()
                .scopes
                .insert(module, AppScope::new(module, self.link()));
        }
    }

    fn send<DST: Component>(&self, module: Module, msg: DST::Message) {
        if let Some((c, _)) = self.link().context::<Shared<AppContext>>(Callback::noop()) {
            if let Some(s) = RefCell::borrow_mut(&c).scopes.get_mut(&module) {
                if TypeId::of::<DST>().eq(s.scope.get_type_id()) {
                    s.scope.clone().downcast::<DST>().send_message(msg);
                } else {
                    log::error!(
                        "消息发送失败：模块{:?}的类型是{:?},不是{:?}",
                        module,
                        s.scope.get_type_id(),
                        TypeId::of::<DST>()
                    );
                }
            }
        }
    }
}
