use crate::schema::candidacies;
use crate::AdvertisementId;
use crate::Date;
use crate::DateTime;
use crate::Id;
use crate::PersonId;
use crate::Url;
use trankeel_kit::config::config;

// # Types

pub type CandidacyId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Candidacystatus"]
pub enum CandidacyStatus {
    Pending,
    Rejected,
    Accepted,
}

impl Default for CandidacyStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Clone, Debug, Insertable, Queryable, SimpleObject)]
#[table_name = "candidacies"]
pub struct Candidacy {
    pub id: CandidacyId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub status: CandidacyStatus,
    pub advertisement_id: AdvertisementId,
    pub person_id: PersonId,
    pub move_in_date: DateTime,
    pub description: String,
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    pub is_student: Option<bool>,
}

impl Candidacy {
    pub fn as_url(&self) -> Url {
        config()
            .routes("candidacy_url")
            .unwrap()
            .replace(":id", &self.id.to_string())
            .into()
    }
}

#[derive(Default, AsChangeset, Identifiable, Insertable)]
#[table_name = "candidacies"]
pub struct CandidacyData {
    pub id: CandidacyId,
    pub status: Option<CandidacyStatus>,
    pub advertisement_id: Option<AdvertisementId>,
    pub person_id: Option<PersonId>,
    pub move_in_date: Option<DateTime>,
    pub description: Option<String>,
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    pub is_student: Option<bool>,
}
