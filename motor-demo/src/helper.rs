use crate::{poll::poll_called, sync::SyncCell};
use core::{task::Poll, time::Duration};
use pilot_sys::{futures::future, time::current_time, var::Var, var::VarChange, var::VarProps};

// helper function, might be integrated into pilot_sys::time at some point
pub async fn loop_until<T>(duration: Duration, run: &dyn Fn() -> Option<T>) -> Result<T, ()> {
    let time = current_time() + duration.as_micros() as u64;
    future::poll_fn(|_| {
        if let Some(result) = run() {
            Poll::Ready(Ok(result))
        } else {
            poll_called();
            if current_time() >= time {
                Poll::Ready(Err(()))
            } else {
                Poll::Pending
            }
        }
    })
    .await
}

/// waits until the variable ist set to true then sets the value back to false and confirms
/// that the host read the change to false. Usually used for buttons
/// Attention: This method will block forever if the host never reads the variable
pub async fn host_set(button: &Var<bool>) {
    button.pos().await; //wait until the host sets the variable to 1
    button.host_read().await; //and we wait that the host has read the value we just changed
    button.set(false); //we set the variable back to 0 to signal the
                       //host that we received the value
    button.host_read().await; //and we wait that the host has read the value we just changed
}
