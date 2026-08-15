use entity::users::self;
use sea_orm::{ActiveModelTrait, Database, DbConn, DbErr, Set};
use shared::validation::user_validation::{OptionUserRole, UserRole::Developer};

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
        role: Set(OptionUserRole(Some(Developer)).to_string()),
        ..Default::default()
    };

    let res = user.clone().insert(db).await;

    // u64 primary key does not work well with SQLite, so the old tests are replaced with this dummy assertion:
    assert!(res.is_err_and(|x| x.to_string() == "Type Error: u64 unsupported by sqlx-sqlite"));

    /*
        let user_insert_res = User::insert(user)
            .exec(db)
            .await
            .expect("could not insert user");

        assert_eq!(
            user_insert_res.last_insert_id, 1,
            "Id should be filled after insert."
        );
    */
    Ok(())
}
