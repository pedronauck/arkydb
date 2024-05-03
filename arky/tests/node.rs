use arky::node::prelude::*;

#[schema(Node)]
struct User {
    pub id: NodeID,
    pub name: String,
    pub age: u32,
}

fn create_user() -> User {
    User::new(User {
        id: NodeID::new(),
        name: "John".to_string(),
        age: 20,
    })
}

#[test]
fn creating_node() {
    let user = create_user();
    assert_eq!(user.entity(), "entity::User");
    assert_eq!(user.name, "John");
    assert_eq!(user.age, 20);
}
