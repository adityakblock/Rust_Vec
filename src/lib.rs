
use std::ptr::NonNull;
use std::alloc::{self, Layout};
pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl <T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len:0,
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero sized types");
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).expect("Could not allocate");
            
            //SAFETY: The layout is hardcoded to be 4 * size_of<T> and size_of<T> > 0
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Could Not allocate!");
            //SAFETY: ptr is non-null and we have just allocated enough
            //space. The memory previously at ptr is not read
            unsafe {ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset = self.len.checked_mul(std::mem::size_of::<T>()).expect("Cannot reach memory location");
            assert!(offset < isize::MAX as usize, "Wrapped isize");
            unsafe { self.ptr.as_ptr().add(self.len).write(item) }
            self.len += 1;

        } else {
            let new_capacity = self.capacity.checked_mul(2).expect("Capacity wrapped");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(size % align).expect("Cant allocate");
            unsafe {
                debug_assert!(self.len == self.capacity);
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * self.capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate");
                ptr.as_ptr().add(self.len).write(item);
                self.ptr = ptr;
                self.len += 1;
                self.capacity = new_capacity;
            }
        
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use super::*;
        let mut vec: MyVec<usize>= MyVec::<usize>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}
