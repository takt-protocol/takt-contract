use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RegistryError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvalidAmount = 3,
    InsufficientStake = 4,
    KeeperAlreadyRegistered = 5,
    KeeperNotFound = 6,
    KeeperNotActive = 7,
    KeeperNotUnbonding = 8,
    CooldownNotElapsed = 9,
    UnauthorizedEngine = 10,
    Overflow = 11,
}
