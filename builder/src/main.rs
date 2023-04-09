use minijinja::context;
use minijinja::Environment;
use std::fs;

fn main() {
    let output_path = "../site/palettes-series-1/index.html";

    let mut file_names: Vec<String> = vec![];

    let video_pixel_art_alfa = fs::read_to_string("src/templates/palettes-series-1.html").unwrap();

    let paths =
        fs::read_dir("/Users/alan/workshop/tmp.alanwsmith.com/site/palettes-series-1/images")
            .unwrap();

    for path in paths {
        let base_path = path.unwrap().path();
        let extension = base_path.extension();
        match extension {
            Some(x) => {
                if x == "png" {
                    file_names.push(base_path.file_name().unwrap().to_str().unwrap().to_string());
                }
                ()
            }
            None => (),
        }
    }
    let output = render_template(video_pixel_art_alfa, file_names);
    fs::write(output_path, output).unwrap();
}

fn render_template(template: String, paths: Vec<String>) -> String {
    let mut env = Environment::new();
    env.add_template("template", template.as_str()).unwrap();
    let tmpl = env.get_template("template").unwrap();
    tmpl.render(context!(paths => paths)).unwrap().to_string()
}
