use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvalidAmount = 3,
    FeeTooHigh = 4,
    InsufficientUserBalance = 5,
    InsufficientJobEscrow = 6,
    EngineNotSet = 7,
    UnauthorizedCaller = 8,
    Overflow = 9,
}
