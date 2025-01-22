use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SlitInstruction {
    Initialize = 0,
    Register = 1,
    Swap = 2,
    Create = 3,
    Stake = 4,
    Redeem = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {

}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Register {
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Swap {
    pub side: u8,
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Create {
    pub id: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Stake {
    pub id: [u8; 16],
    pub chips: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Redeem {
    pub id: [u8; 16],
    pub chips: [u8; 8],
    pub fee_chips: [u8; 8],
}

instruction!(SlitInstruction, Initialize);
instruction!(SlitInstruction, Register);
instruction!(SlitInstruction, Swap);
instruction!(SlitInstruction, Create);
instruction!(SlitInstruction, Stake);
instruction!(SlitInstruction, Redeem);
