use chrono::{Datelike, NaiveDate};

use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::borrow::Cow;

#[derive(Serialize, Debug)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum QueryPlace {
    EntityId { entity_id: Cow<'static, str> },
    Iata { iata: Cow<'static, str> },
}

impl From<&'static str> for QueryPlace {
    fn from(s: &'static str) -> Self {
        Self::Iata { iata: s.into() }
    }
}

impl From<String> for QueryPlace {
    fn from(s: String) -> Self {
        Self::Iata { iata: s.into() }
    }
}

impl From<&String> for QueryPlace {
    fn from(s: &String) -> Self {
        Self::Iata {
            iata: s.clone().into(),
        }
    }
}

fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut state = serializer.serialize_struct("Date", 3)?;
    state.serialize_field("year", &date.year())?;
    state.serialize_field("month", &date.month())?;
    state.serialize_field("day", &date.day())?;
    state.end()
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryLeg {
    pub origin_place_id: QueryPlace,
    pub destination_place_id: QueryPlace,

    #[serde(serialize_with = "serialize_date")]
    pub date: NaiveDate,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CabinClass {
    CabinClassUnspecified,
    CabinClassEconomy,
    CabinClassPremiumEconomy,
    CabinClassBusiness,
    CabinClassFirst,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub market: Cow<'static, str>,

    pub locale: Cow<'static, str>,
    pub currency: Cow<'static, str>,
    pub query_legs: Vec<QueryLeg>,
    pub adults: u32,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children_ages: Vec<u32>,

    // pub include_carriers_and_agents: Option<HashMap<String, Vec<String>>>,
    // pub exclude_carriers_and_agents: Option<HashMap<String, Vec<String>>>,
    // pub include_sustainability_data: Option<bool>,
    pub nearby_airports: Option<bool>,
    pub cabin_class: CabinClass,
}

impl Default for CreateRequest {
    fn default() -> Self {
        Self {
            market: "UK".into(),
            locale: "en-GB".into(),
            currency: "EUR".into(),
            query_legs: vec![],
            adults: 1,
            children_ages: vec![],
            nearby_airports: Some(true),
            cabin_class: CabinClass::CabinClassEconomy,
        }
    }
}
