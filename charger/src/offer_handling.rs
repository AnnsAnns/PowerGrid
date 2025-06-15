use crate::ChargerHandler;
use log::debug;

/// # Description
/// The `ReservedOffer` struct represents an offer that the charger has reserved for a vehicle.
/// Its used only in the charger to manage reserved offers.
/// 
/// # Fields
/// - `id`: A unique identifier for the reserved offer.
/// - `quantity`: The amount of charge reserved for the offer, in kWh.
/// - `price`: The price per unit of charge for the reserved offer.
/// - `was_accepted`: A boolean indicating whether the offer was accepted by the vehicle or not.
#[derive(Debug, Clone)]
pub struct ReservedOffer {
    id: String,
    quantity: usize,
    price: f64,
    was_accepted: bool,
}

impl ReservedOffer {
    /// # Description
    /// Creates a new `ReservedOffer` instance with the specified parameters.
    /// 
    /// # Parameters
    /// - `id`: A unique identifier for the reserved offer.
    /// - `quantity`: The amount of charge reserved for the offer, in kWh.
    /// - `price`: The price per unit of charge for the reserved offer.
    /// 
    /// # Returns
    /// A new `ReservedOffer` instance.
    pub fn new(id: String, quantity: usize, price: f64) -> Self {
        ReservedOffer {
            id,
            quantity,
            price,
            was_accepted: false,
        }
    }
}

impl ChargerHandler {
    /// Reserve an offer by adding it to the reserved offers list
    /// # Arguments
    /// * `offer` - The offer to reserve
    ///
    /// This method will reserve the charge and port for the offer and add it to the list of currently reserved offers.
    pub fn reserve_offer(&mut self, offer: ReservedOffer) {
        debug!(
            "Reserving offer with ID: {}, Quantity: {}, Price: {}",
            offer.id, offer.quantity, offer.price
        );

        self.charger.reserve_charge(offer.quantity as usize);
        self.charger.reserve_port();
        self.currently_reserved_for.push(offer);
    }

    /// Release a reserved offer by its ID
    /// # Arguments
    /// * `offer_id` - The ID of the offer to release
    ///
    /// This method will release the reserved offer and free up the port and charge.
    /// If the offer is not found, it will log a debug message.
    pub fn release_offer(&mut self, offer_id: String, release_reserved_charge: bool) {
        let offer = match self.get_reserved_offer(offer_id.clone()) {
            Some(o) => o,
            None => {
                debug!("Offer with ID {} not found in reserved offers", offer_id);
                return;
            }
        };

        debug!("Releasing reserved offer with ID: {}", offer_id);
        if release_reserved_charge {
            self.charger
                .release_reserved_charge(offer.quantity as usize);
        }
        self.charger.release_port();

        self.currently_reserved_for.retain(|o| o.id != offer_id);
    }

    /// # Description
    /// Get a reserved offer by its ID.<br>
    /// This method searches for a reserved offer by its ID and returns it if found.<br>
    /// If the offer is not found, it returns `None`.
    ///
    /// # Arguments
    /// - `offer_id` - The ID of the offer to retrieve
    pub fn get_reserved_offer(&self, offer_id: String) -> Option<&ReservedOffer> {
        self.currently_reserved_for
            .iter()
            .find(|o| o.id == offer_id)
    }

    /// # Description
    /// Accept a reserved offer by marking it as accepted.<br>
    /// This method will mark the offer as accepted if it exists in the reserved offers.<br>
    /// If the offer is not found, it will log a debug message.
    ///
    /// # Arguments
    /// - `offer_id` - The ID of the offer to accept
    pub fn accept_reserve(&mut self, offer_id: String) {
        if self.get_reserved_offer(offer_id.clone()).is_some() {
            debug!("Accepting reserved offer with ID: {}", offer_id);
            self.currently_reserved_for
                .iter_mut()
                .find(|o| o.id == offer_id)
                .map(|o| o.was_accepted = true);
        } else {
            debug!("Offer with ID {} not found for acceptance", offer_id);
        }
    }
}
