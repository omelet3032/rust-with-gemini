use std::fmt::{self, Display};

// 1. Appliance Enum 정의 (Tv, AirConditioner, Light)
// (Display 트레이트도 직접 구현해 보세요. TV: "TV(채널)", AC: "에어컨(온도)", Light: "전등(상태)")
enum Appliance {
    Tv(u8),
    AirConditioner(i32),
    Light(bool),
}

impl Display for Appliance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Appliance::Tv(value) => write!(f, "{}", value),
            Appliance::AirConditioner(value) => write!(f, "{}", value),
            Appliance::Light(value) => write!(f, "{}", value),
        }
    }
}

// 2. Command 트레이트 정의 (제네릭 F 포함)
trait Command<F> {
    fn name(&self) -> String;
    fn execute(&self, appliance:Appliance, logic:F) -> Result<Appliance, String>;
}
// 3. SmartManager 구조체 및 트레이트 구현
// (클로저 F에 대한 제약 조건: FnOnce(u8) -> u8)
struct SmartManager {
    pub location: String,
}

/* 
    SmartManager의 필드는 location인데 trait은 name?
*/
impl<F> Command<F> for SmartManager 
where
F: FnOnce(u8) -> u8
{
    fn name(&self) -> String {
        format!("SmartManager : {}", &self.location)
    }

    fn execute(&self, appliance:Appliance, logic:F) -> Result<Appliance, String> {
        if let Appliance::Tv(num) = appliance {
            Ok(Appliance::Tv(logic(num)))
        } else {
            Err(String::from("관리 대상 가전이 아닙니다."))
        }
    }

}

// 4. run_command 제네릭 함수 작성
// (P: Command<F> 등 필요한 제약 조건을 모두 찾아내어 작성하세요)

// enum struct closer
fn run_command<P,F>(appliance:Appliance, manager:P, logic:F) -> Result<Appliance, String> 
where 
P: Command<F>,
F: FnOnce(u8) -> u8
{
    manager.execute(appliance, logic)
}

fn main() {
    // 5. 시나리오 테스트
    // - "Living Room" 매니저 생성
    // - 7번 채널이 틀어진 TV 생성

    // - 클로저 정의: 현재 채널에 +10을 하는 로직

    // - run_command 호출 및 결과 확인

    // - (선택) 에어컨 데이터를 넣었을 때 에러가 정상적으로 출력되는지도 확인해 보세요.
}
