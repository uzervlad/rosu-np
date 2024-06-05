use std::{fs::File, io::Write, net::SocketAddr};

use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::mpsc::{self, UnboundedSender}};
use eyre::Result;

use crate::config::Config;

const HTML: &str = include_str!("auth/index.html");

const TWITCH_CLIENT_ID: &str = "ci2s72rvzqny52t3sn1fdxd4vaa8uc";

#[derive(Deserialize)]
struct TwitchResponse<T> {
  data: Vec<T>,
}

#[derive(Deserialize)]
struct TwitchUser {
  login: String,
}

#[derive(Serialize)]
struct ServerResponse {
  success: bool,
  message: String,
}

impl ServerResponse {
  fn ok(message: String) -> Self {
    Self {
      success: true,
      message
    }
  }

  fn not_ok(message: String) -> Self {
    Self {
      success: false,
      message
    }
  }
}

impl Into<Bytes> for ServerResponse {
  fn into(self) -> Bytes {
    serde_json::to_vec(&self).unwrap().into()
  }
}

async fn accept_token(req: Request<hyper::body::Incoming>, sender: UnboundedSender<Config>) -> Result<Response<Full<Bytes>>> {
  let token = match req.uri().query() {
    Some(token) => token,
    None => return Ok(Response::new(Full::new(ServerResponse::not_ok("No token provided".to_owned()).into()))),
  };

  let client = Client::new();

  let login = match client.get("https://api.twitch.tv/helix/users")
    .header("Authorization", format!("Bearer {}", token))
    .header("Client-Id", TWITCH_CLIENT_ID)
    .send().await
  {
    Ok(res) => match res.json::<TwitchResponse<TwitchUser>>().await {
      Ok(res) => {
        match res.data.first() {
          Some(user) => user.login.clone(),
          None => return Ok(Response::new(Full::new(ServerResponse::not_ok("Unable to retrieve user".to_owned()).into()))),
        }
      },
      Err(e) => return Ok(Response::new(Full::new(ServerResponse::not_ok(e.to_string()).into()))),
    },
    Err(e) => return Ok(Response::new(Full::new(ServerResponse::not_ok(e.to_string()).into()))),
  };

  let mut file = File::create("config.json").expect("Unable to create a config");
  let config = Config {
    username: login,
    token: token.to_owned(),
    timeout: 5,
  };
  file.write_all(serde_json::to_vec_pretty(&config).unwrap().as_slice()).expect("Unable to write a config");

  let _ = sender.send(config);

  Ok(Response::new(Full::new(ServerResponse::ok("".to_owned()).into())))
}

async fn serve(req: Request<hyper::body::Incoming>, sender: UnboundedSender<Config>) -> Result<Response<Full<Bytes>>> {
  match req.uri().path() {
    "/token" => accept_token(req, sender).await,
    _ => Ok(Response::new(Full::new(Bytes::from(HTML)))),
  }
}

pub async fn twitch_login() -> Result<Config> {
  let addr = SocketAddr::from(([127, 0, 0, 1], 9727));

  let listener = TcpListener::bind(addr).await?;

  let (send, mut recv) = mpsc::unbounded_channel::<Config>();

  println!("Open this link in your browser to authorize:");
  println!("https://id.twitch.tv/oauth2/authorize?response_type=token&client_id={}&redirect_uri=http://localhost:9727&scope=chat%3Aread+chat%3Aedit", TWITCH_CLIENT_ID);

  loop {
    let (stream, _) = tokio::select! {
      res = async { listener.accept().await } => res?,
      config = recv.recv() => return Ok(config.unwrap())
    };

    let io = TokioIo::new(stream);

    let service = {
      let send = send.clone();
      service_fn(move |req| serve(req, send.clone()))
    };

    tokio::task::spawn(async move {
      if let Err(e) = http1::Builder::new()
        .serve_connection(io, service)
        .await
      {
        eprintln!("Error serving connection: {:?}", e);
      }
    });
  }
}