use std::fs::File;
use std::env;
use std::io::{self, Read};
use std::process;


use webui_rs::webui;
use webui_rs::webui::WebUIBrowser;
use webui_rs::webui::wait;

mod html;
use crate::html::microblog_html;


mod parse;
use crate::parse::*;

pub fn main() -> io::Result<()>{
	// Get command line arguments
	let args: Vec<String> = env::args().collect();

	// Check if the user provided a file path
	if args.len() < 2 {
		eprintln!("Usage: {} <file_path>", args[0]);
		process::exit(1);
	}

	// Get the file path from the command line arguments
	let file_path = &args[1];

	let mut file: Vec<u8> = Vec::new();  // Declare file as a Vec<u8>

	// Try to open the file
	match File::open(file_path) {
		Ok(mut f) => {
			// Read the file content into the Vec<u8>
			match f.read_to_end(&mut file) {
				Ok(_) => {
					println!("File read successfully.");
				}
				Err(e) => {
					eprintln!("Failed to read the file: {}", e);
					process::exit(1);
				}
			}
		}
		Err(e) => {
			eprintln!("Failed to open the file: {}", e);
			process::exit(1);
		}
	}

	let win = webui::Window::new();
	let mut base_html = microblog_html.to_string();
	let parsed_bytes = file.to_html();
	//let css_content = fs::read_to_string(css_file_path)?;
        println!("Generating HTML.");
	let mut right_menu = String::new();
	let size = format!("<p><h3>File:</h3> {}. ", file_path);
	let file_path_html = format!("<h3>File size:</h3> {} bytes.</p>", file.len());
	right_menu.push_str(&size);
	right_menu.push_str(&file_path_html);
	base_html = base_html.replace("{CONTENT}", &parsed_bytes);
	base_html = base_html.replace("{RIGHT_MENU}", &right_menu);
	//base_html = base_html.replace("{CUSTOM_CSS}", &css_content);
        println!("Showing window.");
	win.show_browser(&base_html, WebUIBrowser::AnyBrowser);
	wait();
	Ok(())
}
