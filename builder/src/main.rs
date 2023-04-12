use imagesize::size;
use minijinja::context;
use minijinja::Environment;
use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize)]
struct Payload {
    images: Option<Vec<Image>>,
}

impl Payload {
    fn new() -> Payload {
        Payload {
            images: Some(vec![]),
        }
    }
}

#[derive(Debug, Serialize)]
struct Image {
    path: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

fn main() {
    // let mut env = Environment::new();
    // env.add_template("palettes", template.as_str()).unwrap();

    //
    let mut payload = Payload::new();
    let output_path = "../site/palettes-series-2/index.html";

    let mut file_names: Vec<String> = vec![];

    let video_pixel_art_alfa = fs::read_to_string("src/templates/palettes-series-2.html").unwrap();
    // let video_pixel_art_alfa = fs::read_to_string("src/templates/test.html").unwrap();

    let paths =
        fs::read_dir("/Users/alan/workshop/tmp.alanwsmith.com/site/palettes-series-2/images")
            .unwrap();

    for path in paths {
        let base_path = path.unwrap().path();
        let extension = base_path.extension();
        match extension {
            Some(x) => {
                if x == "png" {
                    let mut i = Image {
                        path: None,
                        width: None,
                        height: None,
                    };
                    file_names.push(base_path.file_name().unwrap().to_str().unwrap().to_string());
                    i.path = Some(base_path.file_name().unwrap().to_str().unwrap().to_string());
                    let img = size(base_path).unwrap();
                    i.width = Some(img.width.try_into().unwrap());
                    i.height = Some(img.height.try_into().unwrap());
                    payload.images.as_mut().unwrap().push(i);
                }
                ()
            }
            None => (),
        }
    }
    let output = render_template(video_pixel_art_alfa, payload);
    dbg!(&output);
    fs::write(output_path, output).unwrap();
}

fn render_template(template: String, payload: Payload) -> String {
    let mut env = Environment::new();
    env.add_template("template", template.as_str()).unwrap();
    let tmpl = env.get_template("template").unwrap();
    tmpl.render(context!(payload => &payload))
        .unwrap()
        .to_string()
}
