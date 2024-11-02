use crate::{fundraiser, state::Fundraiser, MIN_AMOUNT_TO_RAISE};
use pinocchio::{
    sysvars::{clock::Clock, rent::Rent},
    ProgramResult,
};

pub fn initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    unsafe {
        assert!(*(data.as_ptr() as *const u64) > MIN_AMOUNT_TO_RAISE);
    }

    let [maker, fundraiser, vault, rent] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // assert!(maker.is_signer());

    assert_eq!(
        rent.key(),
        &five8_const::decode_32_const("SysvarRent111111111111111111111111111111111")
    );
    let rent = unsafe { (*(rent.borrow_data_unchecked().as_ptr() as *const Rent)).clone() };

    let lamports = rent.minimum_balance(Fundraiser::LEN);

    assert_eq!(
        clock.key(),
        &five8_const::decode_32_const("SysvarC1ock11111111111111111111111111111111")
    );
    let clock = unsafe { *(clock.borrow_data_unchecked().as_ptr() as *const Clock) };

    let time = clock.unix_timestamp;
    unsafe {
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key();
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut i64) = *time;

        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(40) as *mut [u8; 42]) =
            *(data.as_ptr() as *const [u8; 42]);
    };

    let bump_binding = unsafe { *(data.as_ptr().add(41) as *const u8) };
    let signer_seed = [Seed::from(maker.key().as_ref())];
    let signer = [Signer::from(&signer_seed)];

    let instruction = pinocchio_system::instructions::CreateAccount {
        from: maker,
        to: fundraiser,
        lamports,
        space: fundraiser.data_len(),
        owner: self::ID,
    }
    .invoke_signed([signer_seed]);

    Ok(())
}
