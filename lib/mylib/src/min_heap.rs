use std::collections::BinaryHeap;
use std::cmp::Reverse;
use cargo_snippet::snippet;

#[snippet("min_heap")]
#[snippet(prefix="use std::collections::BinaryHeap;")]
#[snippet(prefix="use std::cmp::Reverse;")]
#[snippet(include="min_heap-impl")]
#[derive(Clone, Debug)]
struct MinHeap<T>{
    heap: BinaryHeap<Reverse<T>>
}

#[snippet("min_heap-impl")]
impl<T: Ord> MinHeap<T>{
    pub fn new()->Self{
        let heap = BinaryHeap::new();
        MinHeap{
            heap
        }
    }

    pub fn push(&mut self, x: T){
        self.heap.push(Reverse(x));
    }

    pub fn pop(&mut self)->Option<T>{
        self.heap.pop().map(|x|x.0)
    }

    pub fn peek(&self)->Option<&T>{
        self.heap.peek().map(|x|&x.0)
    }

    pub fn len(&self) -> usize{
        self.heap.len()
    }

    pub fn is_empty(&self)->bool{
        self.heap.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut heap = MinHeap::new();
        heap.push(3);
        heap.push(4);
        heap.push(1);
        heap.push(2);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), None);
    }
}

// impl<T> From<BinaryHeap<T>> for MinHeap<T>{
//     fn from(heap: BinaryHeap<T>)->MinHeap<T>{

//     }
// }