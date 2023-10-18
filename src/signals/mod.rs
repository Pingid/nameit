use std::{cell::Cell, rc::Rc, time::Duration};

use leptos::{leptos_dom::helpers::TimeoutHandle, *};

pub fn debounce_signal<T>(duration: Duration, read: impl Fn() -> T + 'static) -> ReadSignal<T> {
    let (value, set_value) = create_signal(read());
    let timer = Rc::new(Cell::new(None::<TimeoutHandle>));

    create_effect({
        let timer = Rc::clone(&timer);
        move |_| {
            let value = read();
            if let Some(timeout) = timer.take() {
                timeout.clear();
            }
            let handle = set_timeout_with_handle(move || set_value(value), duration);
            if let Ok(handle) = handle {
                timer.set(Some(handle));
            }
        }
    });

    value
}
