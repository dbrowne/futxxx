use anchor_lang::prelude::*;
declare_id!("Bs7GGMzNW9nhrZrhxyaLW1AQiaQX6kk1CTqfvj1RkRvS");
#[program]
pub mod oracle {
    use super::*;

    pub fn create_node_network(ctx: Context<CreateNodeNetwork>, collateral: u64) -> Result<()> {
        ctx.accounts.node_network.collateral_requirement = collateral;
        ctx.accounts.node_network.oracle_count = 0;
        Ok(())
    }

    pub fn create_oracle(ctx: Context<CreateOracle>) -> Result<()> {
        ctx.accounts.oracle.is_resolved = false;
        ctx.accounts.oracle.resolution_bit = false;
        ctx.accounts.oracle.phase = Phase::Commit;
        Ok(())
    }

    pub fn commit(ctx: Context<Commit>, commitment: [u8; 32]) -> Result<()> {
        require!(ctx.accounts.oracle.phase == Phase::Commit, Error::WrongPhase);
        ctx.accounts.commitment.owner = ctx.accounts.signer.key();
        ctx.accounts.commitment.commitment_hash = commitment;
        ctx.accounts.commitment.revealed = false;
        ctx.accounts.commitment.bit = None;
        Ok(())
    }

    pub fn reveal(ctx: Context<Reveal>, bit: bool, salt: String) -> Result<()> {
        require!(ctx.accounts.oracle.phase == Phase::Reveal, Error::AlreadyRevealed);
        require!(!ctx.accounts.commitment.revealed, Error::AlreadyRevealed);

        let input = format!("{}:{}", bit, salt);
        let hash = anchor_lang::solana_program::keccak::hash(input.as_bytes());
        require!(hash.0 == ctx.accounts.commitment.commitment_hash, Error::InvalidReveal);

        ctx.accounts.commitment.revealed = true;
        ctx.accounts.commitment.bit = Some(bit);
        Ok(())
    }

    pub fn advance_phase(ctx: Context<AdvancePhase>) -> Result<()> {
        match ctx.accounts.oracle.phase {
            Phase::Commit => ctx.accounts.oracle.phase = Phase::Reveal,
            Phase::Reveal => ctx.accounts.oracle.phase = Phase::Resolved,
            _ => return Err(Error::AlreadyResolved.into()),
        }
        Ok(())
    }

    pub fn resolve_oracle(ctx: Context<ResolveOracle>) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        require!(!oracle.is_resolved, Error::AlreadyResolved);

        let mut yes_votes = 0;
        let mut total = 0;

        for acc_info in ctx.remaining_accounts.iter() {
            let cmt: Account<Commitment> = Account::try_from(acc_info)?;
            if cmt.revealed {
                if let Some(true) = cmt.bit {
                    yes_votes += 1;
                }
                total += 1;
            }
        }

        require!(total > 0, Error::NoValidReveals);

        oracle.resolution_bit = yes_votes * 2 >= total;
        oracle.is_resolved = true;
        Ok(())
    }
}

#[account]
pub struct NodeNetwork {
    pub collateral_requirement: u64,
    pub oracle_count: u32,
}

#[account]
pub struct Oracle {
    pub is_resolved: bool,
    pub resolution_bit: bool,
    pub phase: Phase,
}

#[account]
pub struct Commitment {
    pub owner: Pubkey,
    pub commitment_hash: [u8; 32],
    pub revealed: bool,
    pub bit: Option<bool>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Phase {
    Commit,
    Reveal,
    Resolved,
}

#[error_code]
pub enum Error {
    #[msg("Already revealed.")]
    AlreadyRevealed,
    #[msg("Invalid bit and salt.")]
    InvalidReveal,
    #[msg("Wrong phase.")]
    WrongPhase,
    #[msg("Oracle already resolved.")]
    AlreadyResolved,
    #[msg("No valid reveals submitted.")]
    NoValidReveals,
}

#[derive(Accounts)]
pub struct CreateNodeNetwork<'info> {
    #[account(init, payer = signer, space = 8 + 40)]
    pub node_network: Account<'info, NodeNetwork>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateOracle<'info> {
    #[account(init, payer = signer, space = 8 + 40)]
    pub oracle: Account<'info, Oracle>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Commit<'info> {
    #[account(mut)]
    pub oracle: Account<'info, Oracle>,
    #[account(init, payer = signer, space = 8 + 72)]
    pub commitment: Account<'info, Commitment>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Reveal<'info> {
    #[account(mut)]
    pub oracle: Account<'info, Oracle>,
    #[account(mut)]
    pub commitment: Account<'info, Commitment>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct AdvancePhase<'info> {
    #[account(mut)]
    pub oracle: Account<'info, Oracle>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResolveOracle<'info> {
    #[account(mut)]
    pub oracle: Account<'info, Oracle>,
    #[account(mut)]
    pub signer: Signer<'info>,
}