
struct User {
    id: u32,
    username: String,
    is_active: bool,
    age: u8,
}

// 사용자 목록을 생성하는 함수 (소유권을 반환)
fn get_user_data() -> Vec<User> {
    vec![
        User {
            id: 101,
            username: String::from("alice"),
            is_active: true,
            age: 25,
        },
        User {
            id: 102,
            username: String::from("bob"),
            is_active: false,
            age: 35,
        },
        User {
            id: 103,
            username: String::from("charlie"),
            is_active: true,
            age: 15,
        },
        User {
            id: 104,
            username: String::from("david"),
            is_active: true,
            age: 40,
        },
        User {
            id: 105,
            username: String::from("eve"),
            is_active: false,
            age: 22,
        },
    ]
}

// 함수는 Vec<User>의 소유권을 받습니다.
fn get_active_adult_names(user_list: Vec<User>) -> Vec<String> {
    // 📝 여기에 코드를 작성하세요.
    // 1. user_list를 참조로 순회 (for item in &vec 형태의 내부 구현)
    // 2. filter() 클로저를 사용하여 활성 상태(is_active: true) 및 20세 이상 조건 처리
    // 3. map() 클로저를 사용하여 username (String)만 추출 (소유권 문제 처리)
    // 4.  collect( 사용하여 Vec<String>으로 변환
    
    // let adult_names = user_list.iter().filter(|item| item.is_active == true && item.age >= 20).map(|item| item.username).collect();
    let adult_names = user_list.into_iter().filter(|item| item.is_active && item.age >= 20).map(|item| item.username).collect();

    adult_names
}

fn main() {
    let user_data = get_user_data();
    let active_adults = get_active_adult_names(user_data);

    // 예상되는 결과: ["alice", "david"]
    println!("활성 성인 사용자 목록: {:?}", active_adults);

    // (이후 user_data가 여전히 유효한지 확인하는 로직은 생략합니다.)
}
