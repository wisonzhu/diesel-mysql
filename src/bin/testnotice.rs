// 按以下要求生成代码
// json的数据格式如下:
//  notice_token: 通知token
//  notice_type: 通知类型，目前只支持text
//  notice_data: 通知内容
//  notice_at: 通知人列表，目前只支持手机号

// 根据上面需求，生成代码如下
use reqwest::Client;
use serde_json::{from_str, Value};
use std::collections::HashMap;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut map = HashMap::new();
    map.insert("notice_token", "token");
    map.insert("notice_type", "text");
    map.insert("notice_data", "data");
    map.insert("notice_at", "at");

    let res = client
        .post("http://dev.openops.com/api/v1/kube-notice/feishu/notice")
        .json(&map)
        .send()
        .await?;

    let text = res.text().await?;
    let v: Value = from_str(&text)?;
    println!("{:?}", v);

    Ok(())
}








