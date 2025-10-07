use crate::domain::models::TeamID;

pub type Places = usize;

#[derive(Debug, Clone)]
pub struct Reservation {
    team_id: TeamID,
    places: Places,
}

impl Reservation {
    pub fn new(team_id: TeamID, places: Places) -> Self {
        Self { team_id, places }
    }

    pub fn team_id(&self) -> &TeamID {
        &self.team_id
    }

    pub fn places(&self) -> Places {
        self.places
    }
}
