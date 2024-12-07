use std::collections::{HashMap, HashSet};

// 基本的なデータ型の例示
fn show_basic_types() {
    // 数値型
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = '字';

    // 複合型
    let tuple: (i32, f64, char) = (1, 2.0, '3');
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("基本型の例:");
    println!("整数: {}", integer);
    println!("浮動小数点: {}", float);
    println!("真偽値: {}", boolean);
    println!("文字: {}", character);
    println!("タプル: {:?}", tuple);
    println!("配列: {:?}", array);
}

// 構造体の定義
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// 今回は未使用のためdead_code警告を抑制
#[allow(dead_code)]
struct Color(i32, i32, i32);

#[allow(dead_code)]
struct Unit;

// 列挙型
#[allow(dead_code)]
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Pending(String),
    Error { code: i32, message: String },
}

// トレイト定義
trait Describable {
    fn describe(&self) -> String;
}

// トレイトの実装
impl Describable for Person {
    fn describe(&self) -> String {
        // 不正な文字を修正
        format!("{}さん（{}歳）", self.name, self.age)
    }
}

fn main() {
    // 1. 変数と可変性
    let mut _mutable = 1; // 未使用変数はアンダースコアで始める
    _mutable = 2;
    let _immutable = 3;

    // 2. データ構造の使用例
    // ベクター
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // 文字列
    let mut string = String::from("こんにちは");
    string.push_str("、世界！");

    // ハッシュマップ
    let mut scores = HashMap::new();
    scores.insert(String::from("青チーム"), 10);
    scores.insert(String::from("赤チーム"), 20);

    // ハッシュセット
    let mut unique_nums = HashSet::new();
    unique_nums.insert(1);
    unique_nums.insert(2);

    // 構造体
    let person = Person {
        name: String::from("田中"),
        age: 25,
    };

    // 3. 制御構造
    // if式
    let number = 7;
    if number < 5 {
        println!("5より小さい");
    } else if number > 5 {
        println!("5より大きい");
    } else {
        println!("5と等しい");
    }

    // match式
    let status = Status::Pending(String::from("処理中"));
    match status {
        Status::Active => println!("アクティブ"),
        Status::Inactive => println!("非アクティブ"),
        Status::Pending(msg) => println!("保留中: {}", msg),
        Status::Error { code, message } => println!("エラー{}: {}", code, message),
    }

    // loop
    let mut counter = 0;
    let _result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    // while
    let mut n = 0;
    while n < 5 {
        println!("n = {}", n);
        n += 1;
    }

    // for
    for i in 0..5 {
        println!("i = {}", i);
    }

    // 4. エラーハンドリング
    // Result型
    let number_result: Result<i32, _> = "42".parse();
    match number_result {
        Ok(num) => println!("パース成功: {}", num),
        Err(e) => println!("パース失敗: {}", e),
    }

    // Option型
    let maybe_value: Option<i32> = Some(42);
    if let Some(value) = maybe_value {
        println!("値が存在します: {}", value);
    }

    // 5. クロージャ
    let add_one = |x| x + 1;
    println!("クロージャ結果: {}", add_one(5));

    // 6. イテレータ
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .filter(|x| x > &5)
        .collect();
    println!("処理後の数字: {:?}", doubled);

    // 基本型の表示
    show_basic_types();

    // トレイトの使用
    println!("人物の説明: {}", person.describe());
}
