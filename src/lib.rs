use std::net::TcpListener;

/// 获取可用端口
///
/// 默认为 9211
pub fn get_unused_port(port: u16) -> u16 {
  let port_string = port.to_string();
  let localhost = String::from("127.0.0.1:");
  match TcpListener::bind((localhost + port_string.as_str()).as_str()) {
      Ok(listener) => port,
      Err(err) => get_unused_port(port + 1),
  }
}

/// 获取本地服务
pub fn get_localhost(port: u16) -> String {
  let port_string = port.to_string();
  let localhost = String::from("127.0.0.1:");
  localhost + port_string.as_str()
}