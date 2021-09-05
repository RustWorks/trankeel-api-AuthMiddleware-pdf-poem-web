use async_graphql::InputObject;
use piteo_data::Amount;
use piteo_data::DateTime;
use piteo_data::LeaseId;

#[derive(InputObject)]
pub struct RentInput {
    amount: Amount,
    charges_amount: Option<Amount>,
    lease_id: LeaseId,
    period_end: Option<DateTime>,
    period_start: Option<DateTime>,
}