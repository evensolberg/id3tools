use anyhow::Result;
use common::thousand_separated;
use serde::Serialize;
use std::collections::BTreeMap;

pub type StatsMap = BTreeMap<String, Statistics>;

pub fn update_stats(stats: &mut StatsMap, key: &str, ms: u64, size: u64) {
    let stats_entry = stats.entry(key.to_string()).or_default();
    stats_entry.add(ms, size);
}

pub fn calc_avg(stats: &mut StatsMap) {
    for stats in stats.values_mut() {
        stats.calc_avg_ms();
        stats.calc_avg_size();
    }
}

pub fn print_stats(stats: &StatsMap) {
    println!("| Type     | File count | Total Duration (ms) | Avg Duration (ms) | Total Size (bytes) | Avg Size (bytes) |");
    println!("|----------|------------|---------------------|-------------------|--------------------|------------------|");
    for (key, stats) in stats {
        println!(
            "| {key:<8} | {:>10} | {:>19} | {:>17} | {:>18} | {:>16} |",
            thousand_separated(stats.file_count),
            thousand_separated(stats.total_ms),
            thousand_separated(stats.avg_ms),
            thousand_separated(stats.total_size),
            thousand_separated(stats.avg_size)
        );
    }
}

pub fn export_summary_csv(stats: &StatsMap, filename: &str) -> Result<()> {
    let mut wtr = csv::Writer::from_path(filename)?;
    wtr.write_record([
        "Type",
        "File count",
        "Total Duration (ms)",
        "Avg Duration (ms)",
        "Total Size (bytes)",
        "Avg Size (bytes)",
    ])?;
    for (key, stats) in stats {
        let file_count = stats.file_count.to_string();
        let total_ms = stats.total_ms.to_string();
        let avg_ms = stats.avg_ms.to_string();
        let total_size = stats.total_size.to_string();
        let avg_size = stats.avg_size.to_string();

        wtr.write_record([key, &file_count, &total_ms, &avg_ms, &total_size, &avg_size])?;
    }
    wtr.flush()?;
    Ok(())
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct Statistics {
    pub file_count: u64,
    pub total_ms: u64,
    pub avg_ms: u64,
    pub total_size: u64,
    pub avg_size: u64,
}

impl Statistics {
    pub const fn add(&mut self, ms: u64, size: u64) {
        self.file_count += 1;
        self.total_ms += ms;
        self.total_size += size;
    }

    pub fn calc_avg_ms(&mut self) {
        self.avg_ms = self.total_ms.checked_div(self.file_count).unwrap_or(0);
    }

    pub fn calc_avg_size(&mut self) {
        self.avg_size = self.total_size.checked_div(self.file_count).unwrap_or(0);
    }
}
