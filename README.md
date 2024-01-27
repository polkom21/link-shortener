# link-shortener

A link shortener project is a web-based application or service that takes long URLs and converts them into shorter, more manageable links. The primary purpose of link shorteners is to create concise and easily shareable links, especially for situations where long URLs are impractical, such as in social media posts, email messages, or QR codes.

This application was built for learning and should not be used in a production environment.

## Libraries

- Rust
- hyper
- diesel

## Migrations

Migrations are pushed on start application.

## Environments

| Name         | Required | Default | Example                                    |
| ------------ | -------- | ------- | ------------------------------------------ |
| DATABASE_URL | Yes      | -       | postgres://pg-user:pg-pass@pg-host/db-name |

## Requirements

### libpq

```sh
sudo apt-get install -y libpq-dev
```
