use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};

declare_id!("2VcxeD66wzXGxGgrQB2YNXHy5y91HESMeV6vouWonq7P");

#[program]
pub mod puppet_master {
    use super::*;

    pub fn pull_strings(ctx: Context<PullStrings>, bump: u8, data: u64) -> Result<()> {
        let bump = &[bump][..];

        puppet::cpi::set_data(ctx.accounts.set_data_ctx().with_signer(&[&[bump]]), data)
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    /// CHECK: only used as a signing PDA
    pub authority: UncheckedAccount<'info>,
    pub puppet_program: Program<'info, Puppet>,
}

impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.puppet_program.to_account_info();
        let cpi_accounts = SetData {
            puppet: self.puppet.to_account_info(),
            authority: self.authority.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}