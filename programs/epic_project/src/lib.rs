use anchor_lang::prelude::*;

// This is the [program id],
// This address is the deployment address
// solana address -k target/deploy/epic_project-keypair.json to check
declare_id!("CdXzRnzedF2v1j5YU4BiMBR3Zsgc5tXvRms3qkTW5zan");

// #[program] this syntax is called a macros
// and it's semilar to inheriting a class.
#[program]
/* pub mod -> tell us that this is a Rust module
   which is an easy way to define a collection of
   functions and varibles -- kinda like a class. */
pub mod epic_project {
    
    use super::*;

    pub fn start_stuff_off(_ctx: Context<StartStuffOff>) -> Result<()> {
        Ok(())
    }
}

/* Here, we'll basically be able to specify 
   different account constraints. */
#[derive(Accounts)]
pub struct StartStuffOff {}
