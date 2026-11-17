//! Production [`Clock`]: one generic web-sys timer registry usable as
//! both `Clock<EngineEvent>` and `Clock<StorageEvent>` (FR-2's timers and
//! FR-8's 500 ms save debounce share the abstraction). `schedule` maps
//! zero-delay events to `window.requestAnimationFrame` — FR-2:52 pins
//! the immediate path to the "next animation frame" — and positive
//! delays to `window.setTimeout`; `cancel` retracts either handle. A
//! fired or cancelled token never fires again, and the callback is
//! released as soon as it runs, so one-shot timers do not accumulate.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::engine::{Clock, TimerToken};

/// Which browser API backs a scheduled event.
enum TimerKind {
    Timeout,
    AnimationFrame,
}

/// One live timer: its JS handle (a timeout id or an animation-frame
/// request id) and the keep-alive closure.
struct LiveTimer {
    handle: i32,
    kind: TimerKind,
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

/// The browser window, without calling into wasm-bindgen off the web
/// target: its externs are unimplemented stubs that abort natively, and
/// the engine's tests drive the clock there.
fn window() -> Option<web_sys::Window> {
    if cfg!(not(target_arch = "wasm32")) {
        return None;
    }
    web_sys::window()
}

impl<Ev: Copy + 'static> Clock<Ev> for WebClock<Ev> {
    fn now_ms(&self) -> u64 {
        if cfg!(not(target_arch = "wasm32")) {
            // Off the web target every wasm-bindgen extern (performance,
            // Date) is an unimplemented stub; time stands still, which
            // the engine's tests never observe (they use their own fake
            // clock).
            return 0;
        }
        // Monotonic when available: Date::now() can jump under NTP or
        // manual clock changes, which would distort FR-2's 900 ms forced
        // deadline arithmetic.
        let now = web_sys::window()
            .and_then(|window| window.performance())
            .map(|performance| performance.now())
            .unwrap_or_else(js_sys::Date::now);
        now.max(0.0) as u64
    }

    fn schedule(&self, delay_ms: u32, event: Ev) -> TimerToken {
        let mut inner = self.inner.borrow_mut();
        inner.next_id += 1;
        let id = inner.next_id;
        // Resolve the window before building the Closure: both are
        // wasm-bindgen externs, unimplemented off the web target.
        let Some(window) = window() else {
            // Native test build: the event can never fire, so the token
            // is simply never registered.
            return TimerToken(id);
        };
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
        // FR-2:52 — the zero-delay path is the immediate (≤ 2 kB)
        // schedule, pinned to the next animation frame; a 0 ms timeout
        // could preempt paint where a frame callback cannot.
        let (kind, handle) = if delay_ms == 0 {
            let handle = window
                .request_animation_frame(function)
                .expect("requestAnimationFrame with a valid callback cannot fail");
            (TimerKind::AnimationFrame, handle)
        } else {
            let handle = window
                .set_timeout_with_callback_and_timeout_and_arguments(
                    function,
                    delay_ms as i32,
                    &js_sys::Array::new(),
                )
                .expect("setTimeout with a valid callback cannot fail");
            (TimerKind::Timeout, handle)
        };
        inner.live.insert(
            id,
            LiveTimer {
                handle,
                kind,
                _closure: closure,
            },
        );
        TimerToken(id)
    }

    fn cancel(&self, token: TimerToken) {
        if let Some(window) = window()
            && let Some(timer) = self.inner.borrow_mut().live.remove(&token.0)
        {
            match timer.kind {
                TimerKind::Timeout => {
                    window.clear_timeout_with_handle(timer.handle);
                }
                TimerKind::AnimationFrame => {
                    let _ = window.cancel_animation_frame(timer.handle);
                }
            }
        }
    }
}
