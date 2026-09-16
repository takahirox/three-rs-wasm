//! Typed notifications. Listener tokens support removal during a dispatch;
//! like Three.js, the current dispatch uses a snapshot of its listeners.
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListenerToken(uuid::Uuid, u64);
type Listener<E> = Rc<dyn Fn(&E, &EventDispatcher<E>)>;
struct State<E> {
    id: uuid::Uuid,
    next: u64,
    listeners: Vec<(ListenerToken, Listener<E>)>,
}
pub struct EventDispatcher<E> {
    state: Rc<RefCell<State<E>>>,
}
impl<E> Default for EventDispatcher<E> {
    fn default() -> Self {
        Self {
            state: Rc::new(RefCell::new(State {
                id: uuid::Uuid::new_v4(),
                next: 0,
                listeners: Vec::new(),
            })),
        }
    }
}
impl<E> Clone for EventDispatcher<E> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}
impl<E> EventDispatcher<E> {
    pub fn add_event_listener(&self, listener: impl Fn(&E, &Self) + 'static) -> ListenerToken {
        let mut state = self.state.borrow_mut();
        let token = ListenerToken(state.id, state.next);
        state.next = state.next.checked_add(1).expect("listener ids exhausted");
        state.listeners.push((token, Rc::new(listener)));
        token
    }
    pub fn has_event_listener(&self, token: ListenerToken) -> bool {
        self.state
            .borrow()
            .listeners
            .iter()
            .any(|(t, _)| *t == token)
    }
    pub fn remove_event_listener(&self, token: ListenerToken) {
        self.state
            .borrow_mut()
            .listeners
            .retain(|(t, _)| *t != token);
    }
    pub fn dispatch_event(&self, event: &E) {
        let snapshot = self.state.borrow().listeners.clone();
        for (_, listener) in snapshot {
            listener(event, self);
        }
    }
}
