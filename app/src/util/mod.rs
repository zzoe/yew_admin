use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use yew::html::ImplicitClone;
use yew::{Component, ComponentLink};

pub fn neq_assign<T: PartialEq>(mut_props: &mut T, props: T) -> bool {
    if props.ne(mut_props) {
        *mut_props = props;
        true
    } else {
        false
    }
}

pub struct WeakComponentLink<COMP: Component>(Rc<RefCell<Option<ComponentLink<COMP>>>>);

impl<COMP: Component> Clone for WeakComponentLink<COMP> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<COMP: Component> ImplicitClone for WeakComponentLink<COMP> {}

impl<COMP: Component> Default for WeakComponentLink<COMP> {
    fn default() -> Self {
        Self(Rc::default())
    }
}

impl<COMP: Component> Deref for WeakComponentLink<COMP> {
    type Target = Rc<RefCell<Option<ComponentLink<COMP>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<COMP: Component> PartialEq for WeakComponentLink<COMP> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<COMP: Component> WeakComponentLink<COMP> {
    pub fn new(link: Option<ComponentLink<COMP>>) -> Self {
        WeakComponentLink(Rc::new(RefCell::new(link)))
    }
}
