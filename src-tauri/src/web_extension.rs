use crate::{
    content_manager::add_url_from_extension,
    foc_error::{FocError, Result},
};
#[cfg(target_os = "windows")]
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::os::unix::net::{UnixListener, UnixStream};
use tauri::{AppHandle, Manager};

static ID: OnceCell<String> = OnceCell::new();

#[derive(Deserialize, Debug)]
pub struct Request<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub image: &'a str,
    pub command: &'a str,
}

#[derive(Debug)]
enum Command {
    SaveUrl,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    command: String,
    message: String,
}

#[derive(Serialize, Debug)]
struct NativeMessagingManifest {
    name: String,
    description: String,
    path: String,
    #[serde(rename = "type")]
    _type: String,
    allowed_origins: Vec<String>,
}

impl Command {
    pub fn from_string(command: &str) -> Self {
        match command {
            "saveUrl" => Command::SaveUrl,
            _ => panic!(
                "Unknown command recieved from focular web extension: {}",
                command
            ),
        }
    }
}

#[derive(Serialize)]
struct Response<'a> {
    payload: &'a str,
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn register(app_handle: AppHandle) -> Result<()> {
    use tauri::api::path::config_dir;

    let identifier = ID.get().expect("listen() called before prepare()").as_str();
    start_extension_listener(app_handle);

    let config_path = config_dir().unwrap();
    // TODO: hardcoded for chromium only, crashes on chrome possibly and other browsers
    let native_messaging_hosts_path = config_path.join("chromium/NativeMessagingHosts");
    // TODO: path and allowed origins are hardcoded
    // TODO: we can use tauri_utils::platform::current_exe()? to get current exe path and other kinds of info
    let manifest = NativeMessagingManifest {
        name: identifier.to_owned(),
        description: "Focular extension - save anything with ease!".to_owned(),
        path: "/home/caleidon/Desktop/focular/src-tauri/target/debug/focular".to_owned(),
        _type: "stdio".to_owned(),
        allowed_origins: vec!["chrome-extension://ddhnbmekgnoabhomcefddlcbfbfjaalf/".to_owned()],
    };

    let serialized = serde_json::to_string(&manifest)?;
    std::fs::write(
        native_messaging_hosts_path.join(format!("{}.json", identifier)),
        &serialized,
    )?;

    Ok(())
}

/* pub fn unregister(scheme: &str) -> Result<(), std::io::Error> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let base = Path::new("Software").join("Classes").join(scheme);

    hkcu.delete_subkey_all(base)?;

    Ok(())
} */

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn check_extension_requests(identifier: &str) -> Result<()> {
    use std::{fs::remove_file, io::ErrorKind};

    let socket_addr = format!("/tmp/{}.sock", identifier);
    match UnixStream::connect(&socket_addr) {
        Ok(connection) => {
            read_request(connection)?;
            send_response()?;
            std::process::exit(0);
        }
        Err(err) => {
            if err.kind() == ErrorKind::ConnectionRefused {
                let _ = remove_file(&socket_addr);
            }
        }
    }

    ID.set(identifier.to_string())
        .expect("check_extension_requests called more than once with different identifiers.");
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn start_extension_listener(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let addr = format!(
            "/tmp/{}.sock",
            ID.get()
                .expect("start_extension_listener called before check_extension_requests")
        );

        let listener = UnixListener::bind(addr).expect("Error while creating extension listener");

        for mut connection in listener.incoming().filter_map(|r| {
            r.map_err(|error| eprintln!("Incoming connection failed: {}", error))
                .ok()
        }) {
            let mut buffer = String::new();
            if let Err(err) = connection.read_to_string(&mut buffer) {
                println!("Error while reading extension message: {}", err);
                continue;
            }

            let request: Request = serde_json::from_str(&buffer).unwrap();
            handle_request(&request, &app_handle);
            buffer.pop();
        }
    });
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn read_request(mut connection: UnixStream) -> Result<()> {
    let mut input = std::io::stdin();
    let mut length = [0; 4];
    input.read_exact(&mut length)?;

    let length = to_u32(length);
    let mut buffer = vec![0; length as usize];
    input.read_exact(&mut buffer)?;

    connection.write_all(&buffer).unwrap();
    connection.write_all(b"\n").unwrap();

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn check_extension_requests(identifier: &str) -> Result<()> {
    if let Ok(connection) = LocalSocketStream::connect(identifier) {
        read_request(connection)?;
        send_response()?;
        std::process::exit(0);
    }

    ID.set(identifier.to_string())
        .expect("check_extension_requests called more than once with different identifiers.");
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn register(app_handle: AppHandle) -> Result<()> {
    use std::path::Path;

    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    let identifier = ID.get().expect("listen() called before prepare()").as_str();
    start_extension_listener(app_handle);

    //TODO: this is completely hardcoded path
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let base = Path::new("Software")
        .join("Google")
        .join("Chrome")
        .join("NativeMessagingHosts")
        .join(identifier);

    let (key, _) = hkcu.create_subkey(&base)?;
    // TODO: we also need to create the manifest dynamically on windows
    key.set_value(
        "",
        &"C:\\Users\\Caleidon\\Desktop\\Focular\\com.envoid.focular.json",
    )?;

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn start_extension_listener(app_handle: AppHandle) {
    use std::io::{BufRead, BufReader};

    std::thread::spawn(move || {
        let listener = LocalSocketListener::bind(
            ID.get()
                .expect("start_extension_listener called before check_extension_requests")
                .as_str(),
        )
        .expect("Error while creating extension listener");

        for connection in listener.incoming().filter_map(|r| {
            r.map_err(|error| eprintln!("Incoming connection failed: {}", error))
                .ok()
        }) {
            let mut connection = BufReader::new(connection);

            let mut buffer = String::new();
            if let Err(io_err) = connection.read_line(&mut buffer) {
                // TODO: might want to switch to some log crate in the future instead of using println
                println!("Error reading incoming connection: {}", io_err);
                continue;
            };

            let request: Request = serde_json::from_str(&buffer).unwrap();
            handle_request(&request, &app_handle);
            buffer.pop();
        }
    });
}

#[cfg(target_os = "windows")]
fn read_request(mut connection: LocalSocketStream) -> Result<()> {
    let mut input = std::io::stdin();
    let mut length = [0; 4];
    input.read_exact(&mut length)?;

    let length = to_u32(length);
    let mut buffer = vec![0; length as usize];
    input.read_exact(&mut buffer)?;

    connection.write_all(&buffer).unwrap();
    connection.write_all(b"\n").unwrap();

    Ok(())
}

fn send_response() -> Result<()> {
    let mut output = std::io::stdout();

    // TODO: we probably want to respond somehow if the request was nicely recieved and the app wasn't off
    let response = "Rust has recieved your message!";
    let response_json = serde_json::to_string(response).unwrap();
    let len = response_json.len();
    if len > 1024 * 1024 {
        return Err(FocError::Extension(
            "Message we tried to send to the extension is too long!".to_owned(),
        ));
    }

    let bytes = to_byte_array(len as u32);
    output.write_all(&bytes)?;
    output.write_all(response_json.as_bytes())?;
    output.flush()?;

    Ok(())
}

fn handle_request(request: &Request, app_handle: &AppHandle) {
    match Command::from_string(request.command) {
        Command::SaveUrl => {
            tauri::async_runtime::block_on(add_url_from_extension(request)).unwrap();
            app_handle
                .emit_to(
                    "main",
                    "web_extension_event",
                    Payload {
                        command: "saveUrl".to_owned(),
                        message: "".to_owned(),
                    },
                )
                .unwrap();
        }
    }
}

fn to_u32(arr: [u8; 4]) -> u32 {
    #[cfg(target_endian = "big")]
    {
        u32::from_be_bytes(arr)
    }
    #[cfg(target_endian = "little")]
    {
        u32::from_le_bytes(arr)
    }
}

fn to_byte_array(value: u32) -> [u8; 4] {
    #[cfg(target_endian = "big")]
    {
        value.to_be_bytes()
    }
    #[cfg(target_endian = "little")]
    {
        value.to_le_bytes()
    }
}
