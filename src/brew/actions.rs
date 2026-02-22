use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

use crate::app::AppEvent;
use crate::brew::types::PackageKind;

pub async fn run_brew_action(
    action: &str,
    name: &str,
    kind: PackageKind,
    tx: mpsc::Sender<AppEvent>,
) {
    let kind_flag = match kind {
        PackageKind::Cask => Some("--cask"),
        PackageKind::Formula => None,
    };

    let mut cmd = Command::new("brew");
    cmd.arg(action);
    if let Some(flag) = kind_flag {
        cmd.arg(flag);
    }
    cmd.arg(name);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            let _ = tx.send(AppEvent::ActionOutput(format!("Error: {e}"))).await;
            let _ = tx.send(AppEvent::ActionDone(false)).await;
            return;
        }
    };

    // Stream stdout
    if let Some(stdout) = child.stdout.take() {
        let tx2 = tx.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let _ = tx2.send(AppEvent::ActionOutput(line)).await;
            }
        });
    }

    // Stream stderr
    if let Some(stderr) = child.stderr.take() {
        let tx3 = tx.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let _ = tx3.send(AppEvent::ActionOutput(line)).await;
            }
        });
    }

    let status = child.wait().await;
    let success = status.map(|s| s.success()).unwrap_or(false);
    let _ = tx.send(AppEvent::ActionDone(success)).await;
}
