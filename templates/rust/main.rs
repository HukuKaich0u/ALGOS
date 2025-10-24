// ========================================
// 1. null的なアプローチ（他言語風、Rustでは非推奨）
// ========================================
#[derive(Debug)]
struct UserBad {
    name: String,
    email: String,
    phone: String, // 電話番号がない人もいる → 空文字列で表現？
}

// ❌ 問題: 空文字列が「ない」を意味するのか「本当に空」なのか不明
fn print_user_bad(user: &UserBad) {
    println!("Name: {}", user.name);
    println!("Email: {}", user.email);

    // 電話番号があるかチェック（曖昧）
    if user.phone.is_empty() {
        println!("Phone: なし");
    } else {
        println!("Phone: {}", user.phone);
    }
}

// ========================================
// 2. Option<T>を使った安全なアプローチ
// ========================================
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    phone: Option<String>, // ✅ 明示的に「あるかないか」を表現
}

// ✅ コンパイラが強制的に None のケースを扱わせる
fn print_user(user: &User) {
    println!("Name: {}", user.name);
    println!("Email: {}", user.email);

    // match で安全に処理
    match &user.phone {
        Some(number) => println!("Phone: {}", number),
        None => println!("Phone: なし"),
    }
}

// Option<T> の便利なメソッド例
fn send_sms(user: &User, message: &str) {
    // if let で Some の場合だけ処理
    if let Some(number) = &user.phone {
        println!("📱 SMS送信: {} へ「{}」", number, message);
    } else {
        println!("⚠️  電話番号がないため SMS を送信できません");
    }
}

// ========================================
// 3. Option<T> の実用例: 検索機能
// ========================================
fn find_user_by_id(id: u32) -> Option<User> {
    // データベース検索を模擬
    if id == 1 {
        Some(User {
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
            phone: Some(String::from("090-1234-5678")),
        })
    } else if id == 2 {
        Some(User {
            name: String::from("Bob"),
            email: String::from("bob@example.com"),
            phone: None, // Bob は電話番号を登録していない
        })
    } else {
        None // ユーザーが見つからない
    }
}

// ========================================
// 4. Option<T> のチェーンメソッド（強力！）
// ========================================
fn get_area_code(user: &User) -> Option<String> {
    // 電話番号があれば市外局番を取得
    user.phone
        .as_ref() // &Option<String> → Option<&String>
        .map(|number| {
            // 電話番号の最初の3文字を取得
            number.chars().take(3).collect()
        })
}

// ========================================
// 5. null との比較デモ
// ========================================
fn demonstrate_null_problem() {
    println!("\n=== ❌ null的アプローチの問題点 ===");

    let user_bad = UserBad {
        name: String::from("Charlie"),
        email: String::from("charlie@example.com"),
        phone: String::from(""), // 空文字列 = ない？
    };

    print_user_bad(&user_bad);

    // 問題: これは意図的な空文字列？それとも未入力？
    let ambiguous = UserBad {
        name: String::from("Dave"),
        email: String::from("dave@example.com"),
        phone: String::from(""), // 判別不可能
    };
    println!("\n曖昧なケース:");
    print_user_bad(&ambiguous);
}

fn demonstrate_option_benefits() {
    println!("\n=== ✅ Option<T>の利点 ===\n");

    // 1. 値がある場合
    let alice = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        phone: Some(String::from("090-1234-5678")),
    };

    println!("--- ユーザー1 ---");
    print_user(&alice);
    send_sms(&alice, "こんにちは！");

    if let Some(area) = get_area_code(&alice) {
        println!("市外局番: {}", area);
    }

    // 2. 値がない場合
    let bob = User {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        phone: None, // 明示的に「ない」
    };

    println!("\n--- ユーザー2 ---");
    print_user(&bob);
    send_sms(&bob, "こんにちは！");

    // 3. 検索でユーザーが見つからない場合
    println!("\n--- ユーザー検索 ---");
    match find_user_by_id(1) {
        Some(user) => println!("✅ ユーザー発見: {}", user.name),
        None => println!("❌ ユーザーが見つかりません"),
    }

    match find_user_by_id(999) {
        Some(user) => println!("✅ ユーザー発見: {}", user.name),
        None => println!("❌ ユーザーが見つかりません"),
    }

    // 4. unwrap_or でデフォルト値
    println!("\n--- デフォルト値の使用 ---");
    let phone = bob.phone.unwrap_or(String::from("電話番号未登録"));
    println!("Bob の電話番号: {}", phone);

    // 5. and_then でチェーン処理
    println!("\n--- チェーン処理 ---");
    let result = find_user_by_id(1)
        .and_then(|user| user.phone)
        .map(|phone| format!("連絡先: {}", phone))
        .unwrap_or(String::from("連絡先なし"));
    println!("{}", result);
}

// ========================================
// メイン関数
// ========================================
fn main() {
    println!("╔═══════════════════════════════════════════╗");
    println!("║  Option<T> vs null の比較デモ            ║");
    println!("╚═══════════════════════════════════════════╝");

    demonstrate_null_problem();
    demonstrate_option_benefits();

    println!("\n\n=== 📚 Option<T>が優れている理由 ===");
    println!("1. ✅ 型システムで「値がないかも」を明示");
    println!("2. ✅ コンパイラが None のケースを強制的にチェック");
    println!("3. ✅ null pointer exception が起きない");
    println!("4. ✅ unwrap, map, and_then などの便利なメソッド");
    println!("5. ✅ 「ない」と「空」を明確に区別できる");
    println!("\n❌ null の問題:");
    println!("  - ランタイムエラー（NullPointerException）");
    println!("  - チェック忘れが起きやすい");
    println!("  - 「ない」と「空」の区別が曖昧");
}
