use std::fmt::{self, Display};

// 1. Appliance Enum 정의 (Tv, AirConditioner, Light)
// (Display 트레이트도 직접 구현해 보세요. TV: "TV(채널)", AC: "에어컨(온도)", Light: "전등(상태)")
pub enum Appliance {
    Tv(u8),
    AirConditioner(i32),
    Light(bool),
}

impl Display for Appliance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Appliance::Tv(s) => write!(f, "{}", s),
            Appliance::AirConditioner(s) => write!(f, "{}", s),
            Appliance::Light(s) => write!(f, "{}", s),
        }
    }
}

// 2. Command 트레이트 정의 (제네릭 F 포함)
pub trait Command {
    fn name(&self) -> String;
    fn execute<F>(&self, appliance: Appliance, logic: F) -> Result<Appliance, String>
    where
        F: FnOnce(u8) -> u8;
}

// 3. SmartManager 구조체 및 트레이트 구현
// (클로저 F에 대한 제약 조건: FnOnce(u8) -> u8)
pub struct SmartManager {
    pub location: String,
}

impl Command for SmartManager {
    fn name(&self) -> String {
        self.location.clone()
    }

    fn execute<F>(&self, appliance: Appliance, logic: F) -> Result<Appliance, String>
    where
        F: FnOnce(u8) -> u8,
    {
        if let Appliance::Tv(s) = appliance {
            Ok(Appliance::Tv(logic(s)))
        } else {
            Err(String::from("관리 대상이 아닙니다"))
        }
    }
}
// 4. run_command 제네릭 함수 작성
// (P: Command<F> 등 필요한 제약 조건을 모두 찾아내어 작성하세요)
fn run_command<P, F>(appliance: Appliance, manager: P, logic: F) -> Result<Appliance, String>
where
    P: Command,
    F: FnOnce(u8) -> u8,
{
    println!("name : {}", manager.name());
    manager.execute(appliance, logic)
}

fn main() {
    // 5. 시나리오 테스트
    // - "Living Room" 매니저 생성
    // - 7번 채널이 틀어진 TV 생성

    let manager = SmartManager {
        location: String::from("Living Room"),
    };

    let appliance = Appliance::Tv(7);
    // - 클로저 정의: 현재 채널에 +10을 하는 로직

    let logic = |channel: u8| channel + 10;
    // - run_command 호출 및 결과 확인

    let result = run_command(appliance, manager, logic);
    match result {
        Ok(s) => println!("result : {}", s),
        Err(e) => eprintln!("err : {}", e),
    }
    // - (선택) 에어컨 데이터를 넣었을 때 에러가 정상적으로 출력되는지도 확인해 보세요.
    let airconditioner = Appliance::AirConditioner(30);
    let manager2 = SmartManager {
        location: String::from("Living Room"),
    };
    let logic2 = |channel: u8| channel + 10;
    let result2 = run_command(airconditioner, manager2, logic2);

    match result2 {
        Ok(s) => println!("result : {}", s),
        Err(e) => eprintln!("err : {}", e),
    }
}
