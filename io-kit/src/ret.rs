use io_kit_sys::mach_sys::kern_return_t;
pub use io_kit_sys::ret::{
    kIOReturnAborted, kIOReturnBadArgument, kIOReturnBadMedia, kIOReturnBadMessageID,
    kIOReturnBusy, kIOReturnCannotLock, kIOReturnCannotWire, kIOReturnDMAError,
    kIOReturnDeviceError, kIOReturnError, kIOReturnExclusiveAccess, kIOReturnIOError,
    kIOReturnIPCError, kIOReturnInternalError, kIOReturnInvalid, kIOReturnIsoTooNew,
    kIOReturnIsoTooOld, kIOReturnLockedRead, kIOReturnLockedWrite, kIOReturnMessageTooLarge,
    kIOReturnNoBandwidth, kIOReturnNoChannels, kIOReturnNoCompletion, kIOReturnNoDevice,
    kIOReturnNoFrames, kIOReturnNoInterrupt, kIOReturnNoMedia, kIOReturnNoMemory, kIOReturnNoPower,
    kIOReturnNoResources, kIOReturnNoSpace, kIOReturnNotAligned, kIOReturnNotAttached,
    kIOReturnNotFound, kIOReturnNotOpen, kIOReturnNotPermitted, kIOReturnNotPrivileged,
    kIOReturnNotReadable, kIOReturnNotReady, kIOReturnNotResponding, kIOReturnNotWritable,
    kIOReturnOffline, kIOReturnOverrun, kIOReturnPortExists, kIOReturnRLDError, kIOReturnStillOpen,
    kIOReturnSuccess, kIOReturnTimeout, kIOReturnUnderrun, kIOReturnUnformattedMedia,
    kIOReturnUnsupported, kIOReturnUnsupportedMode, kIOReturnVMError,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOReturn {
    Success,
    Error,
    NoMemory,
    NoResources,
    IPCError,
    NoDevice,
    NotPrivileged,
    BadArgument,
    LockedRead,
    LockedWrite,
    ExclusiveAccess,
    BadMessageID,
    Unsupported,
    VMError,
    InternalError,
    IOError,
    CannotLock,
    NotOpen,
    NotReadable,
    NotWritable,
    NotAligned,
    BadMedia,
    StillOpen,
    RLDError,
    DMAError,
    Busy,
    Timeout,
    Offline,
    NotReady,
    NotAttached,
    NoChannels,
    NoSpace,
    PortExists,
    CannotWire,
    NoInterrupt,
    NoFrames,
    MessageTooLarge,
    NotPermitted,
    NoPower,
    NoMedia,
    UnformattedMedia,
    UnsupportedMode,
    Underrun,
    Overrun,
    DeviceError,
    NoCompletion,
    Aborted,
    NoBandwidth,
    NotResponding,
    IsoTooOld,
    IsoTooNew,
    NotFound,
    Invalid,
    Unknown(kern_return_t),
}

impl From<kern_return_t> for IOReturn {
    fn from(code: kern_return_t) -> IOReturn {
        match code {
            kIOReturnSuccess => IOReturn::Success,
            kIOReturnError => IOReturn::Error,
            kIOReturnNoMemory => IOReturn::NoMemory,
            kIOReturnNoResources => IOReturn::NoResources,
            kIOReturnIPCError => IOReturn::IPCError,
            kIOReturnNoDevice => IOReturn::NoDevice,
            kIOReturnNotPrivileged => IOReturn::NotPrivileged,
            kIOReturnBadArgument => IOReturn::BadArgument,
            kIOReturnLockedRead => IOReturn::LockedRead,
            kIOReturnLockedWrite => IOReturn::LockedWrite,
            kIOReturnExclusiveAccess => IOReturn::ExclusiveAccess,
            kIOReturnBadMessageID => IOReturn::BadMessageID,
            kIOReturnUnsupported => IOReturn::Unsupported,
            kIOReturnVMError => IOReturn::VMError,
            kIOReturnInternalError => IOReturn::InternalError,
            kIOReturnIOError => IOReturn::IOError,
            kIOReturnCannotLock => IOReturn::CannotLock,
            kIOReturnNotOpen => IOReturn::NotOpen,
            kIOReturnNotReadable => IOReturn::NotReadable,
            kIOReturnNotWritable => IOReturn::NotWritable,
            kIOReturnNotAligned => IOReturn::NotAligned,
            kIOReturnBadMedia => IOReturn::BadMedia,
            kIOReturnStillOpen => IOReturn::StillOpen,
            kIOReturnRLDError => IOReturn::RLDError,
            kIOReturnDMAError => IOReturn::DMAError,
            kIOReturnBusy => IOReturn::Busy,
            kIOReturnTimeout => IOReturn::Timeout,
            kIOReturnOffline => IOReturn::Offline,
            kIOReturnNotReady => IOReturn::NotReady,
            kIOReturnNotAttached => IOReturn::NotAttached,
            kIOReturnNoChannels => IOReturn::NoChannels,
            kIOReturnNoSpace => IOReturn::NoSpace,
            kIOReturnPortExists => IOReturn::PortExists,
            kIOReturnCannotWire => IOReturn::CannotWire,
            kIOReturnNoInterrupt => IOReturn::NoInterrupt,
            kIOReturnNoFrames => IOReturn::NoFrames,
            kIOReturnMessageTooLarge => IOReturn::MessageTooLarge,
            kIOReturnNotPermitted => IOReturn::NotPermitted,
            kIOReturnNoPower => IOReturn::NoPower,
            kIOReturnNoMedia => IOReturn::NoMedia,
            kIOReturnUnformattedMedia => IOReturn::UnformattedMedia,
            kIOReturnUnsupportedMode => IOReturn::UnsupportedMode,
            kIOReturnUnderrun => IOReturn::Underrun,
            kIOReturnOverrun => IOReturn::Overrun,
            kIOReturnDeviceError => IOReturn::DeviceError,
            kIOReturnNoCompletion => IOReturn::NoCompletion,
            kIOReturnAborted => IOReturn::Aborted,
            kIOReturnNoBandwidth => IOReturn::NoBandwidth,
            kIOReturnNotResponding => IOReturn::NotResponding,
            kIOReturnIsoTooOld => IOReturn::IsoTooOld,
            kIOReturnIsoTooNew => IOReturn::IsoTooNew,
            kIOReturnNotFound => IOReturn::NotFound,
            kIOReturnInvalid => IOReturn::Invalid,
            code => IOReturn::Unknown(code),
        }
    }
}