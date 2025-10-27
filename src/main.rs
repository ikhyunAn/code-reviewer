use code_reviewer::config::Config;

fn main() {
    println!("=== Testing Config Loading ===\n");

    // Test 1: Load from file
    println!("Test 1: Loading config from file...");
    match Config::from_file("src/config/test_config.toml") {
        Ok(config) => {
            println!("✓ Config loaded successfully from file!");
            println!("Config: {:#?}\n", config);
        }
        Err(e) => {
            eprintln!("✗ Failed to load config from file: {}", e);
            eprintln!("Error details: {:?}\n", e);
        }
    }

    // Test 2: Load from environment variables
    // println!("Test 2: Loading config from environment...");
    // match Config::from_env() {
    //     Ok(config) => {
    //         println!("✓ Config loaded successfully from environment!");
    //         println!("Config: {:#?}\n", config);
    //     }
    //     Err(e) => {
    //         eprintln!("✗ Failed to load config from environment: {}", e);
    //         eprintln!("Error details: {:?}\n", e);
    //     }
    // }

    // Test 3: Load from config/config.toml if it exists
    // println!("Test 3: Loading config from config/config.toml...");
    // match Config::from_file("config/config.toml") {
    //     Ok(config) => {
    //         println!("✓ Config loaded successfully from config/config.toml!");
    //         println!("Config: {:#?}\n", config);
    //     }
    //     Err(e) => {
    //         eprintln!("✗ Failed to load config from config/config.toml: {}", e);
    //         eprintln!("(This is expected if the file doesn't exist)\n");
    //     }
    // }

    println!("=== Testing Complete ===");
}
