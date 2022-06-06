use dom::*;

#[test]
fn test_house_with_rooms() {
    let house_title = "house".to_string();
    let room1_title = "room1".to_string();
    let room2_title = "room2".to_string();

    let mut house = smart_house::House::new(house_title);
    let room1 = smart_house::Room::new(room1_title.clone());
    let room2 = smart_house::Room::new(room2_title.clone());

    {
        assert_eq!(house.rooms.len(), 0);
        house.insert_room(room1_title.clone(), room1);
        house.insert_room(room2_title.clone(), room2);
        assert_eq!(house.rooms.len(), 2);
    }

    {
        let inserted_room = house.get_room(&room1_title);
        assert!(inserted_room.is_some());

        assert_eq!(inserted_room.unwrap().title, room1_title);
        assert_eq!(inserted_room.unwrap().devices.len(), 0);

        let not_inserted_room = house.get_room("non_existing_room");
        assert!(not_inserted_room.is_none());
    }

    {
        let inserted_room = house.get_room_mut(&room1_title);
        assert!(inserted_room.is_some());

        inserted_room.unwrap().title = "updated_title".to_string();

        let inserted_room = house.get_room(&room1_title);

        assert_eq!(inserted_room.unwrap().title, "updated_title");
        assert_eq!(inserted_room.unwrap().devices.len(), 0);
    }

    {
        house.remove_room(room1_title);
        assert_eq!(house.rooms.len(), 1);

        house.remove_room(room2_title);
        assert_eq!(house.rooms.len(), 0)
    }
}

#[test]
fn test_room_with_devices() {
    let room_title = "room".to_string();
    let device1_title = "device1".to_string();
    let device2_title = "device2".to_string();

    let mut room = smart_house::Room::new(room_title);
    let device1 = smart_teapot::Teapot::new();
    let device2 = smart_teapot::Teapot::new();

    {
        assert_eq!(room.devices.len(), 0);
        room.insert_device(device1_title.clone(), Box::new(device1));
        room.insert_device(device2_title.clone(), Box::new(device2));
        assert_eq!(room.devices.len(), 2);
    }

    {
        let inserted_device = room.get_device(&device1_title);
        assert!(inserted_device.is_some());

        let inserted_device = room.get_device("non_existing_device");
        assert!(inserted_device.is_none());
    }

    {
        room.remove_device(device1_title);
        assert_eq!(room.devices.len(), 1);

        room.remove_device(device2_title);
        assert_eq!(room.devices.len(), 0);
    }
}
