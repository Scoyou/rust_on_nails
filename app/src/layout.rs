use axum::response::Html;

pub fn layout(title: &str, content: & str) -> Html<String> {
  let layout = Layout {
    title,
    content,
  };

  Html(layout.to_string())
}

markup::define! {
  Layout<'a>(
    title: &'a str,
    content: &'a str,
  )
  {
    @markup::doctype()
    html[lang="en"] {
      head {
        meta [ charset="utf-8" ] {}
        meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
        meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
        title { {title} }

        script [ src = crate::statics::get_index_js(), type="text/javascript", async=""] {}

        link [ rel = "stylesheet", type="text/css" , href = crate::statics::get_index_css()] {}
      }

      body {
        main {
          {markup::raw(content)}
        }
      }
    }
  }
}