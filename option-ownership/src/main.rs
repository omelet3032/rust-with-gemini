struct Product {
    id: u32,
    name: String,
    stock: Option<u32>, // 재고 수량. 값이 없을 수도 있음.
}

// 재고 목록을 생성하는 함수 (소유권을 반환)
fn get_inventory() -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: String::from("셔츠"),
            stock: Some(15),
        },
        Product {
            id: 2,
            name: String::from("바지"),
            stock: None,
        }, // 재고 데이터 없음
        Product {
            id: 3,
            name: String::from("모자"),
            stock: Some(0),
        }, // 재고 0
        Product {
            id: 4,
            name: String::from("양말"),
            stock: Some(40),
        },
    ]
}

// 함수는 Vec<Product>의 소유권을 받습니다.
fn get_low_stock_names(inventory: Vec<Product>) -> Vec<String> {
    let mut low_stock_names = Vec::new();

    // 여기에 코드를 작성하세요.
    // 1. inventory를 참조 형태로 순회 (소유권 유지)
    for product in &inventory {
        if let Some(item) = product.stock {
            if item < 10 {
                low_stock_names.push(product.name.clone());
            }
        }
    }
    // 2. if let 또는 match를 사용하여 product.stock의 값을 안전하게 추출
    // 3. 추출된 값이 10 미만인지 확인
    // 4. 조건을 만족하면 상품의 이름을 복사하여 (clone) low_stock_names에 추가

    low_stock_names
}

fn main() {
    let inventory = get_inventory();
    let low_stock_list = get_low_stock_names(inventory);

    // 예상되는 결과: ["모자"] (재고 0)
    println!("재고 부족 상품 목록: {:?}", low_stock_list);
}
