use anchor_lang::prelude::*;

// This is the [program id],
// This address is the deployment address
// solana address -k target/deploy/epic_project-keypair.json to check
declare_id!("Fdw4Hsmd11AaKDAsuEHG1FAqTCUGiJ97g7WXVT8hd28h");

// #[program] this syntax is called a macros
// and it's semilar to inheriting a class.
#[program]
/* pub mod -> tell us that this is a Rust module
   which is an easy way to define a collection of
   functions and varibles -- kinda like a class. */
pub mod epic_project {
    
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        // Get a reference to the account.
        /* We do &mut to get a "mutable reference" to base_account. 
           When we do this it actually gives us the power to make changes 
           to base_account.  */
        let base_account = &mut ctx.accounts.base_account;

        // Initialize total_gifs.
        base_account.total_gifs = 0;

        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>) -> Result<()> {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;

        base_account.total_gifs += 1;

        Ok(())
    }
}

/* Here, we'll basically be able to specify 
   different account constraints. */
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    /* All we're doing here is telling Solana how we 
       want to initialize BaseAccount. 
       
       1. init -> will tell Solana to create a new account 
          owned by our current program.
       2. [payer = user] -> tells our program who's paying for 
          the account to be created. In this case, it's the 
          user calling the function.
       3. We then say [space = 9000] -> which will allocate 9000 
          bytes of space for our account. You can change 
          this # if you wanted, but, 9000 bytes is enough 
          for the program we'll be building here! 
    */
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,

    /* This data is passed into the program that proves to the 
       program that the user calling this program actually owns 
       their wallet account. */
    #[account(mut)]
    pub user: Signer<'info>,

    /* SystemProgram is the program that basically runs Solana. 
       It is responsible for a lot of stuff, but one of the main 
       things it does is create accounts on Solana. The 
       SystemProgram is a program the creators of Solana deployed 
       that other programs like ours talk to haha — it has an id 
       of 11111111111111111111111111111111. */
    pub system_program: Program<'info, System>,
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}
