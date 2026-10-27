use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EngineError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvalidInterval = 3,
    InvalidBounty = 4,
    InsufficientInitialFunding = 5,
    JobNotFound = 6,
    JobNotActive = 7,
    JobNotPaused = 8,
    UnauthorizedOwner = 9,
    IntervalNotElapsed = 10,
    MaxRunsReached = 11,
    Overflow = 12,
}
