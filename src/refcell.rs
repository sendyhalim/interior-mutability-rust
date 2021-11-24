use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::cell::Cell;
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

  pub fn borrow(&self) -> Option<Ref<'_, T>> {
    return match self.state.get() {
      ReferenceState::Unshared => {
        self.state.set(ReferenceState::Shared(1));

        return Some(Ref { refcell: &self });
      }
      ReferenceState::Shared(n) => {
        self.state.set(ReferenceState::Shared(n + 1));

        return Some(Ref { refcell: &self });
      }
      ReferenceState::Exclusive => None,
    };
  }

  pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
    return match self.state.get() {
      ReferenceState::Exclusive | ReferenceState::Shared(_) => None,
      ReferenceState::Unshared => Some(RefMut { refcell: &self }),
    };
  }
}

// Shared reference data type container to be
// able to decrement reference counting state when it's dropped.
pub struct Ref<'ref_cell, T> {
  refcell: &'ref_cell RefCell<T>,
}

impl<'ref_cell, T> Drop for Ref<'ref_cell, T> {
  fn drop(&mut self) {
    match self.refcell.state.get() {
      ReferenceState::Exclusive => {
        unreachable!("There's no way it's borrowed when there's already an exclusive reference")
      }
      ReferenceState::Unshared => {
        unreachable!("There's no way it's unshared when Ref of the current cell is dropped")
      }
      ReferenceState::Shared(1) => {
        self.refcell.state.set(ReferenceState::Unshared);
      }
      ReferenceState::Shared(n) => {
        self.refcell.state.set(ReferenceState::Shared(n - 1));
      }
    }
  }
}

impl<'ref_cell, T> Deref for Ref<'ref_cell, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    // SAFETY
    // Dereferencing into a shared reference is safe because a Ref
    // is only created when state is Unshared, so no exclusive reference
    // has been given out.
    return unsafe { &*self.refcell.value.get() };
  }
}

// Exclusive reference data type container to be
// able to decrement reference counting state when it's dropped.
pub struct RefMut<'ref_cell, T> {
  refcell: &'ref_cell RefCell<T>,
}

impl<'ref_cell, T> Drop for RefMut<'ref_cell, T> {
  fn drop(&mut self) {
    match self.refcell.state.get() {
      ReferenceState::Exclusive => {
        self.refcell.state.set(ReferenceState::Unshared);
      }
      ReferenceState::Unshared => {
        unreachable!("There's no way it's unshared when Ref of the current cell is dropped")
      }
      ReferenceState::Shared(_) => {
        unreachable!("There's no way it's shared when we've given out an exclusive reference")
      }
    }
  }
}

impl<'ref_cell, T> DerefMut for Ref<'ref_cell, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    // SAFETY
    // Dereferencing into a shared reference is safe because a Ref
    // is only created when state is Unshared, so no exclusive reference
    // has been given out.
    return unsafe { &mut *self.refcell.value.get() };
  }
}
