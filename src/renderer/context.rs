use std::cell::RefCell;
use std::ptr::NonNull;

use crate::renderer::queue::RenderQueue;

thread_local! {
    static ACTIVE_QUEUE: RefCell<Option<NonNull<RenderQueue>>> = const { RefCell::new(None) };
}

pub fn set_active_queue(queue: &mut RenderQueue) {
    ACTIVE_QUEUE.with(|cell| {
        *cell.borrow_mut() = Some(NonNull::from(queue));
    });
}

pub fn clear_active_queue() {
    ACTIVE_QUEUE.with(|cell| {
        *cell.borrow_mut() = None;
    });
}

pub fn with_active_queue<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut RenderQueue) -> R,
{
    ACTIVE_QUEUE.with(|cell| {
        let ptr = *cell.borrow();
        ptr.map(|mut ptr| {
            // SAFETY:
            // a fila ativa é definida apenas durante a fase de draw do frame
            // e limpa logo após o uso.
            let queue = unsafe { ptr.as_mut() };
            f(queue)
        })
    })
}