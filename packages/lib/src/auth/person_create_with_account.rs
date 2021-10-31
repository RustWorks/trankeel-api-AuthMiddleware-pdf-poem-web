use crate::error::Result;
use async_graphql::InputObject;
use log::info;
use trankeel_core::billing::BillingProvider;
use trankeel_core::database::Db;
use trankeel_data::Account;
use trankeel_data::AccountData;
use trankeel_data::AccountId;
use trankeel_data::AccountStatus;
use trankeel_data::Address;
use trankeel_data::AuthId;
use trankeel_data::Lender;
use trankeel_data::LenderId;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use validator::Validate;

// # Input

#[derive(Clone, InputObject)]
pub struct AddressInput {
    pub city: String,
    pub country: Option<String>,
    pub line1: String,
    pub line2: Option<String>,
    pub postal_code: String,
}

#[derive(Clone, InputObject, Validate)]
#[graphql(name = "UserWithAccountInput")]
pub struct CreateUserWithAccountInput {
    pub address: Option<AddressInput>,
    pub auth_id: AuthId,
    #[validate(email)]
    pub email: String, // Email,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub skip_create_customer: Option<bool>,
}

// # Operation

pub async fn create_user_with_account(
    db: &impl Db,
    billing_provider: &impl BillingProvider,
    input: CreateUserWithAccountInput,
) -> Result<Person> {
    input.validate()?;

    // Create account.
    let account = db.accounts().create(Account {
        id: AccountId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        plan_id: None,
        status: AccountStatus::Trialing,
        stripe_customer_id: None,
        stripe_subscription_id: None,
        trial_end: None,
    })?;

    // Create user.
    let user = db.persons().create(Person {
        id: PersonId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        auth_id: Some(input.auth_id),
        email: input.email.clone().into(),
        first_name: input.first_name,
        last_name: input.last_name,
        address: input.address.map(Into::into),
        photo_url: None,
        role: PersonRole::User,
        phone_number: input.phone_number.map(Into::into),
        account_id: account.id,
    })?;

    // Create lender.
    let _lender = db.lenders().create(Lender {
        id: LenderId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: account.id,
        individual_id: Some(user.id),
        company_id: None,
    })?;

    if let Some(true) = input.skip_create_customer {
        return Ok(user);
    }

    // Create subscription.
    let subscription = billing_provider
        .create_subscription_with_customer(input.email.into())
        .await?;
    info!(
        "Created subscription {} for account {}",
        subscription.id, account.id
    );

    // Update the local customer data.
    db.accounts().update(AccountData {
        id: account.id,
        stripe_customer_id: Some(subscription.customer_id),
        stripe_subscription_id: Some(subscription.id),
        status: Some(subscription.status),
        trial_end: subscription.trial_end,
        ..Default::default()
    })?;

    Ok(user)
}

// # Utils

impl From<AddressInput> for Address {
    fn from(item: AddressInput) -> Self {
        Self {
            city: item.city,
            country: item.country,
            line1: item.line1,
            line2: item.line2,
            postal_code: item.postal_code,
        }
    }
}
