use crate::candidacies::CreateWarrantInput;
use crate::error::Result;
use crate::AuthId;
use crate::Date;
use crate::Tenant;
use async_graphql::InputObject;
use piteo_core::database::Db;
use piteo_data::PhoneNumber;
use piteo_data::TenantData;
use piteo_data::TenantId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "TenantUpdateInput")]
pub struct UpdateTenantInput {
    pub apl: Option<bool>,
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    #[validate(email)]
    pub email: Option<String>, // Email,
    pub id: TenantId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub is_student: Option<bool>,
    pub warrants: Option<Vec<CreateWarrantInput>>,
}

// # Operation

pub fn update_tenant(db: &impl Db, _auth_id: &AuthId, input: UpdateTenantInput) -> Result<Tenant> {
    input.validate()?;

    db.tenants().update(input.into())
}

// # Impls

impl From<UpdateTenantInput> for TenantData {
    fn from(item: UpdateTenantInput) -> Self {
        Self {
            id: item.id,
            account_id: Default::default(),
            apl: item.apl,
            birthdate: item.birthdate,
            birthplace: item.birthplace,
            email: item.email.map(Into::into),
            first_name: item.first_name,
            last_name: item.last_name,
            note: item.note,
            phone_number: item.phone_number,
            is_student: item.is_student,
            lease_id: None,
            status: None,
        }
    }
}