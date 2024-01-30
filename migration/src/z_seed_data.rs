use entities::{prelude::*, *};
use sea_orm::entity::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const QUESTIONS: &str = r#"[{
   "id": 1,
   "question": "Is the housing cracked?",
    "grade": "D",
    "next": 2
  }, {
"id": 2,
   "question": "Are there any dents or more than 20 scratches on the phone?",
    "grade": "C",
    "next": 3
}, {
"id": 3,
   "question": "Are there more than twenty minor scratches on the LCD?",
    "grade": "B",
    "next": 4
}, {
"id": 4,
   "question": "Are there any scratches on the housing?",
    "grade": "A+",
    "next": 5
}, {
"id": 5,
   "question": "Are there more than 20 minor scratches on the housing?",
    "grade": "B",
    "next": null
}]"#;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let questions: serde_json::Value = serde_json::from_str(QUESTIONS).unwrap();
        let mut questions = questions.as_array().unwrap().to_owned();
        while let Some(question) = questions.pop() {
            let ques = questions::ActiveModel {
                id: Set(question["id"].as_i64().unwrap() as i32),
                question: Set(question["description"].to_owned().to_string()),
                grade: Set(question["grade"].to_string()),
                next: Set(Some(question["next"].as_i64().unwrap() as i32)),
            };
            let _ = Questions::insert(ques).exec(db).await?.last_insert_id;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        Devices::delete_many().exec(db).await?;

        Ok(())
    }
}
