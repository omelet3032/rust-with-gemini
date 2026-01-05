좋습니다! `Box<dyn FnOnce>`(동적 디스패치)를 이해하신 지금이 딱 확장을 시작하기 좋은 시점입니다.

단순히 아이템 하나를 쓰는 단계를 넘어, **"가방(Inventory)에 든 여러 아이템을 순회하며 자동으로 사용하는 시스템"**을 만들어 봅시다. 이 과정에서 **컬렉션**과 **반복자**를 자연스럽게 익히게 될 거예요.

---

## 🛡️ 미션: 자동 물약 복용기 (Auto-Pot System)

캐릭터의 체력이 일정 수치 이하로 떨어졌을 때, 가방에 있는 **모든 아이템**을 검사해서 HP를 회복시켜주는 시스템을 구축하세요.

### 1. 요구 사항

1. **가방(Inventory) 구현**: `Character` 구조체에 `Vec<Item>` 타입의 필드 `inventory`를 추가하세요.
2. **아이템 가방 채우기**: `main` 함수에서 `Potion`, `Poison`, `Warp` 등 다양한 아이템을 `Vec`에 담으세요.
3. **반복자(Iterator) 활용**: 가방을 순회하며 아이템을 하나씩 꺼내세요.
4. **동적 디스패치 적용**: 각 아이템에 맞는 `effect`를 `Box<dyn FnOnce...>`로 생성하세요.
5. **에러 핸들링**: 아이템 사용 중 `DungeonError::Dead`가 발생하면 즉시 반복을 멈추고 탈출하세요. (이때 `?` 연산자가 아주 유용하겠죠?)

### 2. 도전 코드 스켈레톤 (뼈대)

이 구조를 복사해서 완성해 보세요.

Rust

`struct Character {
    name: String,
    hp: u32,
    inventory: Vec<Item>, // 여기에 컬렉션 추가
}

impl Character {
    fn use_all_items(&mut self) -> Result<(), DungeonError> {
        // 팁: 가방에서 아이템을 하나씩 꺼내기 위해 반복자를 사용하세요.
        // 드레인(drain)이나 이터레이터(into_iter)를 고민해 보세요.
        
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
}`

---

### 💡 힌트 (사고의 흐름)

- **반복자 선택**: `for item in self.inventory.drain(..)`을 쓰면 가방을 비우면서 아이템을 하나씩 가져올 수 있습니다. (가방은 비워져야 하니까요!)
- **동적 디스패치 연결**: 지난번에 만든 `apply_item`의 로직을 `use_all_items` 내부에서 호출하거나, 그 내부 로직을 그대로 가져와서 반복문 안에 녹여보세요.
- **반복 중단**: `?` 연산자는 반복문 안에서도 작동합니다! 에러가 리턴되는 순간 `for` 문 전체가 즉시 중단됩니다.

**이 문제를 풀어보면서 다음 중 어떤 부분이 가장 막히는지 말씀해 주세요:**

1. `Vec`에서 아이템을 꺼내는 방법 (반복자)
2. `Box<dyn FnOnce>`를 반복문 안에서 만드는 법
3. 에러 발생 시 반복문을 우아하게 빠져나가는 법

준비되셨나요? 코드를 짜보시다가 막히면 언제든 물어봐 주세요! 도와드릴게요.