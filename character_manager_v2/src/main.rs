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
        
        /* 
            로직
            character의 inventory를 순회하며 apply_item 함수를 적용후 hp를 추적한다.
            1. inventory를 순회하는 방법
                1. for in
                2. 반복자 메서드

         */
        
        let items:Vec<_>= self.inventory.drain(..).collect();

        println!("--- 가방 정리를 시작합니다 ---");
        
        // 여기에 반복자(iterator)와 apply_item 로직을 결합해보세요.
        for item in items {
            apply_item(item, self)?;
        }
        
        Ok(())
    }
}


fn apply_item(item: Item, warrior: &mut Character) -> Result<(), DungeonError> {
    // effect 클로저는 상황에 따라 다르니까 apply_item의 매개변수로 넣지 말고 내부에서 상황별로 effect를 정의하자

    // C. 코드 간소화 제안 (가독성) match 문 안에서 item.use_on을 반복 호출하고 있는데,
    // 만약 나중에 Item의 종류가 20개가 된다면 코드가 길어질 수 있습니다.

    // 생각해볼 거리: > "아이템이 수치(value)를 가지고 있다면, 그 수치를 먼저 계산해서 뽑아낸 뒤에 use_on은 딱 한 번만 호출할 수는 없을까요?"
   
   
//    let _ = match item {
//         Item::Potion(value) => {
//             let _ = item.use_on(warrior, |hp| hp+ value);
//             item.use_on(warrior, |hp| hp + value)
//         }
//         Item::Poison(value) => {
//             let _ = item.use_on(warrior, |hp| hp.saturating_sub(value))?;
//             item.use_on(warrior, |hp| hp.saturating_sub(value))
//         }
//         Item::Warp => {
//             item.use_on(warrior, std::convert::identity)
//         }
//     }?;
    /* 
        1. match에서 클로저를 다양한 타입으로 받기
        2. value를 추출하기
     */

    // 1. 클로저 동적 디스패치
    // let effect: Box<dyn FnMut(u32) -> u32> = match item {
    let effect: Box<dyn FnOnce(u32) -> u32> = match item {
        Item::Potion(v) => Box::new(move |hp| hp + v),
        Item::Poison(v) => Box::new(move |hp| hp.saturating_sub(v)),
        Item::Warp => Box::new(std::convert::identity),
    };

    item.use_on(warrior, effect)?;

    // 2. value 추출
    // let value = match item {
    //     Item::Potion(value) => value as i32,
    //     Item::Poison(value) => -(value as i32),
    //     Item::Warp => {0}
    // };

    // item.use_on(warrior, |hp| {
    //     if value >= 0 {hp + value as u32}
    //     else {hp.saturating_sub(value.abs() as u32)}
    // })?;

    Ok(())
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