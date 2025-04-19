use std::sync::Arc;

use anyhow::Result;


use crate::domain::{entities::quests::{AddQuestEntity, EditQuestEntity}, repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository}, value_objects::quest_model::AddQuestModel};

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

    pub async fn add(&self , guild_commander_id: i32,add_quest_model: AddQuestModel) -> Result<i32> {
        let add_quest_entity = add_quest_model.to_entity(guild_commander_id);

        let result = self.quest_ops_repository.add(add_quest_entity).await?;

        Ok(result)

    }
    pub async fn edit(&self, quest_id: i32, edit_quest_entity: EditQuestEntity) -> Result<i32>{
        unimplemented!()
    }
    pub async fn remove(&self, quest_id: i32, guild_commander_id: i32) -> Result<()>{
        unimplemented!()
    }
}

// Handler -> Axum
// UseCase -> Application
// Repository
