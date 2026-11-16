//! Production [`Clock`]: one generic web-sys timer registry usable as
//! both `Clock<EngineEvent>` and `Clock<StorageEvent>` (FR-2's timers and
//! FR-8's 500 ms save debounce share the abstraction). `schedule` maps to
//! `window.setTimeout` and `cancel` to `clearTimeout`; a fired or
//! cancelled token never fires again, and the callback is released as
//! soon as it runs, so one-shot timers do not accumulate.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::engine::{Clock, TimerToken};

/// One live timer: its JS timeout handle and the keep-alive closure.
struct LiveTimer {
    handle: i32,
    _closure: Closure<dyn FnMut()>,
}

struct Inner {
    next_id: u64,
    /// Live timers by token id.
    live: HashMap<u64, LiveTimer>,
}

/// A [`Clock`] backed by `web_sys` timeouts. The event vocabulary is a
/// type parameter; `on_fire` routes fired events back into the
/// subsystem that scheduled them.
pub struct WebClock<Ev: 'static> {
    inner: Rc<RefCell<Inner>>,
    on_fire: Rc<dyn Fn(Ev)>,
}

impl<Ev: 'static> WebClock<Ev> {
    /// A clock delivering fired events to `on_fire`.
    pub fn new(on_fire: impl Fn(Ev) + 'static) -> Self {
        WebClock {
            inner: Rc::new(RefCell::new(Inner {
                next_id: 0,
                live: HashMap::new(),
            })),
            on_fire: Rc::new(on_fire),
        }
    }
}

impl<Ev: Copy + 'static> Clock<Ev> for WebClock<Ev> {
    fn now_ms(&self) -> u64 {
        js_sys::Date::now() as u64
    }

    fn schedule(&self, delay_ms: u32, event: Ev) -> TimerToken {
        let mut inner = self.inner.borrow_mut();
        inner.next_id += 1;
        let id = inner.next_id;
        let registry = Rc::clone(&self.inner);
        let on_fire = Rc::clone(&self.on_fire);
        // One-shot timer: deregister (and so drop) itself inside its own
        // callback before delivering the event, so a re-entrant
        // `schedule`/`cancel` from the handler sees consistent state.
        let closure = Closure::new(move || {
            if registry.borrow_mut().live.remove(&id).is_some() {
                on_fire(event);
            }
        });
        let function = closure
            .as_ref()
            .dyn_ref::<js_sys::Function>()
            .expect("a Closure wraps a JS function");
        let Some(window) = web_sys::window() else {
            // Not in a browser (native test build): the event can never
            // fire, so release the closure immediately.
            return TimerToken(id);
        };
        let handle = window
            .set_timeout_with_callback_and_timeout_and_arguments(
                function,
                delay_ms as i32,
                &js_sys::Array::new(),
            )
            .expect("setTimeout with a valid callback cannot fail");
        inner.live.insert(
            id,
            LiveTimer {
                handle,
                _closure: closure,
            },
        );
        TimerToken(id)
    }

    fn cancel(&self, token: TimerToken) {
        if let Some(window) = web_sys::window()
            && let Some(timer) = self.inner.borrow_mut().live.remove(&token.0)
        {
            window.clear_timeout_with_handle(timer.handle);
        }
    }
}
