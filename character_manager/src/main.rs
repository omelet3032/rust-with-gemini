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
            }
            Item::Poison(_) => {
                target.hp = effect(target.hp);
                if target.hp == 0 {
                    Err(DungeonError::Dead)
                } else {
                    Ok(())
                }
            }
            Item::Warp => Err(DungeonError::NotSupported),
        }
    }
}

// fn main() {
//     let mut warrior = Character {
//         name: "Warrior",
//         hp: 50,
//     };
//     // 시나리오 1: 포션 사용 (+30) -> HP 80

//     let potion_value = 30;
//     let _ = Item::Potion(30).use_on(&mut warrior, |hp| hp + potion_value);

//     println!("final_hp : {}", warrior.hp);

//     // 시나리오 2: 강력한 독 사용 (-100) -> Dead 에러 발생 확인

//     // let _ = Item::Poison(100).use_on(&mut warrior, |hp| hp.saturating_sub(100));
//     let result = Item::Poison(100).use_on(&mut warrior, |hp| hp.saturating_sub(100));

//     match result {
//         Ok(_) => println!("final hp2 : {}", warrior.hp),
//         Err(DungeonError::Dead) => println!("Dead에러 발생"),
//         Err(_) => {}
//     }

//     // Item::Poison(100).use_on(&mut warrior, |hp| hp.saturating_sub(100)).unwrap();
//     // 시나리오 3: Warp 사용 -> NotSupported 에러 확인
//     let result2 = Item::Warp.use_on(&mut warrior, |s| s);
//     if let Err(DungeonError::NotSupported) = result2 {
//         println!("지원되지 않음");
//     } else {
//         println!("hp : {}", warrior.hp);
//     }
//     // ※ 팁: 에러 발생 시 '?' 연산자를 쓰려면 별도의 함수(apply_item 등)를 만드세요.
// }
fn main() {
    let mut warrior = Character {
        name: "warrior",
        hp: 50,
    };

    let potion = Item::Potion(30);
    let poison = Item::Poison(100);
    let warp = Item::Warp;

    println!("최초 warrior hp : {}", warrior.hp);

    let _ = apply_item(potion, &mut warrior);
    println!("potion warrior hp: {}", warrior.hp);
    
    // let _ = apply_item(poison, &mut warrior);
    if let Err(DungeonError::Dead) = apply_item(poison, &mut warrior) {
        println!("죽었습니다");
    } else {
        println!("poison warrior hp: {}", warrior.hp);
    }

    if let Err(DungeonError::NotSupported) = apply_item(warp, &mut warrior) {
        println!("warp");
    } else {
        println!("순간이동");
    }

}

fn apply_item(item: Item, warrior: &mut Character) -> Result<(), DungeonError> {
    // effect 클로저는 상황에 따라 다르니까 apply_item의 매개변수로 넣지 말고 내부에서 상황별로 effect를 정의하자

    // C. 코드 간소화 제안 (가독성) match 문 안에서 item.use_on을 반복 호출하고 있는데,
    // 만약 나중에 Item의 종류가 20개가 된다면 코드가 길어질 수 있습니다.

    // 생각해볼 거리: > "아이템이 수치(value)를 가지고 있다면, 그 수치를 먼저 계산해서 뽑아낸 뒤에 use_on은 딱 한 번만 호출할 수는 없을까요?"
   
   
   let _ = match item {
        Item::Potion(value) => {
            let _ = item.use_on(warrior, |hp| hp+ value);
            item.use_on(warrior, |hp| hp + value)
        }
        Item::Poison(value) => {
            let _ = item.use_on(warrior, |hp| hp.saturating_sub(value))?;
            item.use_on(warrior, |hp| hp.saturating_sub(value))
        }
        Item::Warp => {
            item.use_on(warrior, std::convert::identity)
        }
    }?;
    /* 
        1. match에서 클로저를 다양한 타입으로 받기
        2. value를 추출하기
     */

    // 1. 클로저 동적 디스패치
    let effect: Box<dyn FnMut(u32) -> u32> = match item {
        Item::Potion(v) => Box::new(move |hp| hp + v),
        Item::Poison(v) => Box::new(move |hp| hp.saturating_sub(v)),
        Item::Warp => Box::new(std::convert::identity),
    };

    item.use_on(warrior, effect)?;

    // 2. value 추출


    Ok(())
}
