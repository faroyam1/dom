use dom::*;

fn main() {
    let house_title = "Дом 17 на улице 24".to_string();
    let room_title = "Кухня".to_string();
    let device_title = "Чайник".to_string();

    let mut house = smart_house::House::new(house_title);
    let room = smart_house::Room::new(room_title.clone());

    house.insert_room(room_title.clone(), room);

    house
        .get_room_mut(&room_title)
        .unwrap()
        .insert_device(device_title.clone(), Box::new(smart_teapot::Teapot::new()));

    println!(
        "{}",
        smart_house::HouseInfo::info(&house, room_title.clone(), device_title.clone()),
    );

    house
        .get_room_mut(&room_title)
        .unwrap()
        .remove_device(device_title);

    house.remove_room(room_title);
}
