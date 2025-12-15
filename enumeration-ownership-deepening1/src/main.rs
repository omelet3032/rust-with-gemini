#[derive(Debug)]
enum FlightStatus {
    Scheduled,         // 예정됨
    Delayed(u32),      // 지연 (분 단위)
    Cancelled(String), // 취소됨 (취소 사유)
    OnTime,            // 정시
}

#[derive(Debug)]
struct FlightInfo {
    airline: String,
    flight_number: u32,
    departure_city: String,
    arrival_city: String,
}

#[derive(Debug)]
enum FlightData {
    StatusUpdate(FlightStatus), // 상태 업데이트 정보
    NewFlight(FlightInfo),      // 새로운 비행 정보 (소유권이 큽니다)
    Emergency,                  // 긴급 상황
}

// ------------------------------------------
// 🚀 핵심 함수 1: 부분 처리 (if let 집중)
// ------------------------------------------

fn handle_status_updates(data: &FlightData) {
    // 💡 요구사항 A: data가 StatusUpdate일 경우, 내부 FlightStatus를 처리하세요.
    // 💡 힌트: &FlightData를 받았으므로, 내부의 StatusUpdate도 참조(&FlightStatus)로 매칭됩니다. (if let 사용)
    println!("\n--- [A] StatusUpdate 처리 ---");
    // 여기에 코드 작성

    if let FlightData::StatusUpdate(status) = data {
        println!("상태 : {:?}", status);
        if let FlightStatus::Delayed(time) = status {
            println!("지연 시간 : {}", time);
        }
    }
    // 💡 요구사항 B: 만약 StatusUpdate 내의 FlightStatus가 지연(Delayed)일 경우,
    //               지연 시간을 출력하세요. (if let 사용)
    // 💡 힌트: A에서 추출한 값(참조)을 다시 if let으로 처리해야 합니다.
    // 여기에 코드 작성
}

// ------------------------------------------
// 🚀 핵심 함수 2: 소유권 이동 처리 (match 집중)
// ------------------------------------------

fn process_new_flight_data(data: FlightData) -> Option<String> {
    // 💡 요구사항 C: data가 NewFlight일 경우, 내부 FlightInfo 구조체 전체의 소유권을 이동받아
    //               출발 도시와 도착 도시를 연결한 문자열을 Option<String>으로 반환하세요.
    // 💡 요구사항 D: data가 StatusUpdate일 경우, "상태 업데이트 수신됨"을 출력하고 None을 반환하세요.
    // 💡 요구사항 E: Emergency일 경우, "즉시 경고 발령!"을 출력하고 None을 반환하세요.
    // 📌 힌트: data를 인수로 받았으므로, match에서 소유권이 이동됩니다.

    // 여기에 코드 작성
    match data {
        FlightData::NewFlight(flight_info) => {
            // 여기서 data를 한번 더 쓰면 move out이 발생하겠지?
            Some(format!("{}{}", flight_info.departure_city, flight_info.arrival_city))
        }
        FlightData::StatusUpdate(flight_status) => {
            println!("상태 업데이트 수신됨");
            None
        }
        FlightData::Emergency => {
            println!("즉시 경고 발령!");
            None
        }
    }
}

fn main() {
    let flight_a = FlightData::StatusUpdate(FlightStatus::Delayed(45));
    let flight_b = FlightData::NewFlight(FlightInfo {
        airline: String::from("GlobalAir"),
        flight_number: 777,
        departure_city: String::from("Seoul"),
        arrival_city: String::from("New York"),
    });
    let flight_c = FlightData::Emergency;

    // --- A, B 실행 ---
    handle_status_updates(&flight_a);
    handle_status_updates(&flight_c);

    // flight_a는 참조로 전달되었으므로 여전히 유효함
    handle_status_updates(&flight_a);

    // --- C, D, E 실행 (소유권 이동) ---
    println!("\n--- [C, D, E] NewFlight 처리 및 소유권 이동 ---");
    // flight_b의 소유권이 process_new_flight_data로 이동합니다.
    let route_info = process_new_flight_data(flight_b);
    println!("Route Info: {:?}", route_info); // Some("Seoul -> New York")

    let status_result = process_new_flight_data(flight_a);
    println!("Status Result: {:?}", status_result); // None (StatusUpdate 처리됨)

    let emergency_result = process_new_flight_data(flight_c);
    println!("Emergency Result: {:?}", emergency_result); // None (Emergency 처리됨)

    // 📌 이 시점에서 flight_b, flight_c, flight_a는 모두 소유권이 이동되었거나,
    //    이전에 다른 함수에서 사용되어 유효하지 않으므로 사용하면 컴파일 오류가 발생해야 합니다.
    //    (flight_a는 사실 StatusUpdate 내부 String이 없으므로 Copy가 일어나 오류는 안 나지만,
    //     flight_b는 확실히 오류가 발생해야 합니다.)
    // let test_fail = flight_b.flight_number; // <- 주석 해제 시 컴파일 오류가 나야 합니다.
}
