/// types for use in the WASI filesystem
#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};
#[cfg(all(unix, feature = "sys-poll", not(feature="os")))]
use std::convert::TryInto;
use std::{
    collections::VecDeque,
    io::{self, Read, Seek, Write},
    sync::{Arc, Mutex},
    time::Duration,
};
use wasmer_vbus::VirtualBusError;

#[cfg(feature = "host-fs")]
pub use wasmer_vfs::host_fs::{Stderr, Stdin, Stdout};
#[cfg(all(feature = "mem-fs", not(feature = "host-fs")))]
pub use wasmer_vfs::mem_fs::{Stderr, Stdin, Stdout};
#[cfg(all(not(feature = "mem-fs"), not(feature = "host-fs")))]
pub use crate::{
    fs::NullFile as Stderr,
    fs::NullFile as Stdin,
    fs::NullFile as Stdout,
};

use wasmer_vfs::{FsError, VirtualFile};
use wasmer_vnet::NetworkError;

pub fn fs_error_from_wasi_err(err: Errno) -> FsError {
    match err {
        __WASI_EBADF => FsError::InvalidFd,
        __WASI_EEXIST => FsError::AlreadyExists,
        __WASI_EIO => FsError::IOError,
        __WASI_EADDRINUSE => FsError::AddressInUse,
        __WASI_EADDRNOTAVAIL => FsError::AddressNotAvailable,
        __WASI_EPIPE => FsError::BrokenPipe,
        __WASI_ECONNABORTED => FsError::ConnectionAborted,
        __WASI_ECONNREFUSED => FsError::ConnectionRefused,
        __WASI_ECONNRESET => FsError::ConnectionReset,
        __WASI_EINTR => FsError::Interrupted,
        __WASI_EINVAL => FsError::InvalidInput,
        __WASI_ENOTCONN => FsError::NotConnected,
        __WASI_ENODEV => FsError::NoDevice,
        __WASI_ENOENT => FsError::EntryNotFound,
        __WASI_EPERM => FsError::PermissionDenied,
        __WASI_ETIMEDOUT => FsError::TimedOut,
        __WASI_EPROTO => FsError::UnexpectedEof,
        __WASI_EAGAIN => FsError::WouldBlock,
        __WASI_ENOSPC => FsError::WriteZero,
        __WASI_ENOTEMPTY => FsError::DirectoryNotEmpty,
        _ => FsError::UnknownError,
    }
}

pub fn fs_error_into_wasi_err(fs_error: FsError) -> Errno {
    match fs_error {
        FsError::AlreadyExists => __WASI_EEXIST,
        FsError::AddressInUse => __WASI_EADDRINUSE,
        FsError::AddressNotAvailable => __WASI_EADDRNOTAVAIL,
        FsError::BaseNotDirectory => __WASI_ENOTDIR,
        FsError::BrokenPipe => __WASI_EPIPE,
        FsError::ConnectionAborted => __WASI_ECONNABORTED,
        FsError::ConnectionRefused => __WASI_ECONNREFUSED,
        FsError::ConnectionReset => __WASI_ECONNRESET,
        FsError::Interrupted => __WASI_EINTR,
        FsError::InvalidData => __WASI_EIO,
        FsError::InvalidFd => __WASI_EBADF,
        FsError::InvalidInput => __WASI_EINVAL,
        FsError::IOError => __WASI_EIO,
        FsError::NoDevice => __WASI_ENODEV,
        FsError::NotAFile => __WASI_EINVAL,
        FsError::NotConnected => __WASI_ENOTCONN,
        FsError::EntryNotFound => __WASI_ENOENT,
        FsError::PermissionDenied => __WASI_EPERM,
        FsError::TimedOut => __WASI_ETIMEDOUT,
        FsError::UnexpectedEof => __WASI_EPROTO,
        FsError::WouldBlock => __WASI_EAGAIN,
        FsError::WriteZero => __WASI_ENOSPC,
        FsError::DirectoryNotEmpty => __WASI_ENOTEMPTY,
        FsError::Lock | FsError::UnknownError => __WASI_EIO,
    }
}

pub fn net_error_into_wasi_err(net_error: NetworkError) -> Errno {
    match net_error {
        NetworkError::InvalidFd => Errno::Badf,
        NetworkError::AlreadyExists => Errno::Exist,
        NetworkError::Lock => Errno::Io,
        NetworkError::IOError => Errno::Io,
        NetworkError::AddressInUse => Errno::Addrinuse,
        NetworkError::AddressNotAvailable => Errno::Addrnotavail,
        NetworkError::BrokenPipe => Errno::Pipe,
        NetworkError::ConnectionAborted => Errno::Connaborted,
        NetworkError::ConnectionRefused => Errno::Connrefused,
        NetworkError::ConnectionReset => Errno::Connreset,
        NetworkError::Interrupted => Errno::Intr,
        NetworkError::InvalidData => Errno::Io,
        NetworkError::InvalidInput => Errno::Inval,
        NetworkError::NotConnected => Errno::Notconn,
        NetworkError::NoDevice => Errno::Nodev,
        NetworkError::PermissionDenied => Errno::Perm,
        NetworkError::TimedOut => Errno::Timedout,
        NetworkError::UnexpectedEof => Errno::Proto,
        NetworkError::WouldBlock => Errno::Again,
        NetworkError::WriteZero => Errno::Nospc,
        NetworkError::Unsupported => Errno::Notsup,
        NetworkError::UnknownError => Errno::Io,
    }
}

pub fn bus_error_into_wasi_err(bus_error: VirtualBusError) -> __bus_errno_t {
    use VirtualBusError::*;
    match bus_error {
        Serialization => __BUS_ESER,
        Deserialization => __BUS_EDES,
        NotFound => __BUS_EWAPM,
        InvalidWapm => __BUS_EWAPM,
        FetchFailed => __BUS_EFETCH,
        CompileError => __BUS_ECOMPILE,
        InvalidABI => __BUS_EABI,
        Aborted => __BUS_EABORTED,
        BadHandle => __BUS_EBADHANDLE,
        InvalidTopic => __BUS_ETOPIC,
        BadCallback => __BUS_EBADCB,
        Unsupported => __BUS_EUNSUPPORTED,
        BadRequest => __BUS_EBADREQUEST,
        AccessDenied => __BUS_EDENIED,
        InternalError => __BUS_EINTERNAL,
        MemoryAllocationFailed => __BUS_EALLOC,
        InvokeFailed => __BUS_EINVOKE,
        AlreadyConsumed => __BUS_ECONSUMED,
        MemoryAccessViolation => __BUS_EMEMVIOLATION,
        UnknownError => __BUS_EUNKNOWN,
    }
}

pub fn wasi_error_into_bus_err(bus_error: __bus_errno_t) -> VirtualBusError {
    use VirtualBusError::*;
    match bus_error {
        __BUS_ESER => Serialization,
        __BUS_EDES => Deserialization,
        __BUS_EWAPM => InvalidWapm,
        __BUS_EFETCH => FetchFailed,
        __BUS_ECOMPILE => CompileError,
        __BUS_EABI => InvalidABI,
        __BUS_EABORTED => Aborted,
        __BUS_EBADHANDLE => BadHandle,
        __BUS_ETOPIC => InvalidTopic,
        __BUS_EBADCB => BadCallback,
        __BUS_EUNSUPPORTED => Unsupported,
        __BUS_EBADREQUEST => BadRequest,
        __BUS_EDENIED => AccessDenied,
        __BUS_EINTERNAL => InternalError,
        __BUS_EALLOC => MemoryAllocationFailed,
        __BUS_EINVOKE => InvokeFailed,
        __BUS_ECONSUMED => AlreadyConsumed,
        __BUS_EMEMVIOLATION => MemoryAccessViolation,
        __BUS_EUNKNOWN | _ => UnknownError,
    }
}

#[allow(dead_code)]
pub(crate) fn bus_read_rights() -> __wasi_rights_t {
    __WASI_RIGHT_FD_FDSTAT_SET_FLAGS
        | __WASI_RIGHT_FD_FILESTAT_GET
        | __WASI_RIGHT_FD_READ
        | __WASI_RIGHT_POLL_FD_READWRITE
}

#[allow(dead_code)]
pub(crate) fn bus_write_rights() -> __wasi_rights_t {
    __WASI_RIGHT_FD_FDSTAT_SET_FLAGS
        | __WASI_RIGHT_FD_FILESTAT_GET
        | __WASI_RIGHT_FD_WRITE
        | __WASI_RIGHT_POLL_FD_READWRITE
}

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum PollEvent {
    /// Data available to read
    PollIn = 1,
    /// Data available to write (will still block if data is greater than space available unless
    /// the fd is configured to not block)
    PollOut = 2,
    /// Something didn't work. ignored as input
    PollError = 4,
    /// Connection closed. ignored as input
    PollHangUp = 8,
    /// Invalid request. ignored as input
    PollInvalid = 16,
}

impl PollEvent {
    fn from_i16(raw_num: i16) -> Option<PollEvent> {
        Some(match raw_num {
            1 => PollEvent::PollIn,
            2 => PollEvent::PollOut,
            4 => PollEvent::PollError,
            8 => PollEvent::PollHangUp,
            16 => PollEvent::PollInvalid,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PollEventBuilder {
    inner: PollEventSet,
}

pub type PollEventSet = i16;

#[derive(Debug)]
pub struct PollEventIter {
    pes: PollEventSet,
    i: usize,
}

impl Iterator for PollEventIter {
    type Item = PollEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pes == 0 || self.i > 15 {
            None
        } else {
            while self.i < 16 {
                let result = PollEvent::from_i16(self.pes & (1 << self.i));
                self.pes &= !(1 << self.i);
                self.i += 1;
                if let Some(r) = result {
                    return Some(r);
                }
            }
            unreachable!("Internal logic error in PollEventIter");
        }
    }
}

pub fn iterate_poll_events(pes: PollEventSet) -> PollEventIter {
    PollEventIter { pes, i: 0 }
}

#[cfg(all(unix, feature = "sys-poll", not(feature="os")))]
fn poll_event_set_to_platform_poll_events(mut pes: PollEventSet) -> i16 {
    let mut out = 0;
    for i in 0..16 {
        out |= match PollEvent::from_i16(pes & (1 << i)) {
            Some(PollEvent::PollIn) => libc::POLLIN,
            Some(PollEvent::PollOut) => libc::POLLOUT,
            Some(PollEvent::PollError) => libc::POLLERR,
            Some(PollEvent::PollHangUp) => libc::POLLHUP,
            Some(PollEvent::PollInvalid) => libc::POLLNVAL,
            _ => 0,
        };
        pes &= !(1 << i);
    }
    out
}

#[cfg(all(unix, feature = "sys-poll", not(feature="os")))]
fn platform_poll_events_to_pollevent_set(mut num: i16) -> PollEventSet {
    let mut peb = PollEventBuilder::new();
    for i in 0..16 {
        peb = match num & (1 << i) {
            libc::POLLIN => peb.add(PollEvent::PollIn),
            libc::POLLOUT => peb.add(PollEvent::PollOut),
            libc::POLLERR => peb.add(PollEvent::PollError),
            libc::POLLHUP => peb.add(PollEvent::PollHangUp),
            libc::POLLNVAL => peb.add(PollEvent::PollInvalid),
            _ => peb,
        };
        num &= !(1 << i);
    }
    peb.build()
}

#[allow(dead_code)]
impl PollEventBuilder {
    pub fn new() -> PollEventBuilder {
        PollEventBuilder { inner: 0 }
    }

    pub fn add(mut self, event: PollEvent) -> PollEventBuilder {
        self.inner |= event as PollEventSet;
        self
    }

    pub fn build(self) -> PollEventSet {
        self.inner
    }
}

#[cfg(all(unix, feature = "sys-poll", not(feature="os")))]
pub(crate) fn poll(
    selfs: &[&(dyn VirtualFile + Send + Sync + 'static)],
    events: &[PollEventSet],
    seen_events: &mut [PollEventSet],
    timeout: Duration,
) -> Result<u32, FsError> {
    if !(selfs.len() == events.len() && events.len() == seen_events.len()) {
        return Err(FsError::InvalidInput);
    }
    let mut fds = selfs
        .iter()
        .enumerate()
        .filter_map(|(i, s)| s.get_fd().map(|rfd| (i, rfd)))
        .map(|(i, host_fd)| libc::pollfd {
            fd: host_fd.try_into().unwrap(),
            events: poll_event_set_to_platform_poll_events(events[i]),
            revents: 0,
        })
        .collect::<Vec<_>>();
    let result = unsafe {
        libc::poll(
            fds.as_mut_ptr(),
            selfs.len() as _,
            timeout.as_millis() as i32,
        )
    };

    if result < 0 {
        // TODO: check errno and return value
        return Err(FsError::IOError);
    }
    // convert result and write back values
    for (i, fd) in fds.into_iter().enumerate() {
        seen_events[i] = platform_poll_events_to_pollevent_set(fd.revents);
    }
    // unwrap is safe because we check for negative values above
    Ok(result.try_into().unwrap())
}

#[cfg(any(not(unix), not(feature = "sys-poll"), feature="os"))]
pub(crate) fn poll(
    files: &[super::InodeValFilePollGuard],
    events: &[PollEventSet],
    seen_events: &mut [PollEventSet],
    timeout: Duration,
) -> Result<u32, FsError> {

    if !(files.len() == events.len() && events.len() == seen_events.len()) {
        tracing::debug!("the slice length of 'files', 'events' and 'seen_events' must be the same (files={}, events={}, seen_events={})", files.len(), events.len(), seen_events.len());
        return Err(FsError::InvalidInput);
    }

    let mut ret = 0;
    for (n, file) in files.iter().enumerate() {
        let mut builder = PollEventBuilder::new();

        let mut can_read = None;
        let mut can_write = None;
        let mut is_closed = None;

        /*
        tracing::debug!(
            "poll_evt can_read={} can_write={} is_closed={}",
            can_read,
            can_write,
            is_closed
        );
        */

        for event in iterate_poll_events(events[n]) {
            match event {
                PollEvent::PollIn => {
                    if can_read.is_none() {
                        can_read = Some(
                            file.bytes_available_read()?
                                .map(|s| s > 0)
                                .unwrap_or(false));
                    }
                    if can_read.unwrap_or_default() {
                        tracing::debug!("poll_evt can_read=true file={:?}", file);
                        builder = builder.add(PollEvent::PollIn);
                    }
                }
                PollEvent::PollOut => {
                    if can_write.is_none() {
                        can_write = Some(file
                            .bytes_available_write()?
                            .map(|s| s > 0)
                            .unwrap_or(false));
                    }
                    if can_write.unwrap_or_default() {
                        tracing::debug!("poll_evt can_write=true file={:?}", file);
                        builder = builder.add(PollEvent::PollOut);
                    }
                }
                PollEvent::PollHangUp |
                PollEvent::PollInvalid |
                PollEvent::PollError => {
                    if is_closed.is_none() {
                        is_closed = Some(file.is_open() == false);
                    }
                    if is_closed.unwrap_or_default() {
                        tracing::debug!("poll_evt is_closed=true file={:?}", file);
                        builder = builder.add(event);
                    }
                }
            }
        }
        let revents = builder.build();
        if revents != 0 {
            ret += 1;
        }
        seen_events[n] = revents;
    }

    if ret == 0 && timeout > Duration::ZERO {
        return Err(FsError::WouldBlock);
    }

    Ok(ret)
}

pub trait WasiPath {}

#[deprecated(
    since = "3.0.0-beta.2",
    note = "Moved to `wasmer_wasi::pipe::WasiBidirectionalSharedPipePair`, `Pipe` is only a transitional reexport"
)]
pub use crate::state::WasiBidirectionalSharedPipePair as Pipe;

/*
TODO: Think about using this
trait WasiFdBacking: std::fmt::Debug {
    fn get_stat(&self) -> &Filestat;
    fn get_stat_mut(&mut self) -> &mut Filestat;
    fn is_preopened(&self) -> bool;
    fn get_name(&self) -> &str;
}
*/
