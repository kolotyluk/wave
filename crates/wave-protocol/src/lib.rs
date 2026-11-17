//! The Wave protocol.
//!
//! This crate is the working definition of what servers, doors and robots say
//! to one another. For now the Rust source *is* the protocol. It will not stay
//! that way: Wave must be implementable in any language, so a written
//! specification follows once a second implementation is attempted.
//!
//! Until then, three rules keep the move to a specification cheap.
//!
//! 1. **Nothing but the protocol lives here.** No input or output, no storage,
//!    no networking, no user interface. Only the shapes of messages and
//!    operations, and the rules for encoding them.
//! 2. **Encoding is exact.** Every operation is signed, and a signature is over
//!    exact bytes. Two implementations must therefore produce the same bytes
//!    for the same operation. The encoding is specified here, deliberately,
//!    and never left to a library's defaults.
//! 3. **Test vectors are the neutral truth.** The `vectors/` directory at the
//!    root of the repository holds example messages with their expected bytes
//!    and results. They are not Rust. Any implementation, in any language,
//!    must pass them. They are the beginning of the conformance suite.
//!
//! The crate has no dependencies and compiles for every target Wave uses,
//! including WebAssembly for the server and for the browser.

/// The version of the Wave protocol this crate defines.
///
/// Version 0 means "not yet stable": anything may change without notice.
pub const PROTOCOL_VERSION: u32 = 0;

/// What kind of participant is taking part in a Wave.
///
/// People and robots join a Wave through the same interface and are both
/// named, permissioned and attributable. The protocol records which is which,
/// so that nobody mistakes an agent for a person.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticipantKind {
    /// A human being.
    Person,
    /// A software agent acting under authority granted within the Wave.
    Robot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_is_not_yet_stable() {
        assert_eq!(PROTOCOL_VERSION, 0);
    }

    #[test]
    fn people_and_robots_are_distinguishable() {
        assert_ne!(ParticipantKind::Person, ParticipantKind::Robot);
    }
}
