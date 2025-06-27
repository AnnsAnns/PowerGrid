use crate::ChargerHandler;
use tracing::debug;

/// # Description
/// The `ReservedOffer` struct represents an offer that the charger has reserved for a vehicle.
/// Its used only in the charger to manage reserved offers.
/// 
/// # Fields
/// - `vehicle_name`: The name of the vehicle for which the offer is reserved.
/// - `quantity`: The amount of charge reserved for the offer, in kWh.
/// - `price`: The price per unit of charge for the reserved offer.
/// - `was_accepted`: A boolean indicating whether the offer was accepted by the vehicle or not.
#[derive(Debug, Clone)]
pub struct ReservedOffer {
    vehicle_name: String,
    quantity: usize,
    price: f64,// TODO: should be used??
    was_accepted: bool,
}

impl ReservedOffer {
    /// # Description
    /// Creates a new `ReservedOffer` instance with the specified parameters.
    /// 
    /// # Parameters
    /// - `vehicle_name`: The name of the vehicle for which the offer is reserved.
    /// - `quantity`: The amount of charge reserved for the offer, in kWh.
    /// - `price`: The price per unit of charge for the reserved offer.
    /// 
    /// # Returns
    /// A new `ReservedOffer` instance.
    pub fn new(vehicle_name: String, quantity: usize, price: f64) -> Self {
        ReservedOffer {
            vehicle_name,
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
        debug!("Reserving offer {:?}", offer);
        self.charger.reserve_charge(offer.quantity as usize);
        self.charger.reserve_port();
        self.currently_reserved_for.push(offer);
    }

    /// Release a reserved offer by vehicle name
    /// # Arguments
    /// * `vehicle_name` - The name of the vehicle for which the offer was reserved
    ///
    /// This method will release the reserved offer and free up the port and charge.
    /// If the offer is not found, it will log a debug message.
    pub fn release_offer(&mut self, vehicle_name: String, release_reserved_charge: bool) {
        let offer = match self.get_reserved_offer(vehicle_name.clone()) {
            Some(o) => o,
            None => {
                debug!("ReservedOffer for {} not found in reserved offers", vehicle_name);
                return;
            }
        };

        debug!("Releasing ReservedOffer for {}", vehicle_name);
        if release_reserved_charge {
            self.charger
                .release_reserved_charge(offer.quantity as usize);
        }
        self.charger.release_port();

        self.currently_reserved_for.retain(|o| o.vehicle_name != vehicle_name);
    }

    /// # Description
    /// Get a reserved offer by vehicle_name.<br>
    /// This method searches for a reserved offer by the vehicle name and returns it if found.<br>
    /// If the offer is not found, it returns `None`.
    ///
    /// # Arguments
    /// - `vehicle_name` - The name of the vehicle
    pub fn get_reserved_offer(&self, vehicle_name: String) -> Option<&ReservedOffer> {
        self.currently_reserved_for
            .iter()
            .find(|o| o.vehicle_name == vehicle_name)
    }

    /// # Description
    /// Accept a reserved offer by marking it as accepted.<br>
    /// This method will mark the offer as accepted if it exists in the reserved offers.<br>
    /// If the offer is not found, it will log a debug message.
    ///
    /// # Arguments
    /// - `vehicle_name` - The name of the vehicle
    pub fn accept_reserve(&mut self, vehicle_name: String) {
        if self.get_reserved_offer(vehicle_name.clone()).is_some() {
            debug!("Accepting reserved offer for {}", vehicle_name);
            self.currently_reserved_for
                .iter_mut()
                .find(|o| o.vehicle_name == vehicle_name)
                .map(|o| o.was_accepted = true);
        } else {
            debug!("Offer for {} not found for acceptance", vehicle_name);
        }
    }
}
