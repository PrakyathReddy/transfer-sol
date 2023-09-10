use anchor_lang::prelude::*;

declare_id!("DZe1TcbWUcY2ftLZJ9BXT695VBLYj9deE4N2NkGh25oP");

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
