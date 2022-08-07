use webbrowser;

fn main() {
    
    // webbrowser::open("http://github.com");
    let a = if webbrowser::open("http://github.com").is_ok() {
        // println!("Hello, world!");
    };
    
}
