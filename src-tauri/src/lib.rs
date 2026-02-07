use serde::Serialize;
use std::time::Instant;
use tauri::Emitter;

#[tauri::command]
fn echo_key(ch: String) -> EchoResponse {
    let received_at = epoch_micros();
    EchoResponse {
        ch,
        received_at_us: received_at,
    }
}

#[derive(Serialize)]
struct EchoResponse {
    ch: String,
    received_at_us: f64,
}

#[tauri::command]
async fn start_flood(
    lines: u32,
    app: tauri::AppHandle,
) -> Result<FloodSummary, String> {
    let count = if lines == 0 { 10_000 } else { lines };
    let flood_id = epoch_micros_u64();
    let start = Instant::now();

    const CHUNK_LINES: u32 = 100;
    let mut chunk = String::new();
    for i in 0..count {
        chunk.push_str(&format!(
            "[{:>5}/{:>5}] The quick brown fox jumps over the lazy dog -- line {}\r\n",
            i + 1,
            count,
            i + 1,
        ));

        if i % CHUNK_LINES == CHUNK_LINES - 1 {
            app.emit("terminal-output", &chunk)
                .map_err(|e| e.to_string())?;
            chunk.clear();
            tokio::task::yield_now().await;
        }
    }

    if !chunk.is_empty() {
        app.emit("terminal-output", &chunk)
            .map_err(|e| e.to_string())?;
    }

    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    let done_msg = format!(
        "\r\n--- flood complete: {} lines in {:.1} ms ---\r\n",
        count, elapsed_ms
    );
    app.emit("terminal-output", &done_msg)
        .map_err(|e| e.to_string())?;

    app.emit(
        "terminal-flood-done",
        FloodDoneEvent {
            flood_id,
            lines: count,
            elapsed_ms,
        },
    )
    .map_err(|e| e.to_string())?;

    Ok(FloodSummary {
        flood_id,
        lines: count,
        elapsed_ms,
    })
}

#[derive(Serialize)]
struct FloodSummary {
    flood_id: u64,
    lines: u32,
    elapsed_ms: f64,
}

#[derive(Clone, Serialize)]
struct FloodDoneEvent {
    flood_id: u64,
    lines: u32,
    elapsed_ms: f64,
}

#[tauri::command]
async fn start_stream(
    lines: u32,
    delay_us: u64,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let count = if lines == 0 { 100 } else { lines };
    let delay = tokio::time::Duration::from_micros(delay_us);

    for i in 0..count {
        let line = format!("[stream {:>5}/{}] output line\r\n", i + 1, count);
        app.emit("terminal-output", &line)
            .map_err(|e| e.to_string())?;
        if !delay.is_zero() {
            tokio::time::sleep(delay).await;
        }
    }
    Ok(())
}

fn epoch_micros() -> f64 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        * 1_000_000.0
}

fn epoch_micros_u64() -> u64 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u64
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            echo_key,
            start_flood,
            start_stream,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
