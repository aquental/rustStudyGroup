use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use mpl_token_metadata::types::DataV2;

use mpl_token_metadata::accounts::{MasterEdition, Metadata as MetadataAccount};

declare_id!("BJ4WxMKh29S3tNZcA3SwpYfjJJ32vSqB8db1qpFdWmT8");
#[program]
pub mod metaplex_nft {

    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.associated_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        mint_to(cpi_context, 1)?;

        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );

        let data_v2 = DataV2 {
            name: name,
            symbol: symbol,
            uri: uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );

        create_master_edition_v3(cpi_context, None)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    /// CHECK: The signer field is safe because it is verified elsewhere in the program.
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    #[account(
            init,
            payer = signer,
            mint::decimals = 0,
            mint::authority = signer.key(),
            mint::freeze_authority = signer.key(),
        )]
    pub mint: Box<Account<'info, Mint>>,
    #[account(
            init_if_needed,
            payer = signer,
            associated_token::mint = mint,
            associated_token::authority = signer
        )]
    pub associated_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK - address
    #[account(
            mut,
            address = MetadataAccount::find_pda(&mint.key()).0,
        )]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK - address
    #[account(
            mut,
            address = MasterEdition::find_pda(&mint.key()).0,
        )]
    pub master_edition_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
