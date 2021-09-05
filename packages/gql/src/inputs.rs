use crate::scalars::AuthId;
use crate::scalars::Date;
use crate::scalars::DateTime;
use crate::scalars::Decimal;
use crate::scalars::Email;
use crate::scalars::PhoneNumber;
use async_graphql::ID;
use piteo_core::error::Error;
use piteo_core::FileType;
use piteo_core::ImportSource;
use piteo_core::LeaseFurnishedDuration;
use piteo_core::LeaseRentPeriodicity;
use piteo_core::LeaseRentReferenceIrl;
use piteo_core::PlanCode;
use piteo_core::PropertyBuildPeriodType;
use piteo_core::PropertyBuildingLegalStatus;
use piteo_core::PropertyEnergyClass;
use piteo_core::PropertyGasEmission;
use piteo_core::PropertyHabitationUsageType;
use piteo_core::PropertyRoomType;
use piteo_core::PropertyStatus;
use piteo_core::PropertyUsageType;
use piteo_core::RentChargesRecuperationMode;
use piteo_core::RentPaymentMethod;
use piteo_core::TransactionType;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(async_graphql::InputObject)]
#[graphql(name = "UserWithAccountInput")]
pub struct CreateUserWithAccountInput {
    address: Option<AddressInput>,
    auth_id: AuthId,
    email: Email,
    first_name: String,
    last_name: String,
    skip_create_customer: Option<bool>,
}

impl From<CreateUserWithAccountInput> for piteo_lib::CreateUserWithAccountInput {
    fn from(item: CreateUserWithAccountInput) -> Self {
        Self {
            auth_id: item.auth_id.into(),
            email: item.email.into(),
            first_name: item.first_name,
            last_name: item.last_name,
            address: item.address.map(Into::into),
            skip_create_customer: item.skip_create_customer,
        }
    }
}

#[derive(async_graphql::InputObject)]
pub struct UserInput {
    address: AddressInput,
    email: Email,
    first_name: String,
    last_name: String,
}

#[derive(async_graphql::InputObject)]
pub struct UserUpdateInput {
    address: AddressInput,
    first_name: String,
    last_name: String,
}

#[derive(async_graphql::InputObject)]
pub struct AccountActivatePlanInput {
    id: ID,
    name: String,
    plan_code: PlanCode,
}

#[derive(async_graphql::InputObject)]
pub struct AccountUpdateInput {
    id: ID,
    payment_method_id: ID,
}

#[derive(async_graphql::InputObject)]
pub struct AddressInput {
    city: String,
    country: Option<String>,
    line1: String,
    line2: Option<String>,
    postal_code: String,
}

impl From<AddressInput> for piteo_lib::AddressInput {
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

#[derive(async_graphql::InputObject)]
#[graphql(name = "PropertyInput")]
pub struct CreatePropertyInput {
    address: AddressInput,
    build_period: PropertyBuildPeriodType,
    building_legal_status: PropertyBuildingLegalStatus,
    common_spaces: Option<String>,
    energy_class: Option<PropertyEnergyClass>,
    equipments: Option<String>,
    gas_emission: Option<PropertyGasEmission>,
    heating_method: PropertyUsageType,
    housing_type: PropertyUsageType,
    lender_id: ID,
    name: String,
    note: Option<String>,
    ntic_equipments: Option<String>,
    other_spaces: Option<String>,
    room_count: PropertyRoomType,
    status: Option<PropertyStatus>,
    surface: f64,
    tax: Option<Decimal>,
    tenant_private_spaces: Option<String>,
    usage_type: PropertyHabitationUsageType,
    water_heating_method: PropertyUsageType,
}

impl TryFrom<CreatePropertyInput> for piteo_lib::CreatePropertyInput {
    type Error = Error;

    fn try_from(item: CreatePropertyInput) -> Result<Self, Self::Error> {
        Ok(Self {
            address: item.address.into(),
            build_period: item.build_period,
            building_legal_status: item.building_legal_status,
            common_spaces: item.common_spaces,
            energy_class: item.energy_class,
            equipments: item.equipments,
            gas_emission: item.gas_emission,
            heating_method: item.heating_method,
            housing_type: item.housing_type,
            lender_id: item.lender_id.try_into()?,
            name: item.name,
            note: item.note,
            ntic_equipments: item.ntic_equipments,
            other_spaces: item.other_spaces,
            room_count: item.room_count,
            status: item.status,
            surface: item.surface,
            tax: item.tax.map(Into::into),
            tenant_private_spaces: item.tenant_private_spaces,
            usage_type: item.usage_type,
            water_heating_method: item.water_heating_method,
        })
    }
}

#[derive(async_graphql::InputObject)]
#[graphql(name = "PropertyUpdateInput")]
pub struct UpdatePropertyInput {
    address: Option<AddressInput>,
    build_period: Option<PropertyBuildPeriodType>,
    building_legal_status: Option<PropertyBuildingLegalStatus>,
    common_spaces: Option<String>,
    energy_class: Option<PropertyEnergyClass>,
    equipments: Option<String>,
    gas_emission: Option<PropertyGasEmission>,
    heating_method: Option<PropertyUsageType>,
    housing_type: Option<PropertyUsageType>,
    id: ID,
    name: Option<String>,
    note: Option<String>,
    ntic_equipments: Option<String>,
    other_spaces: Option<String>,
    room_count: Option<PropertyRoomType>,
    status: Option<PropertyStatus>,
    surface: Option<f64>,
    tax: Option<Decimal>,
    tenant_private_spaces: Option<String>,
    usage_type: Option<PropertyHabitationUsageType>,
    water_heating_method: Option<PropertyUsageType>,
}

impl TryFrom<UpdatePropertyInput> for piteo_lib::UpdatePropertyInput {
    type Error = Error;

    fn try_from(item: UpdatePropertyInput) -> Result<Self, Self::Error> {
        Ok(Self {
            address: item.address.map(Into::into),
            build_period: item.build_period,
            building_legal_status: item.building_legal_status,
            common_spaces: item.common_spaces,
            energy_class: item.energy_class,
            equipments: item.equipments,
            gas_emission: item.gas_emission,
            heating_method: item.heating_method,
            housing_type: item.housing_type,
            id: item.id.try_into()?,
            name: item.name,
            note: item.note,
            ntic_equipments: item.ntic_equipments,
            other_spaces: item.other_spaces,
            room_count: item.room_count,
            status: item.status,
            surface: item.surface,
            tax: item.tax.map(Into::into),
            tenant_private_spaces: item.tenant_private_spaces,
            usage_type: item.usage_type,
            water_heating_method: item.water_heating_method,
        })
    }
}

#[derive(async_graphql::InputObject)]
#[graphql(name = "TenantInput")]
pub struct CreateTenantInput {
    apl: Option<bool>,
    birthdate: Date,
    birthplace: Option<String>,
    email: Email,
    first_name: String,
    last_name: String,
    note: Option<String>,
    phone_number: Option<PhoneNumber>,
    visale_id: Option<String>,
}

impl From<CreateTenantInput> for piteo_lib::CreateTenantInput {
    fn from(item: CreateTenantInput) -> Self {
        Self {
            apl: item.apl,
            birthdate: item.birthdate.into(),
            birthplace: item.birthplace,
            email: item.email.into(),
            first_name: item.first_name,
            last_name: item.last_name,
            note: item.note,
            phone_number: item.phone_number.map(Into::into),
            visale_id: item.visale_id,
        }
    }
}

#[derive(async_graphql::InputObject)]
#[graphql(name = "TenantUpdateInput")]
pub struct UpdateTenantInput {
    apl: Option<bool>,
    birthdate: Option<Date>,
    birthplace: Option<String>,
    email: Option<Email>,
    id: ID,
    first_name: Option<String>,
    last_name: Option<String>,
    note: Option<String>,
    phone_number: Option<PhoneNumber>,
    visale_id: Option<String>,
}

impl TryFrom<UpdateTenantInput> for piteo_lib::UpdateTenantInput {
    type Error = Error;

    fn try_from(item: UpdateTenantInput) -> Result<Self, Self::Error> {
        Ok(Self {
            apl: item.apl,
            birthdate: item.birthdate.map(Into::into),
            birthplace: item.birthplace,
            email: item.email.map(Into::into),
            id: item.id.try_into()?,
            first_name: item.first_name,
            last_name: item.last_name,
            note: item.note,
            phone_number: item.phone_number.map(Into::into),
            visale_id: item.visale_id,
        })
    }
}

#[derive(async_graphql::InputObject)]
pub struct CompanyInput {
    address: AddressInput,
    email: Email,
    legal_entity: String,
}

#[derive(async_graphql::InputObject)]
pub struct CompanyUpdateInput {
    address: Option<AddressInput>,
}

#[derive(async_graphql::InputObject)]
pub struct LenderIndividualInput {
    individual: UserInput,
}

#[derive(async_graphql::InputObject)]
pub struct LenderCompanyInput {
    company: CompanyInput,
}

#[derive(async_graphql::InputObject)]
pub struct LenderIndividualUpdateInput {
    id: ID,
    individual: UserUpdateInput,
}

#[derive(async_graphql::InputObject)]
pub struct LenderCompanyUpdateInput {
    company: CompanyUpdateInput,
    id: ID,
}

#[derive(async_graphql::InputObject)]
pub struct LeaseFurnishedInput {
    data: Option<LeaseFurnishedDataInput>,
    deposit_amount: Option<Decimal>,
    effect_date: Date,
    renew_date: Option<Date>,
    file: Option<FileInput>,
    property_id: ID,
    rent_amount: Decimal,
    rent_charges_amount: Option<Decimal>,
    signature_date: Option<Date>,
    tenant_ids: Vec<ID>,
    // r#type: LeaseType,
}

#[derive(async_graphql::InputObject)]
pub struct LeaseFurnishedUpdateInput {
    data: Option<LeaseFurnishedDataInput>,
    file: Option<FileInput>,
    id: ID,
}

#[derive(async_graphql::InputObject)]
pub struct DocumentGenerateInput {
    id: ID,
    r#type: FileType,
}

#[derive(async_graphql::InputObject)]
pub struct SendPaymentNoticeInput {
    lease_id: ID,
    date: Option<Date>,
}

#[derive(async_graphql::InputObject)]
pub struct LeaseFurnishedDataInput {
    charges_recuperation_mode: Option<RentChargesRecuperationMode>,
    charges_revision_method: Option<String>,
    colocation_insurance_lender: Option<bool>,
    colocation_insurance_monthly_amount: Option<Decimal>,
    colocation_insurance_total_amount: Option<Decimal>,
    duration: Option<LeaseFurnishedDuration>,
    lender_fee_cap: Option<Decimal>,
    lender_fee_cap_other: Option<String>,
    lender_fee_cap_prestations: Option<Decimal>,
    other_conditions: Option<String>,
    rent_complement: Option<Decimal>,
    rent_complement_property_justification: Option<String>,
    rent_first_amount: Option<Decimal>,
    rent_irl: Option<LeaseRentReferenceIrl>,
    rent_irl_revision_date: Option<Date>,
    rent_maj_decree_increased_amount: Option<Decimal>,
    rent_maj_decree_reference_amount: Option<Decimal>,
    rent_majoration_decree: Option<bool>,
    rent_max_evolution_relocation: Option<bool>,
    rent_payment_date: Option<Date>,
    rent_payment_method: Option<RentPaymentMethod>,
    rent_payment_place: Option<String>,
    rent_periodicity: Option<LeaseRentPeriodicity>,
    rent_underestimated_method: Option<String>,
    rent_underestimated_monthly_variation: Option<Decimal>,
    resolutary_clause: Option<String>,
    solidarity_clause: Option<String>,
    tenant_fee_cap_new_rental: Option<Decimal>,
    tenant_fee_cap_prestations: Option<Decimal>,
    tenant_fee_cap_report_by_meter: Option<Decimal>,
    tenant_fee_cap_report_prestations: Option<Decimal>,
    tenant_last_rent_amount: Option<Decimal>,
    tenant_last_rent_received_date: Option<Date>,
    tenant_last_rent_revision_date: Option<Date>,
    works_decence_since_last_rental: Option<String>,
    works_rent_decrease_tenant: Option<String>,
    works_rent_increase_lender: Option<String>,
}

#[derive(async_graphql::InputObject)]
pub struct RentInput {
    amount: Decimal,
    charges_amount: Option<Decimal>,
    lease_id: ID,
    period_end: Option<DateTime>,
    period_start: Option<DateTime>,
}

#[derive(async_graphql::InputObject)]
pub struct RentReceiptInput {
    lease_id: ID,
    send_mail: Option<bool>,
}

#[derive(async_graphql::InputObject)]
pub struct RentReceiptsInput {
    send_mail: Option<bool>,
}

#[derive(async_graphql::InputObject)]
pub struct TransactionInput {
    amount: Decimal,
    lease_id: ID,
    date: Date,
    r#type: Option<TransactionType>,
}

#[derive(async_graphql::InputObject)]
pub struct FileInput {
    download_url: String,
    r#type: FileType,
}

#[derive(async_graphql::InputObject)]
pub struct ImportInput {
    files: Vec<FileInput>,
    source: ImportSource,
}

#[derive(async_graphql::InputObject)]
pub struct MailSendInput {
    id: ID,
    r#type: FileType,
}
