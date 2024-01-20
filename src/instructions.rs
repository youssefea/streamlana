use borsh::{ BorshDeserialize, BorshSerialize, Debug };

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LockArgs {
    amount: u64,
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UnLockArgs {
    amount: u64,
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct StartStreamArgs {
    amount: u64,
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UpdateStreamArgs {
    amount: u64,
}

pub enum StreamingInstructions {
    Lock(LockArgs),
    UnLock(UnLockArgs),
    StartStream (StartStreamArgs),
    UpdqteStream(UpdateStreamArgs),
}

impl StreamingInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => Self::Lock(LockArgs::try_from_slice(rest)?),
            1 => Self::UnLock(UnLockArgs::try_from_slice(rest)?),
            2 => Self::StartStream(StartStreamArgs::try_from_slice(rest)?),
            3 => Self::UpdqteStream(UpdateStreamArgs::try_from_slice(rest)?),
            _ => return Err(InvalidInstruction.into()),
        })
    }
}