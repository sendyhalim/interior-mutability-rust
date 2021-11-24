use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Copy, Clone)]
enum ReferenceState {
  Unshared,
  Exclusive,
  Shared(i32),
}

pub struct RefCell<T> {
  value: UnsafeCell<T>,
  state: Cell<ReferenceState>,
}

impl<T> RefCell<T> {
  pub fn new(value: T) -> RefCell<T> {
    return RefCell {
      value: UnsafeCell::new(value),
      state: Cell::new(ReferenceState::Unshared),
    };
  }

  pub fn borrow(&self) -> Option<&T> {
    return match self.state.get() {
      ReferenceState::Unshared => Some(unsafe { &*self.value.get() }),
      ReferenceState::Shared(n) => {
        self.state.set(ReferenceState::Shared(n + 1));

        return Some(unsafe { &*self.value.get() });
      }
      ReferenceState::Exclusive => None,
    };
  }

  pub fn borrow_mut(&self) -> Option<&mut T> {
    return match self.state.get() {
      ReferenceState::Exclusive | ReferenceState::Shared(_) => None,
      ReferenceState::Unshared => Some(unsafe { &mut *self.value.get() }),
    };
  }
}
