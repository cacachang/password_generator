use inquire::Text;

fn main() {
    let length_ans = Text::new("請問您的密碼長度要設定多少？").prompt();

    let mut length = 8;

    match length_ans {
        Ok(length_ans) => match length_ans.parse::<i32>() {
            Ok(number) => {
                println!("設定的密碼長度為: {}", number);
                length = number;
            }
            Err(_) => {
                println!("您輸入的不是有效的整數");
            }
        },
        Err(_) => {
            println!("請輸入密碼長度");
        }
    }

    let mut charset = String::from("");
}
