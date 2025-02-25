pub trait Parse {
fn to_html(&self) -> String;
}

impl Parse for Vec<u8> {
fn to_html(&self) -> String {
    let mut html = String::new();

    let index_padding = self.len().to_string().len();
    
    // Top bar offsets 
    let mut top_bar = "<span style=\"color: #9ad2d8; position:fixed; background-color:#2c2c2c;\">".to_string();
    top_bar.push_str(&"&nbsp;".repeat(index_padding));
    top_bar.push_str("&emsp;"); // Insert four spaces gap
    for i in 0..0x10 {
	top_bar.push_str(&format!("{:02x}", i));  // Format i to be 2 digits wide
	top_bar.push_str("&nbsp;");
    }
    for i in 0..16 {
        top_bar.push_str("&nbsp;");
    }
    top_bar.push_str("</span><br>");
    html.push_str(&top_bar);

    // Pad the first "0x0" similarly to the index format
    html.push_str(&format!("<span style=\"color: #9ad2d8;\">{:0width$}</span>", 0, width = index_padding)); // Pad "0x0" to match index padding
    html.push_str("&emsp;"); // Insert four spaces gap

    // Buffer to collect printable characters for each line
    let mut printable_buffer = String::new();

    // Iterate through the bytes in `self`
    for (i, &byte) in self.iter().enumerate() {
        // Convert byte to hexadecimal and append it
        let byte_str = format!("{:02x}", byte);
        html.push_str(&byte_str);

        // Collect printable characters for the end of the line
        if byte.is_ascii_alphanumeric() {
            printable_buffer.push(byte as char);
        } else {
            printable_buffer.push('.');
        }

        // Insert a space between bytes unless it's the end of the line
        if (i + 1) % 16 != 0 {
            html.push_str(" ");
        } else {
            // After every 16th byte, append the printable part to the end of the line
            html.push_str("  ");  // Two spaces before printable chars
            html.push_str(&printable_buffer);
            printable_buffer.clear();  // Clear the buffer for the next line
            html.push_str("<br>");

            // Add the index at the start of the next line
            let pos = i + 1;
            html.push_str(&format!("<span style=\"color: #9ad2d8;\">{:0width$X}</span>", pos, width = index_padding)); // Pad index dynamically and format as hex
            html.push_str("&emsp;"); // Insert four spaces gap
        }
    }

    // Handle remaining printable characters for the last line (if not exactly divisible by 16)
    if !printable_buffer.is_empty() {
        html.push_str("  ");
        html.push_str(&printable_buffer);
    }

    html.push_str("</p>");
    html
}
}
