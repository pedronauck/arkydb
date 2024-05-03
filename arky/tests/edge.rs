use arky::edge::prelude::*;
use arky::node::prelude::*;

#[schema(EdgeData)]
struct Owns {
    pub since: u32,
}

#[schema(Node)]
struct User {
    pub id: NodeID,
    pub name: String,
    pub age: u32,
    pub cars: EdgeRef,
}

#[schema(Node)]
struct Car {
    pub id: NodeID,
    pub name: String,
    pub model: String,
    pub owner: EdgeRef,
}

#[test]
fn creating_edge() {
    let mut user_cars_edge = EdgeList::new("user_owns");
    let mut car_users_edge = Edge::new("car_owned_by");

    let user = User::new(User {
        id: NodeID::new(),
        name: "John".to_string(),
        age: 20,
        cars: EdgeRef::new(&user_cars_edge),
    });
    let car = Car::new(Car {
        id: NodeID::new(),
        name: "Ford".to_string(),
        model: "Mustang".to_string(),
        owner: EdgeRef::new(&car_users_edge),
    });

    let owns_data = Owns::new(Owns { since: 2019 });
    user_cars_edge.link(&user, &car, owns_data.clone());
    car_users_edge.link(&car, &user, owns_data.clone());

    let user_data = Owns::get(&user_cars_edge.items[0].data).unwrap();
    assert_eq!(user_cars_edge.label, "user_owns");
    assert_eq!(user_cars_edge.items.len(), 1);
    assert_eq!(user_cars_edge.items[0].label, "user_owns");
    assert_eq!(user_cars_edge.items[0].from, user.key());
    assert_eq!(user_cars_edge.items[0].to, car.key());
    assert_eq!(user_data.since, 2019);

    let car_users_edge_item = car_users_edge.item.as_ref().unwrap();
    let car_data = Owns::get(&car_users_edge_item.data).unwrap();
    assert_eq!(car_users_edge.label, "car_owned_by");
    assert_eq!(car_users_edge_item.label, "car_owned_by");
    assert_eq!(car_users_edge_item.from, car.key());
    assert_eq!(car_users_edge_item.to, user.key());
    assert_eq!(car_data.since, 2019);
}

#[test]
fn unlinking_edge() {
    let mut user_cars_edge = EdgeList::new("user_owns");
    let mut car_users_edge = Edge::new("car_owned_by");

    let user = User::new(User {
        id: NodeID::new(),
        name: "John".to_string(),
        age: 20,
        cars: EdgeRef::new(&user_cars_edge),
    });
    let car = Car::new(Car {
        id: NodeID::new(),
        name: "Ford".to_string(),
        model: "Mustang".to_string(),
        owner: EdgeRef::new(&car_users_edge),
    });

    let owns_data = Owns::new(Owns { since: 2019 });
    user_cars_edge.link(&user, &car, owns_data.clone());
    car_users_edge.link(&car, &user, owns_data.clone());
    user_cars_edge.unlink(&user, &car).unwrap();
    car_users_edge.unlink();

    assert_eq!(user_cars_edge.items.len(), 0);
    assert_eq!(car_users_edge.item, None);
}

#[test]
fn edge_with_data_none() {
    let mut user_cars_edge = EdgeList::new("user_owns");
    let mut car_users_edge = Edge::new("car_owned_by");

    let user = User::new(User {
        id: NodeID::new(),
        name: "John".to_string(),
        age: 20,
        cars: EdgeRef::new(&user_cars_edge),
    });
    let car = Car::new(Car {
        id: NodeID::new(),
        name: "Ford".to_string(),
        model: "Mustang".to_string(),
        owner: EdgeRef::new(&car_users_edge),
    });

    user_cars_edge.link(&user, &car, Data::None);
    car_users_edge.link(&car, &user, Data::None);

    assert_eq!(user_cars_edge.items[0].data, Data::None);
    assert_eq!(car_users_edge.item.as_ref().unwrap().data, Data::None);
}

#[test]
#[should_panic]
fn unlinking_on_empty_list() {
    let mut user_cars_edge = EdgeList::new("user_owns");
    let car_users_edge = Edge::new("car_owned_by");

    let user = User::new(User {
        id: NodeID::new(),
        name: "John".to_string(),
        age: 20,
        cars: EdgeRef::new(&user_cars_edge),
    });
    let car = Car::new(Car {
        id: NodeID::new(),
        name: "Ford".to_string(),
        model: "Mustang".to_string(),
        owner: EdgeRef::new(&car_users_edge),
    });

    user_cars_edge.unlink(&user, &car).unwrap();
}

#[test]
#[should_panic]
fn unlinking_from_wrong_node_id() {
    let mut user_cars_edge = EdgeList::new("user_owns");
    let mut car_users_edge = Edge::new("car_owned_by");

    let user = User::new(User {
        id: NodeID::new(),
        name: "John".to_string(),
        age: 20,
        cars: EdgeRef::new(&user_cars_edge),
    });
    let car = Car::new(Car {
        id: NodeID::new(),
        name: "Ford".to_string(),
        model: "Mustang".to_string(),
        owner: EdgeRef::new(&car_users_edge),
    });

    let owns_data = Owns::new(Owns { since: 2019 });
    user_cars_edge.link(&user, &car, owns_data.clone());
    car_users_edge.link(&car, &user, owns_data.clone());
    user_cars_edge
        .unlink(
            &user,
            &Car::new(Car {
                id: NodeID::new(),
                name: "Ford".to_string(),
                model: "Mustang".to_string(),
                owner: EdgeRef::new(&car_users_edge),
            }),
        )
        .unwrap();
}
