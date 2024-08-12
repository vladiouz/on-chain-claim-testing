use crate::address_info::*;

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("new_claim")]
    fn new_claim_event(&self, #[indexed] address: &ManagedAddress, info: &AddressInfo);

    #[event("new_claim_and_repair")]
    fn new_claim_and_repair_event(&self, #[indexed] address: &ManagedAddress, info: &AddressInfo);

    #[event("new_update_state")]
    fn new_update_state_event(&self, #[indexed] address: &ManagedAddress, info: &AddressInfo);

    #[event("new_set_repair_payment")]
    fn new_set_repair_payment_event(&self);
}
