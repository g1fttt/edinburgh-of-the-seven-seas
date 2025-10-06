use super::thread_pool::Pool;

use http::header::{CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE};
use http::{HeaderValue, StatusCode, request, response};

use format_bytes::write_bytes;
use thiserror::Error;

use std::fs;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::path::Path;

#[derive(Debug)]
pub struct Server {
  thread_pool: Pool,
  listener: TcpListener,
}

impl Server {
  pub fn bind(addr: impl ToSocketAddrs) -> io::Result<Self> {
    let thread_pool = Pool::with_capacity(4);
    let listener = TcpListener::bind(addr)?;

    Ok(Self { thread_pool, listener })
  }

  pub fn handle_conns(&mut self) -> io::Result<()> {
    for stream in self.listener.incoming() {
      let stream = stream?;

      self.thread_pool.execute(|| {
        if let Err(err) = Self::handle_conn(stream)
          && err.kind() != ErrorKind::ConnectionReset
        {
          eprintln!("Failed to handle connection: {err}");
        }
      });
    }
    Ok(())
  }

  fn handle_conn(stream: TcpStream) -> io::Result<()> {
    let (mut reader, mut writer) =
      (BufReader::new(&stream), BufWriter::new(&stream));

    let mut buf = [0; 8192];

    let n = reader.read(&mut buf)?;

    if n == 0 {
      return Ok(());
    }

    let string = str::from_utf8(&buf[..n]).unwrap();
    let request = match parse_request(string) {
      Ok(req) => req,
      Err(err) => panic!("Failed to parse request: {err}"),
    };

    let response = create_response(request)?;
    let response_bytes = format_response_as_bytes(response)?;

    writer.write_all(&response_bytes)
  }
}

type HttpResponse = http::Response<Vec<u8>>;

fn parse_request(string: &str) -> Result<http::Request<()>, ParseError> {
  let mut lines = string.split("\r\n");

  let mut split_head = lines
    .next()
    .map(str::split_ascii_whitespace)
    .ok_or(ParseError::EmptyInput)?;

  let _get = split_head.next();
  let uri = split_head.next().ok_or(ParseError::MissingUri)?;

  let request = request::Builder::new().uri(uri).body(()).unwrap();

  Ok(request)
}

#[derive(Error, Debug)]
enum ParseError {
  #[error("Empty input provided")]
  EmptyInput,
  #[error("No uri were presented")]
  MissingUri,
}

fn create_response(request: http::Request<()>) -> io::Result<HttpResponse> {
  let content_path = match request.uri().path() {
    "/" => "index.html",
    p => &p[1..], // Trim the `/`
  };

  let extension = content_path.split_once('.').map(|(_, ext)| ext);
  let content_type = extension.and_then(|ext| {
    Some(match ext {
      "html" => ContentType::Html,
      "css" => ContentType::Css,
      "js" => ContentType::Js,
      "png" => ContentType::Png,
      "jpg" => ContentType::Jpg,
      "ico" => ContentType::Ico,
      _ => return None,
    })
  });

  let dist_path = Path::new("static");

  let mut content_path = dist_path.join(content_path);
  let mut status = StatusCode::OK;

  if !content_path.exists() {
    content_path = dist_path.join("404.html");
    status = StatusCode::NOT_FOUND;
  }

  let mut response = response::Builder::new().status(status);

  let content_bytes = fs::read(content_path)?;

  let headers = response.headers_mut().unwrap();
  headers.insert(CONTENT_LENGTH, content_bytes.len().into());

  // If the path was provided with unknown extension (or without extension at all),
  // then we're leaving the "guess" job to a web-browser.
  if let Some(ty) = content_type {
    let content_type = HeaderValue::from_static(ty.as_str());

    headers.insert(CONTENT_TYPE, content_type);

    if matches!(ty, ContentType::Png | ContentType::Jpg | ContentType::Ico) {
      headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=60"));
    }
  }

  Ok(response.body(content_bytes).unwrap())
}

fn format_response_as_bytes(response: HttpResponse) -> io::Result<Vec<u8>> {
  let status = response.status();

  let head = format!(
    "{:?} {} {}\r\n",
    response.version(),
    status.as_u16(),
    status.canonical_reason().unwrap(),
  );

  let mut buf = head.into_bytes();

  for (header_name, header_value) in response.headers() {
    write_bytes!(
      &mut buf,
      b"{}: {}\r\n",
      header_name.as_str().as_bytes(),
      header_value.as_bytes()
    )?;
  }

  write_bytes!(&mut buf, b"\r\n{}", response.body())?;

  Ok(buf)
}

#[derive(Clone, Copy)]
enum ContentType {
  Html,
  Css,
  Js,
  Png,
  Jpg,
  Ico,
}

impl ContentType {
  fn as_str(self) -> &'static str {
    match self {
      ContentType::Html => "text/html",
      ContentType::Css => "text/css",
      ContentType::Js => "application/javascript",
      ContentType::Png => "image/png",
      ContentType::Jpg => "image/jpeg",
      ContentType::Ico => "image/x-icon",
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn parse_request() {
    let req = super::parse_request("GET / HTTP/1.1").unwrap();

    assert_eq!(req.method().as_str(), "GET");
    assert_eq!(req.uri().path(), "/");
  }
}
