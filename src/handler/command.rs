pub fn website() -> String {
    "https://axum.rs".to_string()
}

pub fn logo() -> String {
    "https://axum.rs/static/img/logo.png".to_string()
}

pub fn help(msg: Option<&str>) -> String {
    let header = match msg {
        Some(txt) => format!("你输入的 `{}` 有误", txt),
        None => "".to_string(),
    };
    let body = r#"
__使用帮助__
`/website`：访问官方网站
`/logo`：获取官方LOGO
`/help`：显示帮助信息
        "#;
    format!(r#"{}{}"#, header, body)
}
