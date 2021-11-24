use std::cell::UnsafeCell;

pub struct Cell<T> {
  value: UnsafeCell<T>,
}

// Cell is not sharable across threads because it's intended to not implement Sync.
// It's also implied !Sync because UnsafeCell is also !Sync.
impl<T> Cell<T> {
  pub fn new(value: T) -> Cell<T> {
    return Cell {
      value: UnsafeCell::new(value),
    };
  }

  pub fn set(&self, value: T) {
    // SAFETY: We know no one else is concurrently mutating self.value because
    //         Cell is !Sync.
    // SAFETY: We now we're not invalidating any references because we never
    //         given out reference, the `get` method returns a copy of inner value.
    unsafe {
      *self.value.get() = value;
    }
  }

  pub fn get(&self) -> T
  where
    T: Copy,
  {
    // SAFETY: We know no one else is modifying this concurrently only
    //         the current thread is mutating it, remember Cell is !Sync.
    return unsafe { *self.value.get() };
  }
}
