use std::error::Error;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct RequestBody {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    pub role: String,
    pub content: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Reply {
    pub id: String,
    pub model: String,
    pub created: u32,
    pub object: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index: u32,
    pub logprobs: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

pub struct TranslateParam<'a> {
    pub source: &'a str,
    pub api_key: &'a str,
}

pub fn translate(param: TranslateParam) -> Result<String, Box<dyn Error>> {
    let TranslateParam { source, api_key } = param;
    let client = reqwest::blocking::Client::new();
    let request_body = RequestBody {
        model: "qwen-turbo".into(),
        messages: vec![
            Message {
                role: "system".into(),
                content: "你是一个资深的临床试验专家，请协助我将下面的中文翻译成英文，且不要回复答案以外的内容，比如我发送'男性'，你仅需回复Male即可，如果内容包含了类似'{\\uc0\\u12288 }'这种被花括号包裹起来的，里面是类似unicode字符声明的内容，则无需翻译这部分内容，直接保留在原文中即可".into(),
            },
            Message {
                role: "user".into(),
                content: source.into(),
            },
        ],
        temperature: 0.8,
    };
    let body = serde_json::to_vec(&request_body)?;
    let result = client
        .post("https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send()?;
    let bytes = result.bytes()?;
    let reply = serde_json::from_slice::<Reply>(&bytes)?;
    Ok(reply.choices[0].message.content.clone())
}
