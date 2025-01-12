use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};


/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub txt: String,
}

//pub trait Stuff: Sized + BorshSerialize + BorshDeserialize {
//    fn pack(&self, data: &mut [u8]){
//        let encoded = self.try_to_vec().unwrap();
//        data[..encoded.len()].copy_from_slice(&encoded);
//    }
//    fn unpack(src: &[u8]) -> Result<Self, ProgramError>{
//        Self::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)
//    }
//}

//impl Stuff for GreetingAccount {}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();
    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;
    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Account is {:?}", account); 
    // getting stuff from instruction data

    // THIS IS GIVING SIZE ERROR
    let new_greeting = GreetingAccount::try_from_slice(&instruction_data);
    msg!("Greeting was: {:?}, instruction_data_len is: {}", new_greeting, instruction_data.len());
    // Increment and store the number of times the account has been greeted
    //let mut new_greeting: GreetingAccount = GreetingAccount::try_from_slice(&account.data.borrow())?;
    //new_greeting.counter += 1;
    //msg!("Greeting Acc counter: {}", new_greeting.counter);
    let data = &mut &mut account.data.borrow_mut();
    msg!("Data: {:?}", &data);
    msg!("data len: {}", data.len());
    //new_greeting.serialize(data[..])?;
    data[..instruction_data.len()].copy_from_slice(&instruction_data);

    msg!("Greeted {:?} time(s)!", new_greeting);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
