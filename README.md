# Rust JSON Writer CLI Tool

Welcome to the Rust JSON Writer CLI tool! This project is a simple command-line tool built in Rust that demonstrates how to write JSON data. It uses the serde and serde_json crates for serialization and deserialization.

## How to Use

To use the tool, clone the repository and run the following command:

```bash
cargo run
```

## Code Overview

The project defines two structs: `Paragraph` and `Article`. The `Paragraph` struct represents a single paragraph with a name field, while the `Article` struct is a collection of paragraphs along with an article name and author.

In the `main` function, an instance of the `Article` struct is created with sample data. This struct is then serialized into a JSON string using `serde_json::to_string_pretty`.

## Dependencies

The project relies on the serde and serde_json crates for JSON serialization. These dependencies are essential for handling JSON data in Rust.

## Example

The generated JSON output will look like this:

```json
{
    "article": "How to work with JSON in RUST",
    "author": "Vivek Chaurasia",
    "paragraph": [
        {
            "name": "I am building a new Rust CLI tool to read JSON data."
        },
        {
            "name": "I have made few CLI tools in Rust."
        },
        {
            "name": "This is my third project and I will make few more. Thank you."
        }
    ]
}
```

## Observations

This tool showcases the serialization of Rust data structures into JSON format. Feel free to explore and modify the code to suit your needs. Happy JSON writing!
