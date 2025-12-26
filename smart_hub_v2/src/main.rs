use std::fmt::{self, Display};

// 1. HomeError 구현 (InvalidDevice, SafetyAlert)
pub enum HomeError {
    InvalidDevice,
    SafetyAlert(u8),
}
// 2. Appliance Enum 구현
pub enum Appliance {
    Tv(u8),
    AirConditioner(i32),
}

impl Display for HomeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HomeError::InvalidDevice => write!(f, "유효하지 않은 기기입니다."),
            HomeError::SafetyAlert(s) => write!(f, "제한 숫자 초과 : {}", s),
        }
    }
}
impl Display for Appliance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Appliance::Tv(s) => write!(f, "tv 채널: {}", s),
            Appliance::AirConditioner(s) => write!(f, "에어컨 온도 : {}", s),
        }
    }
}

// 3. Command 트레이트 정의
// (execute는 메서드 레벨 제네릭 사용)
pub trait Command {
    fn name(&self) -> &str;
    fn execute<F>(&self, appliance: Appliance, logic: F) -> Result<Appliance, HomeError>
    where
        F: FnOnce(u8) -> u8;
}

// 4. SmartManager<'a> 구조체 정의 및 트레이트 구현
// (주의: Tv 채널이 200 이상이면 SafetyAlert 에러 반환)
// pub struct SmartManager<'a> {
//     pub location: &'a str,
// }

pub struct SmartManager<'a> {
    pub location: &'a str,
}

impl<'a> Command for SmartManager<'a> {
    fn name(&self) -> &str {
        self.location
    }

    fn execute<F>(&self, appliance: Appliance, logic: F) -> Result<Appliance, HomeError>
    where
        F: FnOnce(u8) -> u8,
    {
        if let Appliance::Tv(s) = appliance {
            let result = logic(s);
            if result >= 200 {
                Err(HomeError::SafetyAlert(result))
            } else {
                Ok(Appliance::Tv(result)) // u8은 copy타입이니까 result를 마음대로 쓸 수 있다!
            }
        } else {
            Err(HomeError::InvalidDevice)
        }
    }
}

// 5. run_command 함수 구현
// (매니저 참조 사용, ? 연산자 활용)
// fn run_command(manager:)
fn run_command<P, F>(manager: &P, appliance: Appliance, logic: F) -> Result<Appliance, HomeError>
where
    P: Command,
    F: FnOnce(u8) -> u8,
{
    // let result = Ok(manager.execute(appliance, logic)?);
    // // Ok(result)
    // result

    let result = manager.execute(appliance, logic)?;
    
    println!("{}", result);
    Ok(result)
}
fn main() {
    // 시나리오:
    // 1. "Kitchen" 매니저 생성 (문자열 리터럴 사용)
    let manager = SmartManager {
        location: "Kitchen",
    };

    // 2. 150번 채널 TV 생성
    let appliance = Appliance::Tv(100);
    // let appliance2 = Appliance::Tv(250);
    

    // 3. run_command로 채널을 +60 하는 클로저 실행 -> SafetyAlert 에러 확인
    let logic = |s: u8| s + 60;
    let result = run_command(&manager, appliance, logic);
    match result {
        Ok(s) => println!("result1 성공 // {}", s),
        Err(s) => println!("{}", s),
    }; // 4. 다시 같은 매니저를 사용하여 정상 범위의 명령 실행 -> 성공 확인

    // let result2 = run_command(&manager, appliance2, logic);
    // match result2 {
    //     Ok(s) => println!("result2 성공 : {}", s),
    //     Err(s) => println!("{}", s),
    // }
    // ※ 주의: manager의 소유권이 유지되어야 함!

}
