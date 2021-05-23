mod gap{
  use std;
  use std::ops::Range;
  /// A GapBuffer<T> is a sequence of elements of type `T` that can insert and
    /// remove elements at any position in constant time. Indexing is also constant
    /// time. However, changing the position at which insertion and removal occur
    /// takes time proportional to the distance the insertion position is being
    /// moved.
    pub struct GapBuffer<T> {
        // Storage for elements. This has the capacity we need, but its length
        // always remains zero. GapBuffer puts its elements and the gap in this
        // `Vec`'s "unused" capacity.
        storage: Vec<T>,

        // Range of uninitialized elements in the middle of `storage`.
        // Elements before and after this range are always initialized.
        gap: Range<usize>
    }
  impl<T> GapBuffer<T> {
        pub fn new() -> GapBuffer<T> {
            GapBuffer { storage: Vec::new(), gap: 0..0 }
        }

        /// Return the number of elements this GapBuffer could hold without
        /// reallocation.
        pub fn capacity(&self) -> usize {
            self.storage.capacity()
        }

        /// Return the number of elements this GapBuffer currently holds.
        pub fn len(&self) -> usize {
            self.capacity() - self.gap.len()
        }

        /// Return the current insertion position.
        pub fn position(&self) -> usize {
            self.gap.start
        }
    /// Return a pointer to the `index`'th element of the underlying storage,
        /// as if the gap were not there.
        ///
        /// Safety: `index` must be less than self.capacity().
        unsafe fn space(&self, index: usize) -> *const T {
            self.storage.as_ptr().offset(index as isize)
        }

        /// Return a mutable pointer to the `index`'th element of the underlying
        /// storage, as if the gap were not there.
        ///
        /// Safety: `index` must be less than self.capacity().
        unsafe fn space_mut(&mut self, index: usize) -> *mut T {
            self.storage.as_mut_ptr().offset(index as isize)
        }
    /// Return the offset in the buffer of the `index`'th element, taking
        /// the gap into account. This does not check whether index is in range,
        /// but it never returns the index of space in the gap.
        fn index_to_raw(&self, index: usize) -> usize {
            if index < self.gap.start {
                index
            } else {
                index + self.gap.len()
            }
        }
      /// Return a reference to the `index`'th element,
        /// or `None` if `index` is out of bounds.
        pub fn get(&self, index: usize) -> Option<&T> {
            let raw = self.index_to_raw(index);
            if raw < self.capacity() {
                unsafe {
                    // We just checked `raw` against self.capacity(),
                    // and index_to_raw skips the gap, so this is safe.
                    Some(&*self.space(raw))
                }
            } else {
                None
            }
        }
        /// Set the current insertion position to `pos`.
        /// If `pos` is out of bounds, panic.
        pub fn set_position(&mut self, pos: usize) {
            if pos > self.len() {
                panic!("index {} out of range for GapBuffer", pos);
            }

            unsafe {
                let gap = self.gap.clone();
                if pos > gap.start {
                    // `pos` falls after the gap. Move the gap right
                    // by shifting elements after the gap to before it.
                    let distance = pos - gap.start;
                    std::ptr::copy(self.space(gap.end),
                                   self.space_mut(gap.start),
                                   distance);
                } else if pos < gap.start {
                    // `pos` falls before the gap. Move the gap left
                    // by shifting elements before the gap to after it.
                    let distance = gap.start - pos;
                    std::ptr::copy(self.space(pos),
                                   self.space_mut(gap.end - distance),
                                   distance);
                }

                self.gap = pos .. pos + gap.len();
            }
        }
        
}
