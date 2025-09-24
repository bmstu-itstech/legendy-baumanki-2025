use crate::app::usecases::{
    AnswerTask, CheckAdmin, CheckRegistered, CreateTeam, ExitTeam, GetCharacter, GetCharacterNames,
    GetMedia, GetProfile, GetTask, GetTeamWithMembers, GetUserTask, GetUserTasks, GetUserTeam,
    JoinTeam, RegisterUser, UploadMedia,
};

pub struct App {
    pub answer_task: AnswerTask,
    pub check_admin: CheckAdmin,
    pub check_registered: CheckRegistered,
    pub create_team: CreateTeam,
    pub exit_team: ExitTeam,
    pub get_character: GetCharacter,
    pub get_character_names: GetCharacterNames,
    pub get_media: GetMedia,
    pub get_profile: GetProfile,
    pub get_task: GetTask,
    pub get_team_with_members: GetTeamWithMembers,
    pub get_user_tasks: GetUserTasks,
    pub get_user_task: GetUserTask,
    pub get_user_team: GetUserTeam,
    pub join_team: JoinTeam,
    pub register_user: RegisterUser,
    pub upload_media: UploadMedia,
}
