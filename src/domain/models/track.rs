use crate::domain::error::DomainError;
use crate::domain::models::Points;
use crate::domain::models::Task;
use crate::domain::models::TaskID;
use crate::{
    domain::models::{Answer, MediaID},
    not_empty_string_impl,
};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum TrackTag {
    Muzhestvo,
    Volya,
    Trud,
    Uporstvo,
    Universitet,
}

impl TrackTag {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Muzhestvo => "Мужество",
            Self::Volya => "Воля",
            Self::Trud => "Труд",
            Self::Uporstvo => "Упорство",
            Self::Universitet => "Университет",
        }
    }

    pub fn try_parse(s: &str) -> Option<Self> {
        match s {
            "Мужество" => Some(Self::Muzhestvo),
            "Воля" => Some(Self::Volya),
            "Труд" => Some(Self::Trud),
            "Упорство" => Some(Self::Uporstvo),
            "Университет" => Some(Self::Universitet),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskStatus {
    NotAvailable,
    Available,
    InProgress,
    Completed,
}

#[derive(Debug, Clone)]
pub struct TrackDescription(String);
not_empty_string_impl!(TrackDescription);

#[derive(Debug, Clone)]
pub struct Track {
    tag: TrackTag,
    description: TrackDescription,
    media_id: MediaID,
    tasks: HashMap<TaskID, Task>,
}

impl Track {
    pub fn new(
        tag: TrackTag,
        description: TrackDescription,
        media_id: MediaID,
        tasks: Vec<Task>,
    ) -> Self {
        let tasks = HashMap::from_iter(tasks.into_iter().map(|t| (t.id(), t)));
        Self {
            tag,
            description,
            media_id,
            tasks,
        }
    }

    pub fn task(&self, id: &TaskID) -> Option<&Task> {
        self.tasks.get(id)
    }

    pub fn progress(&self, answers: &[&Answer]) -> TrackProgress {
        TrackProgress::new(self, answers)
    }

    pub fn tag(&self) -> TrackTag {
        self.tag
    }

    pub fn description(&self) -> &TrackDescription {
        &self.description
    }

    pub fn media_id(&self) -> &MediaID {
        &self.media_id
    }
}

pub struct TrackProgress<'a> {
    track: &'a Track,
    answers: HashMap<TaskID, Points>,
}

impl<'a> TrackProgress<'a> {
    pub fn new(track: &'a Track, answers: &[&Answer]) -> Self {
        let answers_map = answers
            .iter()
            .map(|&a| (a.task_id().clone(), a.points()))
            .collect();

        Self {
            track,
            answers: answers_map,
        }
    }

    pub fn task_status(&self, task_id: &TaskID) -> Option<TaskStatus> {
        let task = match self.track.task(task_id) {
            Some(t) => t,
            None => return None,
        };

        match self.answers.get(task_id) {
            Some(points) => {
                if points.is_positive() {
                    Some(TaskStatus::Completed)
                } else {
                    Some(TaskStatus::InProgress)
                }
            }
            None => {
                if task.dependencies().iter().all(|dep_id| {
                    self.answers
                        .get(dep_id)
                        .is_some_and(|points| points.is_positive())
                }) {
                    Some(TaskStatus::Available)
                } else {
                    Some(TaskStatus::NotAvailable)
                }
            }
        }
    }

    pub fn completed_tasks(&self) -> Vec<&'a Task> {
        self.track
            .tasks
            .values()
            .filter(|&t| matches!(self.task_status(&t.id()), Some(TaskStatus::Completed)))
            .collect()
    }

    pub fn available_and_in_progress_tasks(&self) -> Vec<&'a Task> {
        self.track
            .tasks
            .values()
            .filter(|&t| {
                matches!(
                    self.task_status(&t.id()),
                    Some(TaskStatus::Available) | Some(TaskStatus::InProgress)
                )
            })
            .collect()
    }

    pub fn max_points(&self) -> Points {
        self.track
            .tasks
            .values()
            .fold(Points::zero(), |acc, t| acc + t.points())
    }

    pub fn points(&self) -> Points {
        self.answers
            .values()
            .fold(Points::zero(), |acc, &a| acc + a)
    }

    pub fn percent(&self) -> f32 {
        let max = self.max_points();
        if max.is_zero() {
            0.0
        } else {
            (self.points().as_i32() as f32) / (max.as_i32() as f32)
        }
    }

    pub fn full_completed(&self) -> bool {
        self.track.tasks.values().all(|t| {
            self.task_status(&t.id())
                .is_some_and(|t| matches!(t, TaskStatus::Completed))
        })
    }
}
