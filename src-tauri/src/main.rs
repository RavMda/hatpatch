#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[derive(Clone, serde::Serialize)]
struct Output {
	message: String,
	message_type: Type
}

#[derive(Clone, serde::Serialize)]
enum Type {
	INFO,
	ERROR,
	SUCCESS
}

impl Output {
	pub fn send_info(app_handle: &tauri::AppHandle, msg: &str) {
		Output::send(app_handle, msg.to_string(), Type::INFO)
	}

	pub fn send_error(app_handle: &tauri::AppHandle, msg: &str) {
		Output::send(app_handle, msg.to_string(), Type::ERROR)
	}

	pub fn send_success(app_handle: &tauri::AppHandle, msg: &str) {
		Output::send(app_handle, msg.to_string(), Type::SUCCESS)
	}

	fn send(app_handle: &tauri::AppHandle, msg: String, msg_type: Type) {
		app_handle.emit_all("output", Output {
			message: msg,
			message_type: msg_type
		}).expect("something went wrong");
	}
}

use reqwest;
use tauri::Manager;
use std::io::Cursor;

async fn download_fabric(app_handle: &tauri::AppHandle) -> bool {
	Output::send_info(app_handle, "Downloading fabric...");

	let response_result = reqwest::get("https://hatcat.org/fabric-installer.jar").await;
	
	let response = match response_result {
		Ok(response) => response,
		Err(_) => {
			Output::send_error(app_handle, "Couldn't download fabric!");
			return false
		}
	};

	Output::send_info(app_handle, "Downloaded fabric!");

	let temp = std::env::temp_dir();

	let mut file = match std::fs::File::create(temp.join("fabric.jar")) {
		Ok(file) => file,
		Err(err) => {
			Output::send_error(app_handle, format!("Couldn't create fabric.jar file! {}", err).as_str());
			return false
		}
	};

	let bytes = match response.bytes().await {
		Ok(bytes) => bytes,
		Err(_) => {
			Output::send_error(app_handle, "Couldn't get bytes!");
			return false
		}
	};

	let mut content = Cursor::new(bytes);
	match std::io::copy(&mut content, &mut file) {
		Ok(_) => {},
		Err(_) => {
			Output::send_error(app_handle, "Couldn't copy fabric.jar!");
			return false
		}
	};

	true
}

async fn install_fabric(app_handle: &tauri::AppHandle, game_path: String) -> bool {
	let temp = std::env::temp_dir();

	let fabric_path = temp.join("fabric.jar");
	let fabric_path = match fabric_path.to_str() {
		Some(path) => path,
		None => {
			Output::send_error(app_handle, "Couldn't get fabric path!");
			return false
		}
	};

	let game_path = game_path.as_str();
	println!("{}", game_path);


	let args = ["-jar", fabric_path, "client", "-dir", game_path];

	Output::send_info(app_handle, "Installing fabric...");

	let cmd = std::process::Command::new("java").args(args).output();
	match cmd {
		Ok(out) => {
			let status = out.status;
			let stdout = String::from_utf8_lossy(&out.stdout);
			let stderr = String::from_utf8_lossy(&out.stderr);

			if status.success() {
				Output::send_success(app_handle, "Installed fabric!");
			} else {
				Output::send_error(app_handle, "Couldn't install fabric!");

				println!("stdout: {}", stdout);
				println!("stderr: {}", stderr);

				return false
			}
		},
		Err(err) => {
			let msg = format!("Couldn't install fabric! {}", err);
			Output::send_error(app_handle, &msg);
			return false
		}
	}

	true
}

#[tauri::command]
async fn patch_game(app_handle: tauri::AppHandle, game_path: String) {
	if !download_fabric(&app_handle).await {
		return
	};

	if !install_fabric(&app_handle, game_path).await {
		return
	};
}

fn main() {
  tauri::Builder::default()
	.invoke_handler(tauri::generate_handler![patch_game])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
