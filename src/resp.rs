use chrono::Datelike;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, ops::Index};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SearchCreateResponse {
    pub session_token: String,
    pub status: ResultStatus,
    pub action: ResultAction,
    pub content: FlightsLiveSearchContent,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase", deny_unknown_fields)]
// pub struct SearchPollResponse {
//     pub refresh_session_token: String,
//     pub status: ResultStatus,
//     pub content: FlightsLiveSearchContent,
// }

pub type SearchPollResponse = SearchCreateResponse;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResultStatus {
    ResultStatusUnspecified,
    ResultStatusComplete,
    ResultStatusIncomplete,
    ResultStatusFailed,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResultAction {
    ResultActionUnspecified,
    ResultActionReplaced,
    ResultActionNotModified,
    ResultActionOmitted,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlightsLiveSearchContent {
    pub results: LiveSearchResults,
    pub stats: LiveStats,
    pub sorting_options: LiveSortingOptions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveStats {
    pub itineraries: Option<LiveItineraryStats>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveItineraryStats {
    pub min_duration: i32,
    pub max_duration: i32,
    pub total: LiveItinerarySummary,
    pub stops: LiveItineraryStopStats,
    pub has_change_airport_transfer: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveItinerarySummary {
    pub count: i32,
    pub min_price: Price,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Price {
    #[serde(deserialize_with = "deserialize_price_amount")]
    pub amount: Option<i64>,
    pub unit: PriceUnit,
    pub update_status: PriceUpdateStatus,
}

impl Price {
    pub fn decimal(&self) -> Option<Decimal> {
        if let Some(amount) = self.amount {
            match self.unit {
                PriceUnit::PriceUnitUnspecified => None,
                PriceUnit::PriceUnitWhole => Some(Decimal::new(amount, 0)),
                PriceUnit::PriceUnitCenti => Some(Decimal::new(amount, 2)),
                PriceUnit::PriceUnitMilli => Some(Decimal::new(amount, 3)),
                PriceUnit::PriceUnitMicro => Some(Decimal::new(amount, 6)),
            }
        } else {
            None
        }
    }
}

pub fn deserialize_price_amount<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = Value::deserialize(deserializer)?;

    if v.as_str() == Some("") {
        return Ok(None);
    }

    match &v {
        Value::String(s) => s
            .parse::<i64>()
            .map(Some)
            .map_err(|e| serde::de::Error::custom(format!("expected i64, got {:?}", e))),
        Value::Number(n) => n
            .as_i64()
            .ok_or_else(|| serde::de::Error::custom(format!("expected i64, got {:?}", v)))
            .map(Some),
        _ => Err(serde::de::Error::custom(format!(
            "expected number, got {:?}",
            v
        ))),
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceUnit {
    PriceUnitUnspecified,
    PriceUnitWhole,
    PriceUnitCenti,
    PriceUnitMilli,
    PriceUnitMicro,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceUpdateStatus {
    PriceUpdateStatusUnspecified,
    PriceUpdateStatusPending,
    PriceUpdateStatusCurrent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveItineraryStopStats {
    pub direct: LiveItineraryStopSummaryStats,
    pub one_stop: LiveItineraryStopSummaryStats,
    pub two_plus_stops: LiveItineraryStopSummaryStats,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveItineraryStopSummaryStats {
    pub total: LiveItinerarySummary,
    pub ticket_types: LiveItineraryStopTicketStats,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveItineraryStopTicketStats {
    pub single_ticket: LiveItinerarySummary,
    pub multi_ticket_non_npt: LiveItinerarySummary,
    pub multi_ticket_npt: LiveItinerarySummary,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveSortingOptions {
    pub best: Vec<LiveSortingOptionItem>,
    pub cheapest: Vec<LiveSortingOptionItem>,
    pub fastest: Vec<LiveSortingOptionItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum LiveSortingOption {
    Best,
    Cheapest,
    Fastest,
}

impl Index<LiveSortingOption> for LiveSortingOptions {
    type Output = Vec<LiveSortingOptionItem>;

    fn index(&self, index: LiveSortingOption) -> &Self::Output {
        match index {
            LiveSortingOption::Best => &self.best,
            LiveSortingOption::Cheapest => &self.cheapest,
            LiveSortingOption::Fastest => &self.fastest,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveSortingOptionItem {
    pub score: f64,
    pub itinerary_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveSearchResults {
    pub itineraries: HashMap<String, LiveItinerary>,
    pub legs: HashMap<String, LiveLeg>,
    pub segments: HashMap<String, LiveSegment>,
    pub places: HashMap<String, LivePlace>,
    pub carriers: HashMap<String, LiveCarrier>,
    pub agents: HashMap<String, LiveAgent>,
    pub alliances: HashMap<String, LiveAlliance>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveAlliance {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveItinerary {
    pub pricing_options: Vec<LivePricingOption>,
    pub leg_ids: Vec<String>,
    pub sustainability_data: LiveSustainabilityData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LivePricingOption {
    pub price: Price,
    pub agent_ids: Vec<String>,
    pub items: LivePricingOptionItem,
    pub transfer_type: LiveTransferType,
    pub id: String,
    pub pricing_option_fare: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LiveTransferType {
    TransferTypeUnspecified,
    TransferTypeManaged,
    TransferTypeSelfTransfer,
    TransferTypeProtectedSelfTransfer,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveLeg {
    pub origin_place_id: String,
    pub destination_place_id: String,
    pub departure_date_time: LiveLocalDateTime,
    pub arrival_date_time: LiveLocalDateTime,
    pub duration_in_minutes: i32,
    pub stop_count: i32,
    pub marketing_carrier_ids: Vec<String>,
    pub operating_carrier_ids: Vec<String>,
    pub segment_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveLocalDateTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

impl std::fmt::Display for LiveLocalDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LivePlace {
    pub name: String,
    #[serde(rename = "type")]
    pub place_type: LivePlaceType,
    pub iata: Option<String>,
    pub coordinates: Option<LiveCoordinates>,
    pub entity_id: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LivePlaceType {
    PlaceTypeUnspecified,
    PlaceTypeAirport,
    PlaceTypeCity,
    PlaceTypeCountry,
    PlaceTypeContinent,
    PlaceTypeIsland,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiveCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

// I give up
pub type LivePricingOptionItem = Value;
pub type LiveSustainabilityData = Value;
pub type LiveSegment = Value;
pub type LiveCarrier = Value;
pub type LiveAgent = Value;
