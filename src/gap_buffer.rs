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
    
}
