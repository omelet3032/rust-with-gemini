// --------------------
// 1. 데이터 정의
// --------------------

// 재고 상태를 나타내는 열거형
#[derive(Debug)]
enum StockStatus {
    InStock(u32),        // 재고 있음 (수량)
    OutOfStock,          // 재고 없음
    Discontinued,        // 단종 상품
}

// 상품 정보를 담는 구조체
#[derive(Debug)]
struct Product {
    name: String,
    category: String,
    price: u32,
    status: StockStatus,
}

// 상품 데이터 생성
fn get_warehouse_inventory() -> Vec<Product> {
    vec![
        Product { name: String::from("Laptop Pro"), category: String::from("Electronics"), price: 1500, status: StockStatus::InStock(10) },
        Product { name: String::from("Coffee Mug"), category: String::from("Kitchen"), price: 15, status: StockStatus::InStock(150) },
        Product { name: String::from("Game Console"), category: String::from("Electronics"), price: 500, status: StockStatus::OutOfStock },
        Product { name: String::from("T-shirt Basic"), category: String::from("Apparel"), price: 30, status: StockStatus::InStock(50) },
        Product { name: String::from("Vintage Clock"), category: String::from("Decor"), price: 80, status: StockStatus::Discontinued },
        Product { name: String::from("Smartphone X"), category: String::from("Electronics"), price: 900, status: StockStatus::InStock(5) },
        Product { name: String::from("Leather Wallet"), category: String::from("Apparel"), price: 50, status: StockStatus::InStock(100) },
    ]
}

// --------------------
// 2. 함수형 요구사항 (클로저 트레이드 사용)
// --------------------

// 상품을 필터링하는 클로저가 구현해야 할 타입 별칭
// 즉, 이 클로저는 &Product 참조를 받아 bool을 반환해야 합니다.
// type ProductFilter = impl Fn(&Product) -> bool;
// type ProductFilter = Box<dyn Fn(&Product) -> bool>;

// --------------------
// 3. 핵심 함수 (작성 필요)
// --------------------

// 🚀 목표: 재고 목록을 받아, 주어진 필터 클로저에 따라 필터링한 후, 
//         필터링된 상품의 이름과 가격(Name: Price)을 담은 새로운 Vec<String>을 반환합니다.
// 📌 소유권 힌트: 입력 Vec<Product>의 소유권을 받아 처리하세요.
fn process_and_summarize_inventory<F>(inventory: Vec<Product>, filter_fn:F) -> Vec<String> 
where 
    F: Fn(&Product) -> bool,

{
    // 여기에 코드를 작성하세요 (이터레이터 체인을 사용하세요.)
    let filtered_invertory = inventory.into_iter().filter(filter_fn).map(|item| format!("{}: {}", item.name, item.price)).collect();
    
    filtered_invertory
}

// --------------------
// 4. 메인 로직 (클로저 정의 필요)
// --------------------

fn main() {
    let inventory = get_warehouse_inventory();

    // 📌 여기에 클로저 1을 정의하세요.
    // 💡 요구사항: "Electronics" 카테고리에 속하며, 가격이 1000 이상인 상품만 필터링하는 클로저를 만드세요.
    
    /* 
     "클로저를 정의하라는 문제인데 난 이터레이터를 만들고 있었다"
     문법과 논리를 모르고 무작정 자동 완성으로 해결하려다보니 이터레이터를 만들고 있었다.
    */
    // let filter_expensive_electronics = /* 여기에 클로저 정의 */ 
    // inventory.into_iter().filter(|item| item.category.contains("Electronics") && item.price >= 1000);

    // let filter_expensive_electronics = |item: &Product|{
    //     item.category.contains("Electronics") && item.price >= 1000
    // };


   /*  let filter_expensive_electronics = Box::new(|item: &Product|{
        item.category.contains("Electronics") && item.price >= 1000
    });
 */
    let filter_expensive_electronics = |item: &Product|{
        item.category.contains("Electronics") && item.price >= 1000
    };

    let result_a = process_and_summarize_inventory(inventory, filter_expensive_electronics);
    
    // 📌 여기에 클로저 2를 정의하세요.
    // 💡 요구사항: 재고가 50개 이상인 (StockStatus::InStock(수량)) 상품만 필터링하는 클로저를 만드세요.
    // 💡 힌트: 열거형 패턴 매칭을 활용해야 합니다.
    let filter_high_stock = /* 여기에 클로저 정의 */ 
        |item:&Product| {
            if let StockStatus::InStock(stock) = item.status {
                stock >= 50
            } else {
                false
            }
        }
        ;
    
    // process_and_summarize_inventory는 소유권을 소비하므로, inventory를 재생성해야 합니다.
    let inventory_b = get_warehouse_inventory();

    let result_b = process_and_summarize_inventory(inventory_b, filter_high_stock);

    println!("--- [결과 A: 비싼 전자제품] ---");
    println!("{:?}", result_a); 
    // 예상: ["Laptop Pro: $1500"]

    println!("\n--- [결과 B: 재고 50개 이상] ---");
    println!("{:?}", result_b);
    // 예상: ["Coffee Mug: $15", "Leather Wallet: $50"]
}

