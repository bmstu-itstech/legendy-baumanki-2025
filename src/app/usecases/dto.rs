use crate::domain::models::{
    FullName, GroupName, MAX_TEAM_SIZE, Team, TeamID, TeamName, User, Username,
};

pub struct UserDTO {
    pub username: Option<Username>,
    pub full_name: FullName,
    pub group_name: GroupName,
}

impl From<User> for UserDTO {
    fn from(u: User) -> Self {
        Self {
            username: u.username().clone(),
            full_name: u.full_name().clone(),
            group_name: u.group_name().clone(),
        }
    }
}

pub struct TeamDTO {
    pub id: TeamID,
    pub name: TeamName,
    pub size: usize,
    pub max_size: usize,
    pub completed: bool,
}

impl From<Team> for TeamDTO {
    fn from(t: Team) -> Self {
        Self {
            id: t.id().clone(),
            name: t.name().clone(),
            size: t.member_ids().len(),
            max_size: MAX_TEAM_SIZE,
            completed: t.is_completed(),
        }
    }
}

pub struct Profile {
    pub full_name: FullName,
    pub group_name: GroupName,
    pub team_name: Option<TeamName>,
}

pub struct TeamWithMembersDTO {
    pub id: TeamID,
    pub name: TeamName,
    pub size: usize,
    pub max_size: usize,
    pub completed: bool,
    pub members: Vec<UserDTO>,
}
