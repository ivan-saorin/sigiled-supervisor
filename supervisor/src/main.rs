// sigiled-supervisor — SIGILED gestisce tutto di sé tranne la propria
// resurrezione: la resurrezione è questo servizio (requisiti DEC-01..07).
// Un dovere solo, un file solo. Zero dipendenze runtime dallo stack: bearer
// statico da env, mai OIDC (l'IdP può essere morto insieme al resto).
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use std::io::Write;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
struct Cfg {
    token: String,
    repo: String,          // SIGILED_REPO_DIR: checkout di sigiled sul box
    restart_cmd: String,   // SUPERVISOR_RESTART_CMD, {sha} sostituito
    health_url: String,    // SIGILED_HEALTH_URL
    log_path: String,      // append-only, la memoria del supervisor
    busy: Arc<AtomicBool>, // un restart alla volta (409 sul secondo)
}

fn env_or(k: &str, d: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| d.into())
}

fn sh(dir: &str, cmd: &str) -> (bool, String) {
    let out = Command::new("sh").arg("-lc").arg(cmd).current_dir(dir).output();
    match out {
        Ok(o) => {
            let text = format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
            (o.status.success(), text)
        }
        Err(e) => (false, format!("spawn: {e}")),
    }
}

// Dentro gli handler i comandi passano dal blocking pool: su un box a 1 CPU
// il runtime ha 1 worker, e un curl bloccante verso noi stessi (health check)
// sarebbe un deadlock a tempo.
async fn sh_async(dir: String, cmd: String) -> (bool, String) {
    tokio::task::spawn_blocking(move || sh(&dir, &cmd)).await.unwrap_or((false, "join".into()))
}

fn log_line(cfg: &Cfg, line: &str) {
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&cfg.log_path) {
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let _ = writeln!(f, "{ts} {line}");
    }
}

fn authed(cfg: &Cfg, headers: &HeaderMap) -> bool {
    let given = headers.get("authorization").and_then(|v| v.to_str().ok()).and_then(|v| v.strip_prefix("Bearer "));
    // confronto a lunghezza-e-xor costante, come il control plane
    given.is_some_and(|g| g.len() == cfg.token.len() && g.bytes().zip(cfg.token.bytes()).fold(0u8, |a, (x, y)| a | (x ^ y)) == 0)
}

fn deployed_sha(cfg: &Cfg) -> String {
    sh(&cfg.repo, "git rev-parse HEAD").1.trim().to_string()
}

async fn healthy(cfg: &Cfg) -> bool {
    sh_async("/".into(), format!("curl -sf -m 5 {} >/dev/null", cfg.health_url)).await.0
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn status(State(cfg): State<Cfg>, headers: HeaderMap) -> (StatusCode, Json<Value>) {
    if !authed(&cfg, &headers) {
        return (StatusCode::UNAUTHORIZED, Json(json!({"detail": "bad bearer"})));
    }
    let last = std::fs::read_to_string(&cfg.log_path).ok().and_then(|s| s.lines().last().map(str::to_string));
    let ok = healthy(&cfg).await;
    (StatusCode::OK, Json(json!({
        "deployed_sha": deployed_sha(&cfg),
        "healthy": ok,
        "last_restart": last,
    })))
}

async fn restart(State(cfg): State<Cfg>, headers: HeaderMap, body: Option<Json<Value>>) -> (StatusCode, Json<Value>) {
    if !authed(&cfg, &headers) {
        return (StatusCode::UNAUTHORIZED, Json(json!({"detail": "bad bearer"})));
    }
    if cfg.busy.swap(true, Ordering::SeqCst) {
        return (StatusCode::CONFLICT, Json(json!({"detail": "restart in progress"})));
    }
    let t0 = Instant::now();
    let previous = deployed_sha(&cfg);
    // sha esplicito = rollback (DEC-04); default: l'attuale pin remoto
    let sha = body.and_then(|Json(v)| v["sha"].as_str().map(str::to_string));
    let checkout = match &sha {
        Some(s) => format!("git fetch --all -q && git checkout -q {s}"),
        None => "git fetch -q origin master && git checkout -q origin/master".into(),
    };
    let (ok_git, log_git) = sh_async(cfg.repo.clone(), checkout).await;
    let (ok_run, log_run) = if ok_git {
        let cmd = cfg.restart_cmd.replace("{sha}", sha.as_deref().unwrap_or("HEAD"));
        sh_async(cfg.repo.clone(), cmd).await
    } else {
        (false, String::new())
    };
    // attesa health: fino a 60s
    let mut is_healthy = false;
    if ok_run {
        for _ in 0..12 {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            if healthy(&cfg).await { is_healthy = true; break; }
        }
    }
    let new = deployed_sha(&cfg);
    let dur = t0.elapsed().as_secs();
    let tail: String = format!("{log_git}\n{log_run}").lines().rev().take(15).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n");
    log_line(&cfg, &format!("restart prev={previous} new={new} healthy={is_healthy} dur={dur}s"));
    cfg.busy.store(false, Ordering::SeqCst);
    (StatusCode::OK, Json(json!({
        "previous_sha": previous, "new_sha": new, "healthy": is_healthy,
        "duration_secs": dur, "log_tail": tail,
    })))
}

#[tokio::main]
async fn main() {
    let cfg = Cfg {
        token: std::env::var("SUPERVISOR_TOKEN").expect("SUPERVISOR_TOKEN is required"),
        repo: env_or("SIGILED_REPO_DIR", "/opt/sigiled"),
        restart_cmd: env_or("SUPERVISOR_RESTART_CMD", "docker compose up -d --build sigiled"),
        health_url: env_or("SIGILED_HEALTH_URL", "http://localhost:8080/healthz"),
        log_path: env_or("SUPERVISOR_LOG", "/var/log/sigiled-supervisor.log"),
        busy: Arc::new(AtomicBool::new(false)),
    };
    let app = Router::new()
        .route("/health", get(health))
        .route("/sigiled/status", get(status))
        .route("/sigiled/restart", post(restart))
        .with_state(cfg);
    let port = env_or("PORT", "9090");
    let addr = format!("0.0.0.0:{port}");
    println!("sigiled-supervisor on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
