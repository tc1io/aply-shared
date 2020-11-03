use maud::*;

// Taken from https://bulma.io/documentation/overview/start/
pub fn head(page_title: &str) -> Markup {
    html! {

        head {
            title { (page_title) }
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta name="author" content="";
            meta name="description" content="";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/bulma/0.9.0/css/bulma.min.css";
        }
    }
}

//<script defer src="https://use.fontawesome.com/releases/v5.3.1/js/all.js"></script>



//script defer? src="https://use.fontawesome.com/releases/v5.1.0/js/all.js";
//link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.3.1/css/all.css" integrity="sha384-mzrmE5qonljUremFsqc01SB46JvROS7bZs3IO2EmfFsd15uHvIt+Y8vEf7N7fWAU" crossorigin="anonymous";
