use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    /* As we have a struct defined for the article we are using that to declare an article variable
    which will be used by the serde_json to write into the variable called json. */
    let article = Article {
        article: String::from("How to work with JSON in RUST"),
        author: String::from("Vivek Chaurasia"),
        paragraph: vec![
            Paragraph {
                name: String::from("I am building a new Rust CLI tool to read JSON data."),
            },
            Paragraph {
                name: String::from("I have made few CLI tools in Rust."),
            },
            Paragraph {
                name: String::from("This is my third project and I will make few more. Thank you."),
            },
        ],
    };

    /* serde_json::to_string - Serialize the given data structure as a String of JSON. */
    let json = serde_json::to_string_pretty(&article).unwrap();
    println!("The JSON is \n{}", json);
}
