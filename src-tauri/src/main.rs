#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowEvent};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.set_focus().unwrap(); // Ensure the window gets focus

            Ok(())
        })
        .on_page_load(|window, _payload| {
            window.eval(r#"
                // Unlock FPS limit
                window.requestAnimationFrame = function(callback) { return window.setTimeout(callback, 0); };

                // Function to load a script and return a promise
                function loadScript(src) {
                    return new Promise((resolve, reject) => {
                        const script = document.createElement('script');
                        script.src = src;
                        script.onload = resolve;
                        script.onerror = () => {
                            console.error('Failed to load resource:', src);
                            reject(new Error('Failed to load script: ' + src));
                        };
                        document.head.appendChild(script);
                    });
                }

                // Function to load multiple resources and handle errors
                async function loadResources() {
                    const resources = [
                        'https://correct/path/to/otSDKStub.js',
                        'https://correct/path/to/gpt.js',
                        // Add other necessary script URLs here
                    ];

                    for (const resource of resources) {
                        try {
                            await loadScript(resource);
                            console.log('Successfully loaded:', resource);
                        } catch (error) {
                            console.error(error);
                        }
                    }
                }

                // Load resources
                loadResources();

                // Use MutationObserver to detect when the start button becomes available
                const observer = new MutationObserver((mutations) => {
                    const startButton = document.getElementById('start-button');
                    if (startButton) {
                        startButton.click();
                        console.log('Start button clicked');
                        observer.disconnect(); // Stop observing once the button is clicked
                    }
                });

                observer.observe(document, { childList: true, subtree: true });

                // Also try to click the button if it's already present
                document.addEventListener('DOMContentLoaded', (event) => {
                    const startButton = document.getElementById('start-button');
                    if (startButton) {
                        startButton.click();
                        console.log('Start button clicked');
                    }
                });
            "#).unwrap();
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
