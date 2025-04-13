#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{App, Builder};

#[tauri::command]
fn parse_steam(discount_filter: u32, max_pages: u32) -> Result<Vec<String>, String> {
    use reqwest::blocking::Client;
    use scraper::{Html, Selector};
    use std::fs::File;
    use std::io::Write;
    use std::{thread, time};

    let client = Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| e.to_string())?;

    let row_selector = Selector::parse("a.search_result_row").unwrap();
    let title_selector = Selector::parse("span.title").unwrap();
    let discount_selector = Selector::parse("div.discount_pct").unwrap();
    let price_old_selector = Selector::parse("div.discount_original_price").unwrap();
    let price_new_selector = Selector::parse("div.discount_final_price").unwrap();

    let mut all_games = Vec::new();
    let output_file = format!("{}_steam.txt", discount_filter);

    for page in 1..=max_pages {
        let url = format!("https://store.steampowered.com/search/?specials=1&page={}", page);

        let body = client.get(&url)
            .send()
            .and_then(|res| res.text())
            .map_err(|e| format!("Ошибка запроса: {}", e))?;

        let document = Html::parse_document(&body);
        let rows = document.select(&row_selector).collect::<Vec<_>>();

        if rows.is_empty() {
            break;
        }

        for row in rows {
            let title = row.select(&title_selector)
                .next().map(|t| t.text().collect::<String>())
                .unwrap_or_else(|| "Без названия".to_string());

            let discount_text = row.select(&discount_selector)
                .next().map(|d| d.text().collect::<String>())
                .unwrap_or("-0%".to_string());

            let discount_clean = discount_text.trim().trim_start_matches('-').trim_end_matches('%');
            let discount_value = discount_clean.parse::<u32>().unwrap_or(0);
            if discount_value != discount_filter {
                continue;
            }

            let old_price = row.select(&price_old_selector)
                .next().map(|p| p.text().collect::<String>())
                .unwrap_or("неизвестно".to_string());

            let new_price = row.select(&price_new_selector)
                .next().map(|p| p.text().collect::<String>())
                .unwrap_or("неизвестно".to_string());

            let entry = format!("{} | {} | {}", title, old_price.trim(), new_price.trim());
            all_games.push(entry);
        }

        thread::sleep(time::Duration::from_secs(2));
    }

    let mut file = File::create(&output_file).map_err(|e| e.to_string())?;
    for game in &all_games {
        writeln!(file, "{}", game).unwrap();
    }

    Ok(all_games)
}

fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![parse_steam])
        .run(tauri::generate_context!())
        .expect("Ошибка запуска Tauri приложения");
}
