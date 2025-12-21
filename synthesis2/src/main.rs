use std::fmt::{self, Display};

#[derive(Debug)]
pub enum SecureData {
    Sensitive(String), // 암호화가 필요한 텍스트
    Public(String),    // 일반 텍스트
    Key(u8),           // 암호화 키
}

// --------------------------------------------------
// 1. [직접 작성] 트레이트 정의 (DataProcessor)
// --------------------------------------------------
// 요구사항:
// - 제네릭 타입 T를 가집니다.
// - id(&self) 메서드는 &str을 반환합니다.
// - process(&self, data: SecureData, func: T) 메서드는 Result<SecureData, String>을 반환합니다.

pub trait DataProcessor<T> {
    // 여기에 트레이트 명세 작성
    fn id(&self) -> &str;
    fn process(&self, data: SecureData, func: T) -> Result<SecureData, String>;
}

// --------------------------------------------------
// 2. [직접 작성] 구조체 및 구현 (MessageEncoder)
// --------------------------------------------------
pub struct MessageEncoder {
    pub code_name: String,
}

impl<T> DataProcessor<T> for MessageEncoder 
where 
T: FnOnce(String) -> String,
    // T에 대한 제약 조건을 작성하세요. (String을 받아 String을 반환하는 FnOnce)
{
    // id 메서드 구현 (&self.code_name 활용)
    fn id(&self) -> &str {
        println!("id : {}", &self.code_name);
        &self.code_name
    }
    
    fn process(&self, data: SecureData, func: T) -> Result<SecureData, String> {
        let result = match data {
            Ok(s) => {
                SecureData::Sensitive(func(s))
            },
            Err(_) => {
                String::from("Encryption failed: Not sensitive data")
            }
        };
        result
    }
    // process 메서드 구현:
    // 1. match로 data를 검사합니다.
    // 2. SecureData::Sensitive(s)인 경우에만 func(s)를 실행하고 결과를 다시 Sensitive에 담아 Ok 반환.
    // 3. 나머지는 Err("Encryption failed: Not sensitive data") 반환.
}

// --------------------------------------------------
// 3. [직접 작성] 실행 함수 (execute_task)
// --------------------------------------------------
// 제네릭 P와 T를 사용하여 run_pipeline과 유사한 함수를 만드세요.
pub fn execute_task<P, T>(data: SecureData, processor: P, task: T) -> Result<SecureData, String>
where
    // P와 T에 필요한 트레이트 경계를 모두 작성하세요.
{
    // 로직: 프로세서 ID 출력 후 process 실행 결과 반환
}

// --------------------------------------------------
// 4. [직접 작성] Display 트레이트 구현 (SecureData)
// --------------------------------------------------
impl Display for SecureData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match를 사용해 각 유닛을 출력 가능하게 만드세요.
        // Sensitive는 "****"라고 출력되게 하고, 나머지는 실제 값을 출력하세요.
    }
}

// --------------------------------------------------
// 5. [직접 작성] 메인 함수 (테스트)
// --------------------------------------------------
fn main() {
    // 1. 인스턴스 생성: "X-Shadow"라는 이름을 가진 MessageEncoder
    
    // 2. 데이터 생성: SecureData::Sensitive 로 "Top Secret Message" 생성
    
    // 3. 클로저 생성: 
    //    - 소유권(String)을 받습니다.
    //    - 문자열 앞뒤에 "---"를 붙이고 모두 대문자로 바꾸는 로직을 작성하세요.
    //    - 힌트: format!("---{}---", s.to_uppercase()) 

    // 4. execute_task 호출 및 결과 매칭 출력 (Display 덕분에 {}로 출력 가능해야 함)
    
}