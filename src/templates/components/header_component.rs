use horrorshow::prelude::*;
pub fn header(title: String) -> Box<dyn RenderBox> {
    box_html! {
        head {
               title: format!("Website Title TODO {}", title);
                meta(charset="UTF-8");
                meta(name="description", content="TODO");
                meta(name="keywords", content="TODO");
                meta(name="author", content="TODO");
                :Raw("<!-- Latest compiled and minified CSS -->
                 <link rel=\"stylesheet\" href=\"https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css\" integrity=\"sha384-BVYiiSIFeK1dGmJRAkycuHAHRg32OmUcww7on3RYdg4Va+PmSTsz/K68vbdEjh4u\" crossorigin=\"anonymous\">
                 <link rel=\"stylesheet\" href=\"https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap-theme.min.css\" integrity=\"sha384-rHyoN1iRsVXV4nD0JutlnGaslCJuC7uwjduW9SVrLvRYooPp2bWYgmgJQIXwl/Sp\" crossorigin=\"anonymous\">
                 <script src=\"https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/js/bootstrap.min.js\" integrity=\"sha384-Tc5IQib027qvyjSMfHjOMaLkfuWVxZxUPnCJA7l2mCWNIpG9mGCD8wGNIcPD7Txa\" crossorigin=\"anonymous\"></script>\
                 <link rel=\"stylesheet\" href=\"/files/css/style.css\">
                 <link href=\"https://fonts.googleapis.com/css?family=ZCOOL+KuaiLe&display=swap\" rel=\"stylesheet\">")
           }
    }
}
