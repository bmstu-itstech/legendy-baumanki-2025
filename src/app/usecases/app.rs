use crate::app::usecases::{
    ChangeFullName, ChangeGroupName, CheckRegistered, CheckTeamExists, CreateTeam, ExitTeam,
    GetProfile, GetTeamWithMembers, GetUserTeam, JoinTeam, RegisterUser, RemoveMember,
};

pub struct App {
    pub change_full_name: ChangeFullName,
    pub change_group_name: ChangeGroupName,
    pub check_registered: CheckRegistered,
    pub check_team_exists: CheckTeamExists,
    pub create_team: CreateTeam,
    pub exit_team: ExitTeam,
    pub get_profile: GetProfile,
    pub get_team_with_members: GetTeamWithMembers,
    pub get_user_team: GetUserTeam,
    pub join_team: JoinTeam,
    pub register_user: RegisterUser,
    pub remove_member: RemoveMember,
}
