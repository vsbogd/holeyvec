# Vector with holes

Vector with holes data structure. Each time an element is removed its index is added into a
list of empty elements. No other values are moved or touched. This allows removing elements in
a constant time.

When new element is pushed it takes the last empty element from the list. It means element is
not necessary added into the end of the vector. If there is no empty elements then element is
added to the end of the vector similarly to [std::vec::Vec].

## Examples

```rust
use holeyvec::HoleyVec;

let mut v = HoleyVec::new();

v.push(1);
v.push(2);
v.push(3);
v.remove(1);
assert_eq!(v.iter().copied().collect::<Vec<i32>>(), vec![1, 3]);

v.push(4);
assert_eq!(v.iter().copied().collect::<Vec<i32>>(), vec![1, 4, 3]);
```
