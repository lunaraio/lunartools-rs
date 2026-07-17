# lunartools-rs

Rust SDK for the LunarTools remote APIs: captcha solving, inbox OTP retrieval, and Discord webhook forwarding.

Requests are routed to a specific user's running LunarTools toolbox. The toolbox must be open and connected.

```toml
[dependencies]
lunartools-rs = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The crate is `lunartools-rs`; the library is imported as `lunartools`.

## Quick start

Initialize once with your Client ID (Settings page in the toolbox), then pass the relevant API key per call.

```rust
use lunartools::{LunarTools, OtpOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LunarTools::new("lt_c0467cf9074b81e3a916178e597c9d0d")?;

    let otp = client
        .otp("lt_ik_...", &OtpOptions::new("softies_archaic_8x@icloud.com").site("nike"))
        .await?;

    println!("{} from {:?}", otp.otp_code, otp.imap_email);
    Ok(())
}
```

## Inbox API

`otp()` waits for a one-time code to arrive and returns it. Only unread mail is searched, newest first, and the matched message is marked read.

```rust
let options = OtpOptions::new("alias@icloud.com") // required - indexes the inbox
    .imap_email("mailbox@icloud.com")             // optional - omit to search every connected account
    .site("nike")                                 // optional - use a built-in parser
    .timeout_ms(60_000);                          // optional - default 60s, max 120s

let otp = client.otp(api_key, &options).await?;
```

Omit `imap_email` and the toolbox searches every connected account, then reports which one served the code in `otp.imap_email`.

With no `site` and no `regex`, the toolbox auto-detects the code (4, 6, or 8 characters), falling back to AI if the heuristic finds nothing.

Built-in sites: `bestbuy`, `crunchyroll`, `disney`, `eql`, `funko`, `goat`, `nike`, `privacy`, `samsclub`, `target`, `topps`, `walmart`, `zumiez`.

### Counting by subject

Use `count()` to count matching mail instead of extracting a code. Useful for win trackers. Returns immediately and marks nothing read.

```rust
use lunartools::CountOptions;

let result = client
    .count(api_key, &CountOptions::new("alias@icloud.com", "WINNER"))
    .await?;

println!("{}", result.count);
```

## Solving API

```rust
use lunartools::SolveOptions;

let result = client
    .solve(
        api_key,
        &SolveOptions::new(
            "hcaptcha",
            "https://example.com/checkout",
            "e94865c2-4231-4c25-9c6e-2b797b2b56cf",
        ),
    )
    .await?;

println!("{} in {}ms", result.token, result.solve_ms);
```

Add `.proxy_url("http://user:pass@host:port")` to override the harvester's own proxy.

## Webhooks

Posts a Discord-shaped payload to your forwarder token, which fans out to every Discord URL configured for it. Embeds without a `timestamp` get one automatically.

```rust
use lunartools::{Embed, WebhookPayload};

let payload = WebhookPayload::new()
    .username("LunarTools")
    .embed(
        Embed::new()
            .title("Checkout Success")
            .color(0x5865F2)
            .field("Site", "Nike", true),
    );

let result = client.webhook("your-webhook-token", &payload).await?;
println!("{} of {} delivered", result.delivered, result.count);
```

The webhook token comes from the toolbox and is not the same as an API key.

## Errors

Every failure is a `LunarToolsError` carrying the API's code.

```rust
match client.otp(api_key, &options).await {
    Ok(otp) => println!("{}", otp.otp_code),
    Err(error) => {
        if error.retryable() {
            // client_offline, client_disconnected, auth_unavailable, too_many_inflight
        }
        eprintln!("{} ({})", error.code(), error.status());
    }
}
```

| Code | Meaning |
|---|---|
| `invalid_key` | Unknown API key for this client |
| `key_disabled` | The key exists but is disabled |
| `client_offline` | The toolbox is not connected |
| `imap_not_connected` | No matching IMAP account is connected |
| `unsupported_site` | `site` is not a built-in parser |
| `invalid_regex` | `regex` failed to compile |
| `otp_timeout` | No code arrived before the deadline |
| `no_harvester` | No accepting harvester of that captcha type is open |
| `solve_timeout` | The solve did not finish before the deadline |
| `too_many_inflight` | Too many concurrent requests for this client |

## Options

```rust
use std::time::Duration;

let client = LunarTools::builder("lt_...")
    .base_url("https://remote.lunaraio.com")
    .timeout(Duration::from_secs(150))
    .build()?;
```

Requests are long-polled and can take up to 120s, so keep the client timeout above your per-request `timeout_ms`.
