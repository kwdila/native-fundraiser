pub struct Fundraiser {
    pub maker: Pubkey,
    pub time_started: i64,

    pub mint_to_raise: Pubkey,
    pub amount_to_raise: u64,
    // pub current_amount: u64,
    pub duration: u8,
    pub bump: u8,
}

impl Space for Fundraiser {
    const LEN: usize = 32 + 32 + 8 + 8 + 1 + 1;
}
