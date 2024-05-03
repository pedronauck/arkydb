use arkycore::{types::*, utils};
use arkymacros_schema::schema;

trait Node {
    fn key(&self) -> NodeID;
    fn entity(&self) -> String;
    fn new<T: Node>(data: T) -> T {
        data
    }
}

#[schema(Node)]
struct Person {
    id: NodeID,
    name: String,
    age: u8,
}

#[test]
fn test_global_id() {
    let first_id = NodeID::new();
    let second_id = NodeID::new();
    let john = Person::new(Person {
        id: first_id,
        age: 30,
        name: String::from("John"),
    });
    let peter = Person::new(Person {
        id: second_id,
        age: 32,
        name: String::from("Peter"),
    });

    assert_eq!(john.id, first_id);
    assert_eq!(peter.id, second_id);
}

#[test]
fn test_schema_trait_impl() {
    let id = NodeID::new();
    let entity = Person {
        id,
        age: 30,
        name: String::from("John"),
    };

    let person = Person::new(entity);
    assert_eq!(person.entity(), utils::format_entity("Person"));
    assert_eq!(person.name, String::from("John"));
    assert_eq!(person.key(), id);
    assert_eq!(person.age, 30);
}
