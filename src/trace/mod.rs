#[cfg(unix)]
mod linux;
#[cfg(unix)]
pub(crate) use linux::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub(crate) use windows::*;
