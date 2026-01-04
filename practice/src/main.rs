/*
🟢 1단계: map (변환하기)

문제: 숫자가 담긴 벡터에서 각 숫자를 제곱한 새로운 벡터를 만드세요.

    데이터: let nums = vec![1, 2, 3, 4, 5];

    목표: [1, 4, 9, 16, 25] 만들기

    힌트: map(|x| ...)을 사용한 뒤 마지막에 .collect::<Vec<_>>()를 붙여야 벡터로 다시 변환됩니다.

🟡 2단계: filter (걸러내기)

문제: 문자열 벡터에서 길이가 3 이상인 단어만 골라내세요.

    데이터: let words = vec!["apple", "at", "banana", "it", "dog"];

    목표: ["apple", "banana", "dog"]만 남기기

    힌트: filter(|w| w.len() >= 3)

🔴 3단계: fold (하나로 합치기)

문제: 1부터 10까지 숫자가 담긴 벡터의 모든 합을 구하세요. (기존 sum() 메서드 대신 fold를 써보세요)

    데이터: let nums = (1..=10).collect::<Vec<_>>();

    목표: 55 계산하기

    힌트: fold(0, |acc, x| acc + x) (0은 초깃값, acc는 누적값입니다)
*/

fn main() {

    // 3단계
    let nums = (1..=10).collect::<Vec<i32>>();

    let nums_iter1 = nums.into_iter().fold(2, |acc, x| acc + x);
    
    println!("{:?}", nums_iter1);

    /*
       1단계 : map
    */
    let nums = vec![1, 2, 3, 4, 5];

    let nums_iter2 = nums.into_iter();

    // let result: Vec<i32> = nums_iter.map(|num| num * num).collect();
    let result = nums_iter2.map(|x| x * x).collect::<Vec<i32>>();
    // let result = nums_iter.map(|x| x * x).collect::<Vec<_>>();
    println!("{:?}", result);
    /*
       2단계 : filter
    */
    let words = vec!["apple", "at", "banana", "it", "dog"];

    let words_iter = words.into_iter();

    let result2 = words_iter.filter(|x| x.len() >= 3).collect::<Vec<_>>();
    for i in result2 {
        println!("i : {}", i);
    }


}
