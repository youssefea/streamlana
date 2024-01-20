use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
};
use spl_token::instruction::transfer;

mod instructions;
use instructions::StreamingInstructions;
use borsh::BorshDeserialize; // Ensure BorshDeserialize is in scope

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StreamingInstructions::try_from_slice(instruction_data)?; // Deserialize instruction data

    match instruction {
        LockArgs { amount } => {
            msg!("Locking {} tokens", amount);

            let account_info_iter = &mut accounts.iter();
            let source_account = next_account_info(account_info_iter)?;
            let destination_account = next_account_info(account_info_iter)?;
            let token_program = next_account_info(account_info_iter)?;
    
            let transfer_instruction = transfer(
                token_program.key,
                source_account.key,
                destination_account.key,
                &source_account.owner,
                &[],
                amount,
            )?;
    
            msg!("Transfer instruction created");
    
            solana_program::program::invoke(
                &transfer_instruction,
                &[
                    source_account.clone(),
                    destination_account.clone(),
                    token_program.clone(),
                ],
            )?;
        }
        UnLockArgs { amount } => {
            msg!("Locking {} tokens", amount);
        }
        StartStreamArgs { amount } => {
            msg!("Locking {} tokens", amount);
        }
        UpdateStreamArgs { amount } => {
            msg!("Locking {} tokens", amount);
        }
    }

    if let LockArgs { amount } = instruction {
        msg!("Locking {} tokens", amount);

      
    }

    Ok(())
}
