use inquire::{Confirm, Text};

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

    const PASSWORD_NUMBER: &str = "1234567890";

    let contain_number = Confirm::new("密碼是否包含數字").with_default(true).prompt();

    match contain_number {
        Ok(contain_number) => {
            if contain_number {
                println!("你的密碼將包含數字");
                charset.push_str(PASSWORD_NUMBER)
            } else {
                println!("你的密碼不包含數字");
            }
        }
        Err(_) => println!("請選擇是否包含數字"),
    }
}
