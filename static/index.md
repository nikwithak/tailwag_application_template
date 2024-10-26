# Welcome to Tailwag

This is the standard template for a Tailwag application. To learn more, see the [Tailwag GitHub Repo](https://github.com/nikwithak/tailwag).

## Getting Started

- This template creates a Tailwag WebService with the following endpoints:

    - `/todo` - A CRUD endpoint for the Todo type, defined in `src/rest_endpoints/todo.rs`.
        - `GET /todo` - retrives all TODO items.
        - `GET /todo/{id}` - retrives only the TODO with the specified `id`.
        - `POST /todo` - creates a new TODO item.

    - `/static/{filename}` - An endpoint that will load any static files in the `static` directory. This file is [`static/index.md`](/static/index.md).
        - The service will automatically render Markdown to HTML for any `.md` files in this directory.
        - All static files are **public**. DO NOT put any sensitive files in the `static` folder.
