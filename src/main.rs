use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

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
        format!("{}さん（{}歳）", self.name, self.age)
    }
}

//---------------- 追加要素ここから -----------------

// ライフタイムと参照の例
// 2つの文字列スライスを受け取り、長い方の参照を返す関数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// ジェネリック関数とトレイト境界
// Debugトレイトを実装している任意の型に対して要素を表示する
fn print_elements<T: std::fmt::Debug>(slice: &[T]) {
    for item in slice {
        println!("{:?}", item);
    }
}

// Option型とResult型におけるさらに詳細なパターンマッチング例
fn option_and_result_patterns() {
    let maybe_num: Option<i32> = Some(100);
    match maybe_num {
        Some(n) if n > 50 => println!("50より大きい値: {}", n),
        Some(n) => println!("50以下の値: {}", n),
        None => println!("値はありません"),
    }

    let parse_result: Result<i32, _> = "not_number".parse();
    match parse_result {
        Ok(n) => println!("パース成功: {}", n),
        Err(e) => println!("パース失敗: {}", e),
    }
}

// `?`演算子によるエラーハンドリング
// ファイルを読み込んでその内容を文字列で返す関数
fn read_file_content(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?; // ここで?を使ってエラーを即Return
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// スマートポインタ(Box)の例
fn box_example() {
    let x = Box::new(10);
    println!("Boxに格納された値: {}", x);
}

// Rc(参照カウント型スマートポインタ)の例
fn rc_example() {
    let rc_value = Rc::new(String::from("共有文字列"));
    let rc_clone1 = Rc::clone(&rc_value);
    let rc_clone2 = Rc::clone(&rc_value);
    println!("Rcの参照カウント: {}", Rc::strong_count(&rc_value));
    println!("オリジナル: {}, クローン1: {}, クローン2: {}",
        rc_value, rc_clone1, rc_clone2);
}

// impl Traitを利用した返り値
// イテレータを返す関数の例
fn create_range_iter() -> impl Iterator<Item = i32> {
    0..5 // 0から4までのイテレータ
}

// 並行処理の例（スレッド生成）
fn thread_example() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("別スレッド: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=5 {
        println!("メインスレッド: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    // スレッド終了待ち
    handle.join().unwrap();
}

// スライスを使った関数
fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

//---------------- 追加要素ここまで -----------------

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

    // ---------------- ここから追加要素呼び出し -----------------

    // ライフタイムの例
    let str1 = "Hello";
    let str2 = "こんにちは";
    let longer = longest(str1, str2);
    println!("より長い文字列: {}", longer);

    // スライスの例
    let slice_result = sum_slice(&numbers);
    println!("スライスの合計: {}", slice_result);

    // ジェネリック関数とトレイト境界
    print_elements(&numbers);

    // OptionとResultの追加パターンマッチ例
    option_and_result_patterns();

    // ?演算子によるファイル読み込み
    // ここでは存在しないファイル名を渡してエラー例示
    match read_file_content("not_exist.txt") {
        Ok(content) => println!("ファイル内容: {}", content),
        Err(e) => println!("ファイル読み込みエラー: {}", e),
    }

    // Boxの例
    box_example();

    // Rcの例
    rc_example();

    // impl Traitを使ったイテレータ返却例
    let range_iter = create_range_iter();
    for val in range_iter {
        println!("range_iterの値: {}", val);
    }

    // 並行処理の例
    thread_example();

    println!("メイン関数終了");
}
