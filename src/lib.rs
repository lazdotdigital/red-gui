mod invocation;

use std::collections::HashMap;
use web_view::{Content, WVResult};

pub trait Red {
    fn input(&mut self, _: &str, _: &str) {}

    fn button(&mut self, _: &str) {}

    fn values(&self) -> HashMap<&'static str, String> {
        HashMap::new()
    }
}
pub struct WindowConfig<'a> {
    pub title: &'a str,
    pub size: (i32, i32),
    pub resizable: bool,
    pub frameless: bool,
    pub debug: bool,
}

pub fn run(app: impl Red, config: WindowConfig, html: &'static str) -> WVResult<impl Red> {
    let wv = web_view::builder()
        .title(config.title)
        .content(Content::Html(html))
        .size(config.size.0, config.size.1)
        .resizable(config.resizable)
        .debug(config.debug)
        .frameless(config.frameless)
        .user_data(app)
        .invoke_handler(invocation::handler)
        .build()?;

    wv.run()
}
