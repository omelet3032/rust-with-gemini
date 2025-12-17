/* 
## 🚀 Rust 종합 심화 연습: 데이터 처리 파이프라인 (문자열 강화)

### 📋 문제 목표

다양한 데이터(특히 `String`)를 처리하고 변환하는 파이프라인을 구축하며, 트레이트, 제네릭, 클로저, 그리고 문자열 소유권 규칙을 모두 숙달합니다.

### 🧩 제공되는 기본 구조

(이전과 동일합니다. 아래 `impl` 블록과 함수 정의를 완성하세요.)

```rust */
use std::fmt::{self, Display};

// --------------------
// 1. 데이터 정의 (Enum)
// --------------------

/// 처리할 수 있는 데이터의 유형을 정의합니다.
#[derive(Debug, Clone)]
pub enum RawData {
    Text(String), // 텍스트 데이터 (String)
    Number(i64),  // 수치 데이터 (i64)
    Flag(bool),   // 상태 플래그 (Boolean)
}

// --------------------
// 2. 트레이트 정의 (Trait)
// --------------------

/// 데이터를 처리/변환하는 모든 프로세서가 구현해야 할 핵심 트레이트입니다.
/// 트레이트 레벨에 정의된 제네릭
/// 메서드 레벨에서 정해진 제네릭과는 구별해야 한
pub trait DataProcessor<T> {
    fn id(&self) -> &str;

    /// 데이터를 처리하는 핵심 메서드입니다.
    /// T는 클로저로, FnOnce 트레이트를 구현해야 합니다.
    /// 📌 힌트: 클로저는 String (소유권)을 받아 String (소유권)을 반환해야 합니다.
    fn process(&self, data: RawData, transform_fn: T) -> Result<RawData, String>;
}


// --------------------
// 3. 구조체 정의 (Struct)
// --------------------

/// 텍스트 데이터를 대문자로 변환하는 전용 프로세서입니다.
pub struct TextConverter {
    name: String,
    config: String,
}

impl TextConverter {
    pub fn new(name: &str, config: &str) -> Self {
        Self {
            name: name.to_string(),
            config: config.to_string(),
        }
    }
}

// --------------------
// 4. 핵심 함수 (작성 필요)
// --------------------

/// 데이터를 파이프라인에 전달하고 최종 결과를 반환하는 함수입니다.
pub fn run_pipeline<P, T>(data: RawData, processor: P, transform_fn: T) -> Result<RawData, String>
where
    // 여기에 제약 조건 작성
    // 📌 P: DataProcessor 트레이트를 구현해야 합니다.
    // 📌 T: 클로저 트레이트 (process 메서드와 일치하도록)를 구현해야 합니다.
    P: DataProcessor<T>,
    T: FnOnce(String) -> String
{
    // 여기에 코드를 작성하세요.
    // 1. 프로세서의 ID를 출력합니다.
    // 2. 프로세서의 process 메서드를 호출합니다.
    println!("id : {}", processor.id());

    processor.process(data, transform_fn)
}

// --------------------
// 5. 트레이트 구현 (작성 필요)
// --------------------

// 🚀 A. TextConverter에 DataProcessor 구현
impl<T> DataProcessor<T> for TextConverter
where
    T: FnOnce(String) -> String
    // 여기에 T의 클로저 트레이트 제약을 다시 한번 명확히 작성하세요.
    // 📌 T: String을 받아 String을 반환하는 FnOnce 클로저여야 합니다.
{
    // 여기에 id 메서드 구현
    fn id(&self) -> &str {
        &self.name
    }

    /// 소유권 이동을 활용하여 문자열을 처리합니다.
    fn process(&self, data: RawData, transform_fn: T) -> Result<RawData, String> {
        // 1. data를 매칭하여 RawData::Text(input_str)만 처리하세요.
        // 2. RawData::Text가 아닌 경우, 에러 메시지를 반환하세요. (예: "Invalid data type")
        // 3. RawData::Text일 경우, 내부 input_str (String)의 소유권을 클로저로 넘기세요.
        // 4. 클로저의 결과를 새로운 RawData::Text로 묶어 Ok로 반환하세요.
        
        // 여기에 코드 작성
        
        if let RawData::Text(input_str) = data {
            // transform_fn(input_str);
            Ok(RawData::Text(transform_fn(input_str)))
        } else {
            Err("Invaild data type".to_string())
        }
        
    }
}

// 🚀 E. RawData가 출력 가능하도록 Display 트레이트를 구현합니다.
impl Display for RawData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 여기에 코드 작성 (match를 사용하여 모든 RawData 유형을 출력)
        
        
        
        
        
    }
}

// --------------------
// 6. 메인 함수 (작성 필요)
// --------------------

fn main() {
    // 💡 요구사항 D1: 초기 데이터 생성 및 변환기 준비
    // 1. RawData::Text로 "  Data cleaning and analysis.  " (앞뒤 공백 포함)을 생성하세요.
    // 2. TextConverter 인스턴스를 생성하세요 (이름: "Cleaner", 설정: "Trim, Upper").
    
    // 여기에 코드 작성
    
    
    
    
    
    
    
    // 💡 요구사항 D2: 클로저 정의 (문자열 메서드 활용)
    // 3. 소유권을 받고 String을 반환하는 클로저를 정의하세요.
    // 📌 클로저 내부에서 문자열의 .trim()과 .to_uppercase() 메서드를 순서대로 활용하세요.
    // 📌 .trim()은 &str을 반환하므로, .to_uppercase()는 String을 반환합니다.
    
    // 여기에 클로저 정의
    let cleaning_transform = /* 여기에 클로저 정의 */ ;


    // 💡 요구사항 D3: 파이프라인 실행
    // 4. run_pipeline을 호출하고 결과를 출력하세요.
    let result = run_pipeline(initial_data, converter, cleaning_transform);
    
    match result {
        Ok(data) => println!("✅ Final Output: {}", data),
        Err(e) => println!("❌ Pipeline Error: {}", e),
    }

    // 💡 요구사항 D4: 에러 상황 테스트 (Number 타입)
    // 5. RawData::Number(123)을 생성하고 run_pipeline을 호출하여 에러를 확인하세요.
    
    // 여기에 코드 작성
    
    
    
    
}