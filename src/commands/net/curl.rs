//! curl command - HTTP requests

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct CurlCommand;

impl Command for CurlCommand {
    fn name(&self) -> &'static str {
        "curl"
    }

    fn description(&self) -> &'static str {
        "Make HTTP requests"
    }

    fn usage(&self) -> &'static str {
        "curl [-X method] [-H header] [-d data] <url>"
    }

    fn extended_help(&self) -> String {
        r#"curl - Make HTTP requests

USAGE:
  curl [OPTIONS] <url>

OPTIONS:
  -X, --request <method>   HTTP method (GET, POST, PUT, DELETE, etc.)
  -H, --header <header>    Add header (format: "Key: Value")
  -d, --data <data>        Send data in request body
  -i, --include            Include response headers in output
  -o, --output <file>      Write output to file

DESCRIPTION:
  Transfer data from or to a server using HTTP/HTTPS.
  Supports various HTTP methods and custom headers.

EXAMPLES:
  curl https://api.github.com           Simple GET request
  curl -i https://httpbin.org/get       Include response headers
  curl -X POST -d "name=test" URL       POST with form data
  curl -H "Authorization: Bearer TOKEN" URL   With auth header
  curl -X PUT -d '{"key":"value"}' URL  PUT with JSON data
  curl -o file.zip URL                  Download to file

JSON APIs:
  curl -H "Content-Type: application/json" \
       -d '{"name":"test"}' \
       https://api.example.com/users

COMMON STATUS CODES:
  200  OK              201  Created
  400  Bad Request     401  Unauthorized
  403  Forbidden       404  Not Found
  500  Server Error

RELATED COMMANDS:
  wget     Download files
  ping     Check host connectivity
"#
        .to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut method = "GET";
        let mut headers: Vec<(&str, &str)> = Vec::new();
        let mut data: Option<&str> = None;
        let mut url: Option<&str> = None;
        let mut show_headers = false;

        let mut iter = args.iter().peekable();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-X" | "--request" => {
                    if let Some(m) = iter.next() {
                        method = m.as_str();
                    }
                }
                "-H" | "--header" => {
                    if let Some(h) = iter.next() {
                        if let Some((key, value)) = h.split_once(':') {
                            headers.push((key.trim(), value.trim()));
                        }
                    }
                }
                "-d" | "--data" => {
                    if let Some(d) = iter.next() {
                        data = Some(d.as_str());
                    }
                }
                "-i" | "--include" => {
                    show_headers = true;
                }
                _ if !arg.starts_with('-') => {
                    url = Some(arg.as_str());
                }
                _ => {}
            }
        }

        let url = url.ok_or_else(|| anyhow::anyhow!("Usage: curl <url>"))?;

        // Build the request
        let client = reqwest::blocking::Client::new();

        let mut request = match method.to_uppercase().as_str() {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            "PATCH" => client.patch(url),
            "HEAD" => client.head(url),
            _ => return Err(anyhow::anyhow!("Unsupported method: {}", method)),
        };

        // Add headers
        for (key, value) in headers {
            request = request.header(key, value);
        }

        // Add body
        if let Some(body) = data {
            request = request.body(body.to_string());
        }

        // Execute request
        let response = request
            .send()
            .map_err(|e| anyhow::anyhow!("Request failed: {}", e))?;

        let mut output = String::new();

        if show_headers {
            output.push_str(&format!(
                "HTTP/1.1 {} {}\n",
                response.status().as_u16(),
                response.status().as_str()
            ));
            for (key, value) in response.headers() {
                output.push_str(&format!("{}: {}\n", key, value.to_str().unwrap_or("")));
            }
            output.push('\n');
        }

        let body = response
            .text()
            .map_err(|e| anyhow::anyhow!("Failed to read response: {}", e))?;

        output.push_str(&body);

        Ok(output)
    }
}
