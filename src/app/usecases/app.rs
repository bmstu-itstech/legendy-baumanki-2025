use crate::app::usecases::{AnswerTask, ChangeFullName, ChangeGroupName, CheckAdmin, CheckRegistered, CheckTeamExists, CreateTeam, ExitTeam, GetMedia, GetProfile, GetTask, GetTeamWithMembers, GetUserTask, GetUserTasks, GetUserTeam, JoinTeam, RegisterUser, RemoveMember, UploadMedia};

pub struct App {
    pub answer_task: AnswerTask,
    pub change_full_name: ChangeFullName,
    pub change_group_name: ChangeGroupName,
    pub check_admin: CheckAdmin,
    pub check_registered: CheckRegistered,
    pub check_team_exists: CheckTeamExists,
    pub create_team: CreateTeam,
    pub exit_team: ExitTeam,
    pub get_media: GetMedia,
    pub get_profile: GetProfile,
    pub get_task: GetTask,
    pub get_team_with_members: GetTeamWithMembers,
    pub get_user_tasks: GetUserTasks,
    pub get_user_task: GetUserTask,
    pub get_user_team: GetUserTeam,
    pub join_team: JoinTeam,
    pub register_user: RegisterUser,
    pub remove_member: RemoveMember,
    pub upload_media: UploadMedia,
}
