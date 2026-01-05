struct Character {
    name: String,
    hp: u32,
    inventory: Vec<Item>, // 여기에 컬렉션 추가
}

enum Item {
    Potion(u32),
    Poison(u32),
    Warp,
}

enum DungeonError {
    Dead,
    NotSupported,
}

impl Character {
    fn use_all_items(&mut self) -> Result<(), DungeonError> {
        // 팁: 가방에서 아이템을 하나씩 꺼내기 위해 반복자를 사용하세요.
        // 드레인(drain)이나 이터레이터(into_iter)를 고민해 보세요.
        
        for item in self.inventory.drain(..) {

        }
        println!("--- 가방 정리를 시작합니다 ---");

        // 여기에 반복자(iterator)와 apply_item 로직을 결합해보세요.
        
        Ok(())
    }
}

fn main() {
    let mut warrior = Character {
        name: "Cloud".to_string(),
        hp: 20,
        inventory: vec![
            Item::Potion(20),
            Item::Poison(5),
            Item::Warp,
            Item::Potion(50),
        ],
    };

    // 가방 안의 아이템 전체 사용 시도
    if let Err(e) = warrior.use_all_items() {
        println!("중단됨: {:?}", e);
    }
    
    println!("최종 HP: {}", warrior.hp);
}