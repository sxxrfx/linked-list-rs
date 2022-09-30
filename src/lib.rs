pub mod benchmarks;
pub mod tests;

pub mod linked_list {
    #[derive(Default, Clone)]
    struct ListNode<T> {
        data: T,
    }
    struct ListHead<T> {
        next: Option<Box<ListHead<T>>>,
        prev: Option<Box<ListHead<T>>>,
    }
}
