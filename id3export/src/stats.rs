use common::thousand_separated;
use serde::Serialize;
use std::collections::HashMap;

pub type StatsMap = HashMap<String, Statistics>;

pub fn update_stats(stats: &mut StatsMap, key: &str, ms: u64, size: u64) {
    let stats_entry = stats.entry(key.to_string()).or_insert(Statistics::new());
    stats_entry.add(ms, size);
}

pub fn calc_avg(stats: &mut StatsMap) {
    for (_, stats) in stats {
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

pub fn export_summary_csv(
    stats: &StatsMap,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
    pub file_count: i32,
    pub total_ms: u64,
    pub avg_ms: u64,
    pub total_size: u64,
    pub avg_size: u64,
}

impl Statistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, ms: u64, size: u64) {
        self.file_count += 1;
        self.total_ms += ms;
        self.total_size += size;
    }

    pub fn calc_avg_ms(&mut self) {
        self.avg_ms = if self.file_count > 0 {
            self.total_ms / self.file_count as u64
        } else {
            0
        }
    }

    pub fn calc_avg_size(&mut self) {
        self.avg_size = if self.file_count > 0 {
            self.total_size / self.file_count as u64
        } else {
            0
        }
    }
}
