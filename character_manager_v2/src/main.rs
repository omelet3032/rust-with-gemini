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

#[derive(Debug)]
enum DungeonError {
    Dead,
    NotSupported,
}

trait Usable {
    fn use_on<F>(&self, character: &mut Character, effect:F) -> Result<(),DungeonError>
    where 
        F: FnOnce(u32) -> u32;
}

impl Usable for Item {
    fn use_on<F>(&self, character: &mut Character, effect:F) -> Result<(),DungeonError>
    where 
       F: FnOnce(u32) -> u32 
    {

        match self {
            Item::Potion(_) => {
                character.hp = effect(character.hp);
                Ok(())
            },
            Item::Poison(_) => {
                character.hp = effect(character.hp);
                if character.hp <= 0 {
                    Err(DungeonError::Dead)
                } else {
                    Ok(())
                }
            },
            Self::Warp => {
                Err(DungeonError::NotSupported)
            }
        }


    } 

}

impl Character {
    fn use_all_items(&mut self) -> Result<(), DungeonError> {
        // 팁: 가방에서 아이템을 하나씩 꺼내기 위해 반복자를 사용하세요.
        // 드레인(drain)이나 이터레이터(into_iter)를 고민해 보세요.
        
        self.inventory.into_iter().

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