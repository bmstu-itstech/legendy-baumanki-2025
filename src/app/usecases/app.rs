use crate::app::usecases::get_available_tasks::GetAvailableTasks;
use crate::app::usecases::get_task::GetTask;
use crate::app::usecases::{AnswerTask, CheckAdmin, CheckRegistered, CheckStartedTrack, GetCharacter, GetCharacterNames, GetMedia, GetProfile, GetAvailableTracks, GetTeamWithMembers, GetTrackInProgress, GetUser, GetUserTeam, GiveFeedback, StartTrack, UploadMedia, GetCompletedTasks, CheckCaptain};

pub struct App {
    pub answer_task: AnswerTask,
    pub check_admin: CheckAdmin,
    pub check_captain: CheckCaptain,
    pub check_registered: CheckRegistered,
    pub check_started_track: CheckStartedTrack,
    pub get_available_tasks: GetAvailableTasks,
    pub get_available_tracks: GetAvailableTracks,
    pub get_character: GetCharacter,
    pub get_character_names: GetCharacterNames,
    pub get_completed_tasks: GetCompletedTasks,
    pub get_media: GetMedia,
    pub get_profile: GetProfile,
    pub get_task: GetTask,
    pub get_team_with_members: GetTeamWithMembers,
    pub get_track_in_progress: GetTrackInProgress,
    pub get_user: GetUser,
    pub get_user_team: GetUserTeam,
    pub give_feedback: GiveFeedback,
    pub start_track: StartTrack,
    pub upload_media: UploadMedia,
}
