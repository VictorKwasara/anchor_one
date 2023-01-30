use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;

pub mod state ;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod addresses {
    use super::*;

   pub fn create_address_info(
    ctx: Context<CreateAddressInfo>,
    name: String,
    house_number: u8,
    street: String ,
    city: String,
   ) -> Result<()> {
     instructions::create::create_address_info(
        ctx,
        name,
        house_number,
        street,
        city,
     )
   }
}


