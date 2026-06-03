import os

with open("quantwave-core/src/utils.rs", "r") as f:
    content = f.read()

# Add serde derive
content = content.replace("#[derive(Debug, Clone)]", "#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]")

# Add IntoIterator for &RingBuffer
into_iter_code = """
impl<'a, T> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = RingBufferIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Default + Clone> From<Vec<T>> for RingBuffer<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut rb = RingBuffer::with_capacity(vec.len());
        for item in vec {
            rb.push_back(item);
        }
        rb
    }
}
"""
content += into_iter_code

with open("quantwave-core/src/utils.rs", "w") as f:
    f.write(content)

# Fix correlation_cycle.rs
cc_path = "quantwave-core/src/indicators/correlation_cycle.rs"
with open(cc_path, "r") as f:
    cc = f.read()
cc = cc.replace("&std::collections::VecDeque<f64>", "&VecDeque<f64>")
with open(cc_path, "w") as f:
    f.write(cc)

# Fix math.rs
math_path = "quantwave-core/src/indicators/math.rs"
with open(math_path, "r") as f:
    math_content = f.read()
math_content = math_content.replace("std::collections::VecDeque", "VecDeque")
if "use crate::utils::RingBuffer as VecDeque;" not in math_content:
    math_content = "use crate::utils::RingBuffer as VecDeque;\n" + math_content
with open(math_path, "w") as f:
    f.write(math_content)

