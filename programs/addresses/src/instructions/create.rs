use anchor_lang::prelude::*;
// solana_program::system_program ;
use crate::state::AddressInfo ;

pub fn create_address_info(
   ctx: Context<CreateAddressInfo>,
   name: String,
   house_number: u8 ,
   street: String,
   city: String,
) -> Result<()>{

  let address_info = AddressInfo::new(
    name,
    house_number,
    street,
    city,
  );

  // Get the length of the data
  let account_span = (address_info.try_to_vec()?).len() ; //what does that try_to_vec do 

  //Get the minimum balance for rent exemption

  let lamports_required = (Rent::get()?).minimum_balance(account_span);

  //making a cpi to the system_program so that you can create a new account to store the address 
 system_program::create_account( //where does this come from, how to fix this 
    CpiContext::new(
      ctx.accounts.system_program.to_account_info(),
      system_program::CreateAccount { //where to get this system_program value 
        from: ctx.accounts.payer.to_account_info(),
        to: ctx.accounts.address_info.to_account_info(),
      },
    ),
    lamports_required, //amount for rent exempt account 
    account_span as u64, //space required by the account 
    &ctx.accounts.system_program.key(), //pass in a reference to the accounts.system_program since we dont want to consume it  
  )?;

  
  let address_info_account = &mut ctx.accounts.address_info ;
  
  address_info_account.set_inner(address_info);
  Ok(())
}

#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
  #[account(mut)]
  address_info: Account<'info, AddressInfo>,
  #[account(mut)]
  payer: Signer<'info>,
  system_program: Program<'info, System>,
}

