use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub action: String,
    pub detail: String,
}

#[derive(Debug, Default)]
pub struct AuditLog {
    pub entries: Vec<AuditEntry>,
}

impl AuditLog {
    pub fn new() -> Self {
        AuditLog { entries: Vec::new() }
    }

    pub fn record(&mut self, action: &str, detail: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.entries.push(AuditEntry {
            timestamp,
            action: action.to_string(),
            detail: detail.to_string(),
        });
    }

    pub fn print_all(&self) {
        for e in &self.entries {
            println!("[{}] {} — {}", e.timestamp, e.action, e.detail);
        }
    }
}