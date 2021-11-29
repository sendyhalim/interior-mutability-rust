use crate::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::Drop;
use std::ptr::NonNull;

struct RcInner<T> {
  value: T,
  ref_count: Cell<usize>,
}

struct Rc<T> {
  inner: NonNull<RcInner<T>>,

  // Will help dropcheck
  _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
  pub fn new(value: T) -> Rc<T> {
    let heap_value = Box::new(RcInner {
      value,
      ref_count: Cell::new(1),
    });

    // Convert to raw pointer, leave it hanging in the heap, we'll
    // manage the memory by our own
    let inner = Box::into_raw(heap_value);

    return Rc {
      inner: unsafe { NonNull::new_unchecked(inner) },
      _marker: PhantomData,
    };
  }
}

impl<T> Clone for Rc<T> {
  fn clone(&self) -> Rc<T> {
    let inner = unsafe { self.inner.as_ref() };
    inner.ref_count.set(inner.ref_count.get() + 1);

    return Rc {
      inner: self.inner,
      _marker: PhantomData,
    };
  }
}

impl<T> Deref for Rc<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    return &unsafe { self.inner.as_ref() }.value;
  }
}

impl<T> Drop for Rc<T> {
  fn drop(&mut self) {
    let inner = unsafe { self.inner.as_ref() };
    let ref_count = inner.ref_count.get();

    if ref_count == 1 {
      drop(self.inner);

      // Will trigger deallocation
      let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
    } else {
      inner.ref_count.set(ref_count - 1);
    }
  }
}
