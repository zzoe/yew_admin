use crate::components::menu::MenuNode;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum Msg {
    BuggerClick,
    MenuClicked(Rc<RefCell<MenuNode>>),
}
