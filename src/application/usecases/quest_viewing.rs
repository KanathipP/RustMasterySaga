use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    entities::quests::{AddQuestEntity, EditQuestEntity, QuestEntity},
    repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository}, value_objects::board_checking_filter::BoardCheckingFilter,
};

use super::quest_viewing;

pub struct QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    quest_viewing_repository: Arc<T>,
}

impl<T> QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_viewing_repository: Arc<T>) -> Self {
        Self {
            quest_viewing_repository,
        }
    }

    pub async fn view_details(&self, quest_id: i32) -> Result<QuestEntity> {
        unimplemented!()
    }
    pub async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        unimplemented!()
    }

}

// Handler -> Axum
// UseCase -> Application
// Repository
