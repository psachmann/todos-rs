use std::sync::atomic::AtomicI32;
use leptos::*;

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: i32,
    pub text: String,
    pub completed: RwSignal<bool>,
}

static ID_COUNTER: AtomicI32 = AtomicI32::new(0);

impl TodoItem {
    pub fn new(text: String) -> Self {
        let id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        Self { id, text, completed: create_rw_signal(false) }
    }
}
