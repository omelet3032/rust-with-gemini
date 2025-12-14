// --------------------
// 1. 사용자 정의 열거형 (Enum)
// --------------------
#[derive(Debug)]
enum UserAction {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Scroll(i32), // 스크롤 방향 (양수: 아래, 음수: 위)
    NoAction,
}

// --------------------
// 2. 핵심 함수 (작성 필요)
// --------------------

fn process_action(action: UserAction) {
    // 💡 요구사항 A: Click 액션일 경우에만 x, y 좌표를 출력하세요. (if let 사용)
    // 나머지 액션은 무시합니다.
    println!("\n--- [A] if let: Click 액션 처리 ---");
    // 여기에 코드 작성
    if let UserAction::Click { x, y } = action {
        println! {"x : {}, y : {}", x, y};
    }

    // 💡 요구사항 B: 모든 UserAction 타입을 처리하세요. (match 사용)
    // - KeyPress일 경우: 어떤 키가 눌렸는지 출력
    // - Scroll일 경우: 스크롤 방향(양수/음수)에 따라 "스크롤 다운" 또는 "스크롤 업" 출력
    // - NoAction일 경우: "대기 중" 출력
    // - Click일 경우: "좌표 ({}, {})에서 클릭 이벤트 발생" 출력
    println!("\n--- [B] match: 모든 액션 처리 ---");
    // 여기에 코드 작성
    match action {
        UserAction::KeyPress(char) => println!("{}", char),
        UserAction::Scroll(value) => {
            if value > 0 {
                println!("스크롤 다운");
            } else {
                println!("스크롤 업");
            }
        }
        UserAction::NoAction => println!("대기 중"),
        UserAction::Click { x, y } => {
            println!("좌표 ({},{})에서 클릭 이벤트 발생", x,y);
        }
    }
}

// --------------------
// 3. Option<T> 연습
// --------------------

fn check_optional_id(optional_id: Option<u32>) {
    // 💡 요구사항 C: optional_id에 값이 있을 경우(Some)에만 해당 ID를 출력하세요. (if let Some 사용)
    // 값이 없을 경우(None)에는 아무것도 하지 않습니다.
    println!("\n--- [C] if let Some: 옵셔널 ID 처리 ---");
    // 여기에 코드 작성
    if let Some(value) = optional_id {
        println!("ID : {}", value);
    }
}

// --------------------
// 4. Result<T, E> 연습
// --------------------

#[derive(Debug)]
enum DatabaseError {
    NotFound,
    AccessDenied,
}

// Result를 반환하는 가상의 함수
fn fetch_user_settings(user_id: u32) -> Result<String, DatabaseError> {
    match user_id {
        1 => Ok(String::from("Theme: Dark")),
        2 => Err(DatabaseError::AccessDenied),
        _ => Err(DatabaseError::NotFound),
    }
}

fn process_result_setting(user_id: u32) {
    let result = fetch_user_settings(user_id);

    // 💡 요구사항 D: result가 성공(Ok)했을 경우에만 설정값(String)을 출력하세요. (if let Ok 사용)
    // 실패(Err)했을 경우에는 아무것도 하지 않습니다.
    println!("\n--- [D] if let Ok: Result 처리 ---");
    // 여기에 코드 작성
    if let Ok(ref value) = result {
        println!("설정값: {}", value);
    } else { 
    }

    // 💡 요구사항 E: Result의 모든 경우를 처리하세요. (match 사용)
    // - 성공(Ok)일 경우: "설정 로드 완료: {}" 출력
    // - 실패(Err)일 경우: DatabaseError 타입에 따라 "DB 오류: 찾을 수 없음" 또는 "DB 오류: 접근 거부" 출력
    println!("\n--- [E] match: 모든 Result 처리 ---");
    // 여기에 코드 작성
    match result {
        Ok(value) => println!{"설정 로드 완료: {}", value},
        Err(DatabaseError::NotFound) => println!("DB 오류: 찾을 수 없음"),
        Err(DatabaseError::AccessDenied) => println!("DB 오류 : 접근 거부"),
        
    }
}

fn main() {
    // 요구사항 A, B 실행
    process_action(UserAction::Click { x: 10, y: 20 });
    process_action(UserAction::KeyPress('A'));
    process_action(UserAction::Scroll(-100));

    // 요구사항 C 실행
    check_optional_id(Some(500));
    check_optional_id(None);

    // 요구사항 D, E 실행
    process_result_setting(1); // 성공
    process_result_setting(2); // AccessDenied
    process_result_setting(3); // NotFound
}
