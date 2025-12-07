use crate::settings::AlertConfig;
use reqwest::blocking::Client;
use serde_json::json;

pub fn send_alert(config: &AlertConfig, message: &str) {
    if !config.enabled {
        return;
    }

    let client = Client::new();

    // Telegram
    if let (Some(token), Some(chat_id)) = (&config.telegram_bot_token, &config.telegram_chat_id) {
        if !token.is_empty() && !chat_id.is_empty() {
            let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
            let params = [("chat_id", chat_id), ("text", &message.to_string())];
            match client.post(&url).form(&params).send() {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        println!("Failed to send Telegram alert: {:?}", resp.text());
                    }
                }
                Err(e) => println!("Error sending Telegram alert: {}", e),
            }
        }
    }

    // Webhook (Generic)
    if let Some(url) = &config.webhook_url {
        if !url.is_empty() {
            let payload = json!({
                "text": message,
                "alert": "Volumetrik Disk Usage Warning"
            });
            send_request(&client, url, &payload, "Generic Webhook");
        }
    }

    // Pushover
    if let (Some(user), Some(token)) = (&config.pushover_user_key, &config.pushover_api_token) {
        if !user.is_empty() && !token.is_empty() {
            let url = "https://api.pushover.net/1/messages.json";
            let params = [
                ("token", token.as_str()),
                ("user", user.as_str()),
                ("message", message),
            ];
            match client.post(url).form(&params).send() {
                Ok(resp) => if !resp.status().is_success() { println!("Failed to send Pushover alert: {:?}", resp.text()); },
                Err(e) => println!("Error sending Pushover alert: {}", e),
            }
        }
    }

    // Gotify
    if let (Some(url), Some(token)) = (&config.gotify_url, &config.gotify_token) {
        if !url.is_empty() && !token.is_empty() {
            let full_url = format!("{}/message?token={}", url.trim_end_matches('/'), token);
            let payload = json!({
                "message": message,
                "title": "Volumetrik Alert",
                "priority": 5
            });
            send_request(&client, &full_url, &payload, "Gotify");
        }
    }

    // Slack
    if let Some(url) = &config.slack_webhook_url {
        if !url.is_empty() {
            let payload = json!({ "text": message });
            send_request(&client, url, &payload, "Slack");
        }
    }

    // Discord
    if let Some(url) = &config.discord_webhook_url {
        if !url.is_empty() {
            let payload = json!({ "content": message });
            send_request(&client, url, &payload, "Discord");
        }
    }

    // Microsoft Teams
    if let Some(url) = &config.teams_webhook_url {
        if !url.is_empty() {
            let payload = json!({ "text": message });
            send_request(&client, url, &payload, "Microsoft Teams");
        }
    }

    // Ntfy
    if let Some(url) = &config.ntfy_url {
        if !url.is_empty() {
            let mut req = client.post(url).body(message.to_string());
            
            if let Some(token) = &config.ntfy_token {
                if !token.is_empty() {
                    req = req.header("Authorization", format!("Bearer {}", token));
                }
            }
            
            req = req.header("Title", "Volumetrik Alert");
            
            match req.send() {
                 Ok(resp) => if !resp.status().is_success() { println!("Failed to send Ntfy alert: {:?}", resp.text()); },
                 Err(e) => println!("Error sending Ntfy alert: {}", e),
            }
        }
    }
}

fn send_request(client: &Client, url: &str, payload: &serde_json::Value, service: &str) {
    match client.post(url).json(payload).send() {
        Ok(resp) => {
            if !resp.status().is_success() {
                println!("Failed to send {} alert: {:?}", service, resp.text());
            }
        }
        Err(e) => println!("Error sending {} alert: {}", service, e),
    }
}
