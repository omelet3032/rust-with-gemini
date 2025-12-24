use std::fmt::{self, Display};

// [1] 커스텀 에러 정의
#[derive(Debug)]
pub enum HomeError {
    InvalidAppliance(String),
    SafetyLock,
}

impl Display for HomeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HomeError::InvalidAppliance(m) => write!(f, "가전 오류: {}", m),
            HomeError::SafetyLock => write!(f, "안전 제한: 허용 범위를 벗어났습니다."),
        }
    }
}

#[derive(Debug)] // 출력을 위해 추가
pub enum Appliance {
    Tv(u8),
    AirConditioner(i32),
}

pub trait Command {
    fn name(&self) -> &str;

    // 이 메서드가 반환하는 Result의 에러 타입을 HomeError로 바꿨습니다.
    fn execute<F>(&self, appliance: Appliance, logic: F) -> Result<Appliance, HomeError>
    where
        F: FnOnce(u8) -> u8;
}

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
        // 🔥 MISSION 1: 직접 로직을 완성하세요!
        // 1. match나 if let으로 appliance가 Tv인지 확인하세요.
        // 2. Tv가 아니라면 HomeError::InvalidAppliance를 Err에 담아 반환하세요.
        // 3. Tv라면 logic(채널)을 실행하고, 그 결과가 200보다 크면 HomeError::SafetyLock을 반환하세요.
        // 4. 모든 조건이 맞으면 Ok(Appliance::Tv(결과))를 반환하세요.

        /* 여기에 코드 작성 */
        
        if let Appliance::Tv(s) = appliance {
            let result = logic(s);
            if result >= 200 {
               return Err(HomeError::SafetyLock)
            }
            println!("{}", result);
            Ok(Appliance::Tv(result))
        } else {
            Err(HomeError::InvalidAppliance(String::from("에러")))
        }
        /* 
            클로저 타입 확인 잘하자 소유권 문제 FnOnce
         */
    }
}

// [5] run_command (매니저를 참조 &P로 받음)
fn run_command<P, F>(appliance: Appliance, manager: &P, logic: F) -> Result<Appliance, HomeError>
where
    P: Command,
    F: FnOnce(u8) -> u8,
{
    // 🔥 MISSION 2: '?' 연산자를 사용해 보세요.
    // 1. manager.name()을 출력합니다.
    // 2. manager.execute를 호출하되, 결과 뒤에 '?'를 붙여서 에러 발생 시 즉시 리턴하게 만드세요.
    // 3. 최종 성공 결과를 Ok()로 감싸서 반환하세요.

    /* 여기에 코드 작성 */
    println!("name: {}", manager.name());
    let result = manager.execute(appliance, logic)?;
    println!("result : {:?}", result);
    
    Ok(result)

}

fn main() {
    let loc = "Living Room";
    let manager = SmartManager { location: loc };

    // 테스트 1: 정상 작동
    println!("--- 테스트 1 (정상) ---");
    let result1 = run_command(Appliance::Tv(10), &manager, |c| c + 20);
    println!("결과: {:?}", result1);

    // 테스트 2: 안전 제한 (200 이상)
    println!("\n--- 테스트 2 (안전 제한) ---");
    let result2 = run_command(Appliance::Tv(150), &manager, |c| c + 100);
    println!("결과: {:?}", result2);

    // 테스트 3: 대상 아님 (에어컨)
    println!("\n--- 테스트 3 (대상 아님) ---");
    let result3 = run_command(Appliance::AirConditioner(24), &manager, |c| c);
    println!("결과: {:?}", result3);
}
