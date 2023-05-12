<samp>

# sarufi-rs  🦀

A rust package to help you interact with the Sarufi platform inspired by [Python Sarufi SDK](https://github.com/Neurotech-HQ/sarufi-python-sdk)

## Installation

```bash
cargo add sarufi
```

## Authentication
```rust
>>> use sarufi::Sarufi
>>> let api_key = std::env::var("YOUR_SARUFI_API_KEY")
>>> let api = Sarufi(api_key).unwrap()
```

## Creating a bot
```rust
>>>   let name = "My Rusty Chatbot";
>>>   let description = Some("A rusty chatbot created using Sarufi API");
>>>   let industry = Some("Technology");

>>>   let bot = api.create_bot(
            name,
            description,
            industry,
        ).await?

>>>     bot.name
>>>     bot.id
```
Check out the file test.rs for more examples

</samp>