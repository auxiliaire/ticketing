use entity::users;
use sea_orm::{Database, DbConn, DbErr, EntityTrait, Set};
use users::Entity as User;

mod common;

#[tokio::test]
async fn test_user() -> Result<(), DbErr> {
    // Connecting SQLite
    let db = Database::connect("sqlite::memory:").await?;

    // Setup database schema
    common::setup_schema(&db).await?;

    // Performing tests
    testcase(&db).await?;

    Ok(())
}

async fn testcase(db: &DbConn) -> Result<(), DbErr> {
    let user = users::ActiveModel {
        name: Set("Alice".to_owned()),
        password: Set("secret".to_owned()),
        role: Set("User".to_owned()),
        ..Default::default()
    };

    let user_insert_res = User::insert(user)
        .exec(db)
        .await
        .expect("could not insert user");

    assert_eq!(
        user_insert_res.last_insert_id, 1,
        "Id should be filled after insert."
    );

    Ok(())
}
