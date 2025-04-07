use std::sync::Arc;

use anyhow::Result;


use crate::domain::{entities::quests::{AddQuestEntity, EditQuestEntity}, repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository}};

use super::quest_viewing;


pub struct QuestOpsUseCase<T1,T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync
{
    quest_ops_repository : Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1,T2> QuestOpsUseCase<T1,T2>
where
T1: QuestOpsRepository + Send + Sync,
T2: QuestViewingRepository + Send + Sync
{
    pub fn new(quest_ops_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            quest_ops_repository,
            quest_viewing_repository
        }
    }

    async fn add(&self , add_quest_entity: AddQuestEntity) -> Result<i32> {
        unimplemented!()
    }
    async fn edit(&self, quest_id: i32, edit_quest_entity: EditQuestEntity) -> Result<i32>{
        unimplemented!()
    }
    async fn remove(&self, quest_id: i32, guild_commander_id: i32) -> Result<()>{
        unimplemented!()
    }
}

// Handler -> Axum
// UseCase -> Application
// Repository
