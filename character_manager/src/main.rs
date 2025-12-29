// 1. DungeonError (Dead, NotSupported)
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
    Poision(u32),
    Warp,
}

// 4. Usable 트레이트 정의
// 5. Item에 대해 Usable 구현 
//    (logic 실행 후 target.hp를 업데이트하고, 0이면 Dead 에러 반환)

fn main() {
    let mut warrior = Character { name: "Warrior", hp: 50 };
    
    // 시나리오 1: 포션 사용 (+30) -> HP 80
    // 시나리오 2: 강력한 독 사용 (-100) -> Dead 에러 발생 확인
    // 시나리오 3: Warp 사용 -> NotSupported 에러 확인
    
    // ※ 팁: 에러 발생 시 '?' 연산자를 쓰려면 별도의 함수(apply_item 등)를 만드세요.
}