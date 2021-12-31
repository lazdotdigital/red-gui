use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_view::{WVResult, WebView};

use crate::Red;

#[derive(Serialize, Deserialize)]
enum Event {
    Input,
    Button,
}

#[derive(Serialize, Deserialize)]
struct Invocation {
    event: Event,
    id: String,
    value: Option<String>,
}

impl Invocation {
    fn new(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
}

pub fn handler(wv: &mut WebView<impl Red>, arg: &str) -> WVResult<()> {
    let app = wv.user_data_mut();
    let invocation = Invocation::new(arg);
    match invocation.event {
        Event::Input => app.input(&invocation.id, &invocation.value.unwrap()),
        Event::Button => app.button(&invocation.id),
    }

    let map = app.values();
    let js = generate_render_js(app.values());
    wv.eval(&js)?;

    Ok(())
}

fn generate_render_js(map: HashMap<&'static str, String>) -> String {
    map.iter()
        .map(|(k, v)| {
            format!(
                r#"
(function() {{
  var els = document.getElementsByClassName("red-value {}");
  for (var i = 0; i < els.length; i++) {{
    els[i].innerHTML = "{}";
  }}
}})();
"#,
                k, v
            )
        })
        .collect::<Vec<String>>()
        .join("")
}
