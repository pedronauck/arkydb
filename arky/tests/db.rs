use arky::edge::prelude::*;
use arky::entity::EntityItem;
use arky::inst::prelude::*;
use arky::node::prelude::*;
use tempdir::TempDir;

#[schema(EdgeData)]
struct Owns {
    pub since: u32,
}

#[schema(Node)]
struct User {
    pub id: NodeID,
    pub name: String,
    pub age: u32,
}

#[schema(Node)]
struct Car {
    pub id: NodeID,
    pub name: String,
    pub model: String,
    pub owner: EdgeRef,
}

fn create_storage() -> RocksDB {
    let dir = TempDir::new("arky").unwrap();
    let db_path = dir.path().join("test_db").to_str().unwrap().to_string();
    RocksDB::new(RocksDBConfig {
        path: db_path,
        set_error_if_exists: false,
        ..Default::default()
    })
}

#[tokio::test]
async fn init_db_with_storage() {
    let storage = create_storage();
    let db = ArkyDB::init(&storage);

    let user1 = User::new(User {
        id: NodeID::new(),
        name: "John".to_string(),
        age: 20,
    });

    let user2 = User::new(User {
        id: NodeID::new(),
        name: "Jane".to_string(),
        age: 20,
    });

    db.insert_node(&user1).await.ok();
    db.insert_node(&user2).await.ok();

    let found_user1 = &db.get_node::<User>(user1.id).await.unwrap();
    let found_user2 = &db.get_node::<User>(user2.id).await.unwrap();

    assert_eq!(&user1, found_user1);
    assert_eq!(&user2, found_user2);

    let entity_name = user1.entity();
    let entity = db.get_entity(&entity_name).await.unwrap();
    assert_eq!(entity_name, entity.name);
}

#[tokio::test]
async fn insert_entity_on_db() {
    let storage = create_storage();
    let db = ArkyDB::init(&storage);

    let entity_item = EntityItem::new("User".to_string());

    db.insert_entity(&entity_item).await.ok();

    let db_entity = db.get_entity("User").await.unwrap();
    assert_eq!(db_entity.name, "User");
}
