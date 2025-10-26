use pinocchio::program_error::ProgramError;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum RpgError {
    InvalidInstruction = 0,
    InvalidAccountOwner = 1,
    InvalidPDA = 2,
    AccountAlreadyInitialized = 3,
    AccountNotInitialized = 4,
    InsufficientFunds = 5,
    InvalidHeroLevel = 6,
    InvalidItemType = 7,
    ItemSlotOccupied = 8,
    ItemSlotEmpty = 9,
    BattleNotActive = 10,
    BattleAlreadyActive = 11,
    InvalidBattleState = 12,
    MaxTurnsReached = 13,
    NotYourTurn = 14,
    InvalidAction = 15,
    HeroDefeated = 16,
    RollNotReady = 17,
    RollAlreadyFulfilled = 18,
    InvalidRNGSeed = 19,
    Unauthorized = 20,
    ArithmeticOverflow = 21,
    InvalidAttributeValue = 22,
    MaxLevelReached = 23,
    InsufficientXP = 24,
}

impl From<RpgError> for ProgramError {
    fn from(e: RpgError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl RpgError {
    pub fn log(&self) {
        pinocchio::msg!("RPG Error: {:?}", self);
    }
}
