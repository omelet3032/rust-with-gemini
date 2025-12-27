use std::time::Instant;

// 1. 소유권 방식 (느림: 메모리 할당과 복사가 일어남)
struct SlowLog {
    message: String,
}

// 2. 라이프타임 참조 방식 (빠름: 주소만 복사함)
struct FastLog<'a> {
    message: &'a str,
}

fn main() {
    // 10만 개의 가짜 로그 데이터 생성
    let raw_data = "ERROR: 시스템에 심각한 오류가 발생했습니다.".repeat(100_000);
    let lines: Vec<&str> = raw_data.lines().collect(); // 실제로는 한 줄이지만 예시를 위해

    // --- 테스트 1: SlowLog (소유권) ---
    let start = Instant::now();
    let mut slow_pool = Vec::new();
    for line in &lines {
        // String으로 변환하면서 새로운 메모리 할당 + 데이터 복사 발생
        slow_pool.push(SlowLog { message: line.to_string() });
    }
    let duration_slow = start.elapsed();
    println!("🐢 SlowLog (String 복사) 소요 시간: {:?}", duration_slow);

    // --- 테스트 2: FastLog (라이프타임 참조) ---
    let start = Instant::now();
    let mut fast_pool = Vec::new();
    for line in &lines {
        // 주소값만 복사함 (할당 X, 복사 X)
        fast_pool.push(FastLog { message: line });
    }
    let duration_fast = start.elapsed();
    println!("🚀 FastLog (라이프타임 참조) 소요 시간: {:?}", duration_fast);

    // 두 방식의 성능 차이 계산
    let ratio = duration_slow.as_nanos() as f64 / duration_fast.as_nanos() as f64;
    println!("\n📊 성능 차이: 참조 방식이 약 {:.2}배 더 빠릅니다!", ratio);
}