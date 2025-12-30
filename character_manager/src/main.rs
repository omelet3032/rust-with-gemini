// 1. DungeonError (Dead, NotSupported)
#[derive(Debug)]
pub enum DungeonError {
    Dead,
    NotSupported,
}
// 2. Character<'a> { name: &'a str, hp: u32 }
pub struct Character<'a> {
    pub name: &'a str,
    pub hp: u32,
}
// 3. Item { Potion(u32), Poison(u32), Warp }
pub enum Item {
    Potion(u32),
    Poison(u32),
    Warp,
}

// 4. Usable 트레이트 정의
pub trait Usable {
    fn use_on<F>(&self, target: &mut Character, effect: F) -> Result<(), DungeonError>
    where
        F: FnOnce(u32) -> u32;
}
// 5. Item에 대해 Usable 구현
//    (logic 실행 후 target.hp를 업데이트하고, 0이면 Dead 에러 반환)
impl Usable for Item {
    fn use_on<F>(&self, target: &mut Character, effect: F) -> Result<(), DungeonError>
    where
        F: FnOnce(u32) -> u32,
    {
        
        match self {
            Item::Potion(_) => {
                target.hp = effect(target.hp);
                Ok(())
            },
            Item::Poison(_) => {
                target.hp = effect(target.hp);
                if target.hp == 0 {
                    Err(DungeonError::Dead)
                } else {
                    Ok(())
                }
            },
            Item::Warp => {
                Err(DungeonError::NotSupported)
            }
        }

    }
}

fn main() {
    let mut warrior = Character {
        name: "Warrior",
        hp: 50,
    };
    // 시나리오 1: 포션 사용 (+30) -> HP 80
    
    let potion_value = 30;
    let _ = Item::Potion(30).use_on(&mut warrior, |hp| hp + potion_value);

    println!("final_hp : {}", warrior.hp);
    
    // 시나리오 2: 강력한 독 사용 (-100) -> Dead 에러 발생 확인
    // let _ = Item::Poison(100).use_on(&mut warrior, |hp| hp.saturating_sub(100)); 
    let result = Item::Poison(100).use_on(&mut warrior, |hp| hp.saturating_sub(100)); 
    
    match result {
        Ok(_) => println!("final hp2 : {}", warrior.hp),
        Err(DungeonError::Dead) => println!("Dead에러 발생"),
        Err(_) => {},
    }

    // Item::Poison(100).use_on(&mut warrior, |hp| hp.saturating_sub(100)).unwrap();
    // 시나리오 3: Warp 사용 -> NotSupported 에러 확인
    let result2 = Item::Warp.use_on(&mut warrior, |s| s);
    if let Err(DungeonError::NotSupported) = result2 {
        println!("지원되지 않음");
    } else {
        println!("hp : {}", warrior.hp);
    }
    // ※ 팁: 에러 발생 시 '?' 연산자를 쓰려면 별도의 함수(apply_item 등)를 만드세요.
}
