use anchor_lang::prelude::*;

declare_id!("854pc6XP1JeoAnRp6Lg6QXE7BuEKVkGebN3avMoD7Xcn");

#[program]
pub mod epic_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
