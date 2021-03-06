//! Weechat Hook module.
//!
//! Weechat hooks are used for many different things, to create commands, to
//! listen to events on a file descriptor, add completions to weechat, etc.
//! This module contains hook creation methods for the `Weechat` object.

#[cfg(feature = "unstable")]
mod signal;

mod bar;
mod commands;
mod completion;
mod fd;
mod timer;

pub use bar::{BarItemCallback, BarItemHandle};
pub use commands::{Command, CommandCallback, CommandRun, CommandSettings};
pub use completion::{Completion, CompletionHook, CompletionPosition};
pub use fd::{FdHook, FdHookCallback, FdHookMode};
#[cfg(feature = "unstable")]
pub use signal::{SignalHook, SignalHookValue};
pub use timer::TimerHook;

use crate::Weechat;
use weechat_sys::{t_hook, t_weechat_plugin};

/// Weechat Hook type. The hook is unhooked automatically when the object is
/// dropped.
pub(crate) struct Hook {
    pub(crate) ptr: *mut t_hook,
    pub(crate) weechat_ptr: *mut t_weechat_plugin,
}

impl Drop for Hook {
    fn drop(&mut self) {
        let weechat = Weechat::from_ptr(self.weechat_ptr);
        let unhook = weechat.get().unhook.unwrap();
        unsafe { unhook(self.ptr) };
    }
}
