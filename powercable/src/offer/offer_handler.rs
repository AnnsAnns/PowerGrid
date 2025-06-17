use std::collections::HashMap;
use super::Offer;

pub struct OfferHandler {
    outstanding_offers: HashMap<String, Offer>,
    sent_offers: HashMap<String, Offer>,
}

impl OfferHandler {
    pub fn new() -> Self {
        OfferHandler {
            outstanding_offers: HashMap::new(),
            sent_offers: HashMap::new(),
        }
    }

    pub fn add_offer(&mut self, offer: Offer) {
        self.outstanding_offers.insert(offer.get_id().to_string(), offer);
    }

    pub fn remove_offer(&mut self, id: &str) {
        self.outstanding_offers.remove(id);
    }

    pub fn get_offer(&self, id: &str) -> Option<&Offer> {
        self.outstanding_offers.get(id)
    }

    pub fn get_first_offer(&self) -> Option<&Offer> {
        self.outstanding_offers.values().next()
    }

    pub fn get_best_non_sent_offer(&self) -> Option<&Offer> {
        self.outstanding_offers
            .values()
            .filter(|offer| !self.sent_offers.contains_key(offer.get_id()))
            .min_by(|a, b| a.get_price().partial_cmp(&b.get_price()).unwrap())
    }

    pub fn has_offer(&self, id: &str) -> bool {
        self.outstanding_offers.contains_key(id)
    }

    pub fn has_sent_offer(&self, id: &str) -> bool {
        self.sent_offers.contains_key(id)
    }

    pub fn add_sent_offer(&mut self, offer: Offer) {
        self.sent_offers.insert(offer.get_id().to_string(), offer);
    }

    pub fn remove_sent_offer(&mut self, id: &str) {
        self.sent_offers.remove(id);
    }

    pub fn get_sent_offer(&self, id: &str) -> Option<&Offer> {
        self.sent_offers.get(id)
    }

    pub fn get_all_offers(&self) -> Vec<&Offer> {
        self.outstanding_offers.values().collect()
    }

    pub fn has_offers(&self) -> bool {
        !self.outstanding_offers.is_empty()
    }

    pub fn remove_all_offers(&mut self) {
        self.outstanding_offers.clear();
        self.sent_offers.clear();
    }
}