use futures::future::join_all;
use rusqlite::{Connection, Result, params};
use scraper::{Html, Selector};
use std::{collections::HashSet, fs, sync::Arc};
use tokio::sync::{Mutex, Semaphore};
use clap::Parser;

/// Fetch titles from URLs and store them in SQLite
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input list of URLs
    #[arg(short, long, default_value = "wr-list.txt")]
    input: String,

    /// SQLite database file to save titles
    #[arg(short, long, default_value = "wr-urls.db")]
    output: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // SQLite データベース接続を Arc<Mutex> で共有
    let conn = Arc::new(Mutex::new(Connection::open(&args.output)?));
    {
        let conn = conn.lock().await;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS url_titles (
                url TEXT PRIMARY KEY,
                title TEXT
            )",
            [],
        )?;
    }

    // すでにアクセス済み URL を取得
    let visited_urls: HashSet<String> = {
        let conn = conn.lock().await;
        conn.prepare("SELECT url FROM url_titles")?
            .query_map([], |row| row.get(0))?
            .collect::<Result<HashSet<String>, _>>()?
    };
    let visited_urls = Arc::new(Mutex::new(visited_urls));

    // 入力ファイルを読み込む
    let urls: Vec<String> = fs::read_to_string(&args.input)
        .expect("Failed to read input file")
        .lines()
        .map(|s| s.trim().to_string())
        .collect();

    // 並列アクセス数の制限（例: 最大10タスク）
    let semaphore = Arc::new(Semaphore::new(10));

    let tasks: Vec<_> = urls
        .into_iter()
        .map(|url| {
            let conn = Arc::clone(&conn);
            let visited_urls = Arc::clone(&visited_urls);
            let semaphore = Arc::clone(&semaphore);

            tokio::spawn(async move {
                // 同時実行数制御
                let _permit = semaphore.acquire().await.unwrap();

                // すでにアクセス済みチェック
                {
                    let visited = visited_urls.lock().await;
                    if visited.contains(&url) {
                        return;
                    }
                }

                // HTTP GET
                let title = match reqwest::get(&url).await {
                    Ok(resp) => match resp.text().await {
                        Ok(text) => {
                            let document = Html::parse_document(&text);
                            let selector = Selector::parse("title").unwrap();
                            document
                                .select(&selector)
                                .next()
                                .map(|e| e.text().collect::<String>())
                        }
                        Err(_) => None,
                    },
                    Err(_) => None,
                };

                if let Some(title) = title {
                    // DB に保存
                    {
                        let conn = conn.lock().await;
                        if let Err(e) = conn.execute(
                            "INSERT OR IGNORE INTO url_titles (url, title) VALUES (?1, ?2)",
                            params![url, title],
                        ) {
                            eprintln!("DB insert error for {}: {}", url, e);
                        }
                    }

                    // すでにアクセス済みセットに追加
                    let mut visited = visited_urls.lock().await;
                    visited.insert(url.clone());

                    println!("Saved: {} -> {}", url, title);
                } else {
                    eprintln!("Failed to get title for {}", url);
                }
            })
        })
        .collect();

    // 全タスク完了を待つ
    join_all(tasks).await;

    Ok(())
}
