use anchor_lang::prelude::*;
use program_b::program::ProgramB; //llamamos al programa b

declare_id!("9meeySgNgNmG29VYBNjmdqfEaAMpgYb63DB8SBe8v6vX");

#[program]
pub mod program_a {
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from program A");

        let pda_address = ctx.accounts.pda_account.key;
        let signer_address = ctx.accounts.signer.key;
        let bump = ctx.bumps.pda_account;

        let instruction =
            &system_instruction::transfer(&pda_address, &signer_address, 1_000_000_000);

        let account_infos = [
            ctx.accounts.pda_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ]; //creamos un array de cuentas para la invocacion cruzada obteniendo la informacion de las cuentas

        let signers_seeds:&[&[&[u8]]] = &[&[b"ackee",signer_address.as_ref(),&[bump]]];

        //creamos una invocacion cruzada de SystemProgram
        //necesitamos de esta cuenta firmada para poder hacer la invocacion cruzada
        //la cuenta firmada es la cuenta de programa a , porque necesitamos autorizar la transaccion de lamports desde la PDA
        invoke_signed(instruction, &account_infos, signers_seeds)?;
//anchor.toml resolution = false

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        mut,
        seeds = [b"ackee",signer.key.as_ref()],
        bump, //bump asi usa una numero canonico para generar la direccion de la cuenta pda empezando por el 255, hasta que salga de la curva y sea valida
        //bump = 5, //bump es un numero aleatorio que se usa para generar la direccion de la cuenta pda
    )]
    pub pda_account: AccountInfo<'info>, //cuenta de programa b

    #[account(mut)]
    pub signer: Signer<'info>, //signer de programa a
    pub system_program: Program<'info, System>, //programa del sistema
    pub program_b: Program<'info, ProgramB>, //definindo en cargo toml. Anchor crea automaticamete el struct de la cuenta de programa b para usar la funcionalidad de CPI.
                                             /*En esta instruccion decimos que vamos usar las instruction Programa, mas precisamente del ProgramB implementando localmente dentro del espacio de trabajo */
}
