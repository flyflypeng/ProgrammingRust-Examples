use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);

    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.pop(), Some(8));
    assert_eq!(heap.pop(), Some(6));
}
