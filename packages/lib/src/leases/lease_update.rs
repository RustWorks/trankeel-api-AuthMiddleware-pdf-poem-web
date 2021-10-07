use super::CreateFurnishedLeaseDetailsInput;
use crate::error::Result;
use crate::files::CreateFileInput;
use crate::AuthId;
use async_graphql::InputObject;
use piteo_core::database::Db;
use piteo_data::Lease;
use piteo_data::LeaseData;
use piteo_data::LeaseId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "LeaseFurnishedUpdateInput")]
pub struct UpdateFurnishedLeaseInput {
    #[graphql(name = "data")]
    details: Option<CreateFurnishedLeaseDetailsInput>,
    file: Option<CreateFileInput>,
    id: LeaseId,
}

// # Operation

pub fn update_furnished_lease(
    db: &impl Db,
    _auth_id: &AuthId,
    input: UpdateFurnishedLeaseInput,
) -> Result<Lease> {
    input.validate()?;

    db.leases().update(input.into())
}

// # Impls

impl From<UpdateFurnishedLeaseInput> for LeaseData {
    fn from(item: UpdateFurnishedLeaseInput) -> Self {
        Self {
            id: item.id,
            details: item.details.map(Into::into),
            account_id: None,
            deposit_amount: None,
            effect_date: None,
            signature_date: None,
            rent_amount: None,
            rent_charges_amount: None,
            type_: None,
            lease_id: None,
            property_id: None,
            expired_at: None,
            renew_date: None,
            duration: None,
        }
    }
}
