use crate::domain::error::DomainError;
use crate::domain::models::{Places, Reservation, TeamID};
use crate::utils::uuid::new_pseudo_uuid;
use crate::{not_empty_string_impl, pseudo_uuid_impl};
use chrono::NaiveTime;

#[derive(Debug, Clone)]
pub struct SlotID(String);
pseudo_uuid_impl!(SlotID, 4);

#[derive(Debug, Clone)]
pub struct Site(String);
not_empty_string_impl!(Site);

#[derive(Debug, Clone)]
pub struct Slot {
    id: SlotID,
    start: NaiveTime,
    site: Site,
    capacity: Places,
    reservations: Vec<Reservation>,
}

impl Slot {
    pub fn new(start: NaiveTime, place: Site, capacity: usize) -> Self {
        Self {
            id: SlotID::new(),
            start,
            reservations: vec![],
            site: place,
            capacity,
        }
    }

    pub fn restore(
        id: SlotID,
        start: NaiveTime,
        place: Site,
        capacity: usize,
        reservations: Vec<Reservation>,
    ) -> Self {
        Self {
            id,
            start,
            site: place,
            capacity,
            reservations,
        }
    }

    pub fn can_be_reserved(&self, places: Places) -> bool {
        self.reserved() + places <= self.capacity()
    }

    pub fn available_places(&self) -> Places {
        self.capacity.saturating_sub(self.reserved())
    }

    pub fn reserve(&mut self, team_id: TeamID, places: Places) -> Result<(), DomainError> {
        if !self.can_be_reserved(places) {
            return Err(DomainError::CanNotReserveSlot(self.id.clone(), places));
        }
        let reservation = Reservation::new(team_id, places);
        self.reservations.push(reservation);
        Ok(())
    }

    pub fn cancel_reservation(&mut self, team_id: &TeamID) -> Result<(), DomainError> {
        let before = self.reservations.len();
        self.reservations.retain(|r| r.team_id() != team_id);
        let after = self.reservations.len();
        if before == after {
            Err(DomainError::TeamNotReservedSlot(team_id.clone()))
        } else {
            Ok(())
        }
    }

    pub fn reserved(&self) -> usize {
        self.reservations.iter().fold(0, |acc, r| acc + r.places())
    }

    pub fn id(&self) -> &SlotID {
        &self.id
    }

    pub fn start(&self) -> NaiveTime {
        self.start
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn site(&self) -> &Site {
        &self.site
    }

    pub fn reservations(&self) -> &Vec<Reservation> {
        &self.reservations
    }
}
