use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RequestBodyText<'a> {
    msgtype: &'a str,
    text: ContentText<'a>,
}

#[derive(Serialize)]
struct ContentText<'a> {
    title: &'a str,
    content: String,
}

#[derive(Serialize)]
struct RequestBodyMarkdown<'a> {
    msgtype: &'a str,
    markdown: ContentMarkdown<'a>,
}

#[derive(Serialize)]
struct ContentMarkdown<'a> {
    title: &'a str,
    text: String,
}

#[derive(Serialize)]
struct CustomResponse<T> {
    msg: String,
    data: Option<T>,
}

#[derive(Deserialize)]
pub struct DingResp {
    pub errcode: i32,
    pub errmsg: String,
}

pub async fn ding_text(infomation: String, title: String, ding_url: String) -> Result<DingResp> {
    if let Ok(c) = serde_json::to_string_pretty(&infomation) {
        return send_text(&ding_url, &title, c.as_str()).await;
    } else {
        return Err(anyhow!("serialize alert to json err: {:?}", infomation));
    };
}

async fn send_text(ding_url: &str, title: &str, c: &str) -> Result<DingResp> {
    let content = ContentText {
        title,
        content: c.to_string(),
    };
    let req_body = RequestBodyText {
        msgtype: "text",
        text: content,
    };

    let client = reqwest::Client::new();
    let res = client
        .post(ding_url)
        .json(&req_body)
        .send()
        .await?
        .json::<DingResp>()
        .await?;
    Ok(res)
}

pub async fn ding_markdown(
    mut infomation: String,
    title: &str,
    ding_url: &str,
) -> Result<DingResp> {
    infomation.push_str("\n");
    return send_markdown(ding_url, title, infomation.as_str()).await;
}

async fn send_markdown(ding_url: &str, title: &str, c: &str) -> Result<DingResp> {
    let content = ContentMarkdown {
        title,
        text: c.to_string(),
    };
    let req_body = RequestBodyMarkdown {
        msgtype: "markdown",
        markdown: content,
    };

    let client = reqwest::Client::new();
    let res = client
        .post(ding_url)
        .json(&req_body)
        .send()
        .await?
        .json::<DingResp>()
        .await?;
    Ok(res)
}
