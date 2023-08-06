use itertools::Itertools;

use crate::resp::{FlightsLiveSearchContent, LiveItinerary, LiveSortingOption};

impl FlightsLiveSearchContent {
    pub fn sorted(&self, option: LiveSortingOption) -> impl Iterator<Item = &LiveItinerary> {
        let sort = &self.sorting_options[option];

        sort.iter()
            .map(|item| &self.results.itineraries[&item.itinerary_id])
    }

    pub fn format_itinerary(&self, itin: &LiveItinerary) -> String {
        format!(
            "{} - {} || {} -> {} || {}",
            &self.results.legs[&itin.leg_ids[0]].departure_date_time,
            &self.results.legs[itin.leg_ids.last().unwrap()].arrival_date_time,
            // key,
            &self.results.places[&self.results.legs[&itin.leg_ids[0]].origin_place_id].name,
            itin.leg_ids
                .iter()
                .map(|leg_id| {
                    &self.results.places[&self.results.legs[leg_id].destination_place_id].name
                })
                .join("->"),
            itin.pricing_options
                .iter()
                .map(|p| {
                    format!(
                        "{}: {}",
                        p.agent_ids.join("."),
                        p.price.decimal().map(|v| v.to_string()).unwrap_or_default()
                    )
                })
                .join(" / ")
        )
    }
}
