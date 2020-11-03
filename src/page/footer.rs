use maud::*;

pub fn footer() -> Markup {
    html! {
        footer class="footer" {
          div class="content has-text-centered" {
            p { "FOOTER" }
          }

        }
    }
}


