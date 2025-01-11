use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A min-heap that stores items of type `T` with a cost of type `C`.
///
/// The cost needs to implement `Ord` so that the min-heap can order the items by their lowest
/// cost.
#[derive(Default)]
pub struct MinHeap<C: Ord, T> {
    data: BinaryHeap<MinCost<C, T>>,
}

impl<C: Ord, T> MinHeap<C, T> {
    /// Creates an empty [`MinHeap`].
    #[inline]
    pub const fn new() -> MinHeap<C, T> {
        MinHeap {
            data: BinaryHeap::new(),
        }
    }

    /// Creates an empty [`MinHeap`] with at least the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> MinHeap<C, T> {
        MinHeap {
            data: BinaryHeap::with_capacity(capacity),
        }
    }

    /// Pushes an item onto the heap with the specified cost.
    #[inline]
    pub fn push(&mut self, cost: C, item: T) {
        self.data.push(MinCost { cost, item });
    }

    /// Removes the greatest item from the min-heap and returns the `(cost, item)` pair, or `None`
    /// if it is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<(C, T)> {
        self.data.pop().map(|MinCost { cost, item }| (cost, item))
    }

    /// Returns the greatest item from the min-heap without removing it, or `None` if it is empty.
    #[inline]
    pub fn peek(&self) -> Option<(&C, &T)> {
        self.data.peek().map(|MinCost { cost, item }| (cost, item))
    }

    /// Returns the length of the min-heap.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the min-heap is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// An item of type `T` that has a cost associated with it.
///
/// `MinCost` compares in reverse order by the cost, so that it can be used in `BinaryHeap` as a
/// min-heap to extract the cost-item pair with the least cost.
#[derive(Copy, Clone, Debug)]
struct MinCost<C, T> {
    /// The cost associated with this value.
    cost: C,
    /// The item itself.
    item: T,
}

impl<C: PartialEq, T> PartialEq for MinCost<C, T> {
    fn eq(&self, other: &MinCost<C, T>) -> bool {
        self.cost == other.cost
    }
}

impl<C: PartialEq, T> Eq for MinCost<C, T> {}

impl<C: Ord, T> PartialOrd for MinCost<C, T> {
    fn partial_cmp(&self, other: &MinCost<C, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: Ord, T> Ord for MinCost<C, T> {
    fn cmp(&self, other: &MinCost<C, T>) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
