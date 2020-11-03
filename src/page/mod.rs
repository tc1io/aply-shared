mod head;
mod footer;

use maud::*;

pub use self::head::head;
pub use self::footer::footer;

//use super::data;


//pub fn page(_u:Option<data::User>, title: &str, _current: &str, content: Markup) -> Markup {
pub fn page(title: &str, _current: &str, content: Markup) -> Markup {
  html! {
    (DOCTYPE)
    html {
      (head(title))
      body {
        div class="container is-fluid" {
          nav class="navbar" {
            div class="container" {
              div class="navbar-brand" {
                a class="navbar-item" href="/" {
                  img src = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEMAAABBCAYAAACKAhl4AAAAAXNSR0IArs4c6QAAAERlWElmTU0AKgAAAAgAAYdpAAQAAAABAAAAGgAAAAAAA6ABAAMAAAABAAEAAKACAAQAAAABAAAAQ6ADAAQAAAABAAAAQQAAAAAoqzCEAAAA9ElEQVR4Ae3SwQ2DMBQFQTv99xz4kSyNEOlgubAkhsPo7bXW977W3nvifly/nrvP//qcm/e9/KbfnjPn7NsZz55z887b2fn9XL43v/n8/I7/n3Nz/5yPdV9hOIKWgUYYYSBAtowwECBbRhgIkC0jDATIlhEGAmTLCAMBsmWEgQDZMsJAgGwZYSBAtowwECBbRhgIkC0jDATIlhEGAmTLCAMBsmWEgQDZMsJAgGwZYSBAtowwECBbRhgIkC0jDATIlhEGAmTLCAMBsmWEgQDZMsJAgGwZYSBAtowwECBbRhgIkC0jDATIlhEGAmTLCAMBsmWAcQH3KUCB+7kwEAAAAABJRU5ErkJggg==" alt="Ply";
                }
                a role="button" class="navbar-burger" data-target="navMenu" aria-label="menu" aria-expanded="false" {
                  span aria-hidden="true";
                  span aria-hidden="true";
                  span aria-hidden="true";
                }
              }
              div class="navbar-menu" id="navMenu" {
              div.navbar-start {
              //   (PreEscaped("<iframe src=\"/nav\"onload=\"this.before((this.contentDocument.body||this.contentDocument).children[0]);this.remove()\"></iframe>"))
                a class="navbar-item" href="#" { "Item 1" }
                a class="navbar-item" href="#" { "Item 2" }
              }
              div.navbar-end {
                (PreEscaped("<iframe src=\"/usernav\" onload=\"this.before((this.contentDocument.body||this.contentDocument).children[0]);this.remove()\"></iframe>"))
              }
            }
          }
        }
        (content)
        (footer())
        (PreEscaped(r#"
        <script>
                document.addEventListener('DOMContentLoaded', () => {

        // Get all "navbar-burger" elements
        const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);

        // Check if there are any navbar burgers
        if ($navbarBurgers.length > 0) {

        // Add a click event on each of them
        $navbarBurgers.forEach( el => {
        el.addEventListener('click', () => {

        // Get the target from the "data-target" attribute
        const target = el.dataset.target;
        const $target = document.getElementById(target);

        // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
        el.classList.toggle('is-active');
        $target.classList.toggle('is-active');

        });
        });
        }

        });
        </script>
    "#))


        }
      }
    }
  }
}

