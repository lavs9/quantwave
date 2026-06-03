import os
import re

utils_code = """use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct RingBuffer<T> {
    buffer: Vec<T>,
    mask: usize,
    head: usize,
    tail: usize,
    pub count: usize,
}

impl<T: Default + Clone> RingBuffer<T> {
    pub fn new() -> Self {
        Self::with_capacity(16)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let pow2 = if capacity == 0 { 16 } else { capacity.next_power_of_two() };
        Self {
            buffer: vec![T::default(); pow2],
            mask: pow2 - 1,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    #[inline]
    pub fn push_back(&mut self, item: T) {
        self.buffer[self.head & self.mask] = item;
        self.head = self.head.wrapping_add(1);
        self.count += 1;
    }

    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        if self.count == 0 {
            None
        } else {
            let item = self.buffer[self.tail & self.mask].clone();
            self.tail = self.tail.wrapping_add(1);
            self.count -= 1;
            Some(item)
        }
    }

    #[inline]
    pub fn push_front(&mut self, item: T) {
        self.tail = self.tail.wrapping_sub(1);
        self.buffer[self.tail & self.mask] = item;
        self.count += 1;
    }

    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {
        if self.count == 0 {
            None
        } else {
            self.head = self.head.wrapping_sub(1);
            self.count -= 1;
            Some(self.buffer[self.head & self.mask].clone())
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.count
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    #[inline]
    pub fn front(&self) -> Option<&T> {
        if self.count == 0 {
            None
        } else {
            Some(&self.buffer[self.tail & self.mask])
        }
    }

    #[inline]
    pub fn back(&self) -> Option<&T> {
        if self.count == 0 {
            None
        } else {
            Some(&self.buffer[self.head.wrapping_sub(1) & self.mask])
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.count = 0;
    }

    pub fn iter(&self) -> RingBufferIter<'_, T> {
        RingBufferIter {
            rb: self,
            index: 0,
        }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut new_count = 0;
        let mut i = 0;
        while i < self.count {
            let idx = self.tail.wrapping_add(i) & self.mask;
            if f(&self.buffer[idx]) {
                if new_count != i {
                    let dest_idx = self.tail.wrapping_add(new_count) & self.mask;
                    self.buffer.swap(idx, dest_idx);
                }
                new_count += 1;
            }
            i += 1;
        }
        self.count = new_count;
        self.head = self.tail.wrapping_add(new_count);
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.count {
            Some(&self.buffer[self.tail.wrapping_add(index) & self.mask])
        } else {
            None
        }
    }
}

pub struct RingBufferIter<'a, T> {
    rb: &'a RingBuffer<T>,
    index: usize,
}

impl<'a, T> Iterator for RingBufferIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.rb.count {
            let actual_idx = self.rb.tail.wrapping_add(self.index) & self.rb.mask;
            self.index += 1;
            Some(&self.rb.buffer[actual_idx])
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for RingBufferIter<'a, T> {
    fn len(&self) -> usize {
        self.rb.count - self.index
    }
}

impl<T> Index<usize> for RingBuffer<T> {
    type Output = T;
    
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.count, "index out of bounds");
        &self.buffer[self.tail.wrapping_add(index) & self.mask]
    }
}

impl<T> IndexMut<usize> for RingBuffer<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.count, "index out of bounds");
        let mask = self.mask;
        let actual = self.tail.wrapping_add(index) & mask;
        &mut self.buffer[actual]
    }
}
"""

with open("quantwave-core/src/utils.rs", "w") as f:
    f.write(utils_code)

with open("quantwave-core/src/lib.rs", "r") as f:
    lib_code = f.read()

if "pub mod utils;" not in lib_code:
    lib_code = "pub mod utils;\n" + lib_code
    with open("quantwave-core/src/lib.rs", "w") as f:
        f.write(lib_code)

with open("quantwave-core/src/traits.rs", "r") as f:
    traits_code = f.read()

new_trait_method = """    /// Process a batch of inputs eagerly.
    /// This allows implementations to override with vectorized logic,
    /// while falling back to scalar processing by default.
    fn next_batch(&mut self, inputs: &[Input]) -> Vec<Self::Output> 
    where
        Input: Copy,
    {
        let mut outputs = Vec::with_capacity(inputs.len());
        for &input in inputs {
            outputs.push(self.next(input));
        }
        outputs
    }
}"""

traits_code = traits_code.replace("    fn next(&mut self, input: Input) -> Self::Output;\n}", "    fn next(&mut self, input: Input) -> Self::Output;\n\n" + new_trait_method)

with open("quantwave-core/src/traits.rs", "w") as f:
    f.write(traits_code)

for root, _, files in os.walk("quantwave-core/src/"):
    for file in files:
        if file.endswith(".rs"):
            filepath = os.path.join(root, file)
            with open(filepath, "r") as f:
                content = f.read()
            if "use std::collections::VecDeque;" in content:
                content = content.replace("use std::collections::VecDeque;", "use crate::utils::RingBuffer as VecDeque;")
                with open(filepath, "w") as f:
                    f.write(content)
