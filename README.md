<samp>

# sarufi-rs  🦀

A rust package to help you interact with the Sarufi platform inspired by [Python Sarufi SDK](https://github.com/Neurotech-HQ/sarufi-python-sdk)

## Installation

```bash
cargo add sarufi
```

## Authentication
```rust
>>> use sarufi::{Sarufi, ApiError}
>>> let api_key = std::env::var("YOUR_SARUFI_API_KEY")
>>> let api = Sarufi(api_key).unwrap()
```

## Creating a bot
```rust

async fn test() -> Result<(),ApiError> {

    let name = "My Rusty Chatbot";
    let description = Some("A rusty chatbot created using Sarufi API");
    let industry = Some("Technology");
    let flow: Option<HashMap<String, Value>> = None;
    let intents: Option<HashMap<String, Vec<String>>> = None;
    let webhook_url = Some("https://example.com/webhook");
    let webhook_trigger_intents: Option<Vec<String>> = None;
    let visible_on_community = Some(true);  

    let bot = api.create_bot(
            name,
            description,
            industry,
            flow,
            intents,
            webhook_url,
            webhook_trigger_intents,
            visible_on_community,
    ).await?;

    println!("{:?}", bot);

    Ok(())
    
}
```
Check out the file test.rs for more examples

</samp>