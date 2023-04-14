use std::cell::RefCell;
use std::rc::Rc;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, String>;

pub type RcRefCell<T> = Rc<RefCell<T>>;
