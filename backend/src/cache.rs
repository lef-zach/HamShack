use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Spot {
    pub callsign: String,
    pub frequency: f64,
    pub mode: String,
    pub spotter: String,
    pub timestamp: u64,
    pub grid: Option<String>,
    pub snr: Option<i32>,
}

#[derive(Clone)]
pub struct SpotCache {
    spots: Arc<RwLock<HashMap<String, Spot>>>,
    max_size: usize,
    retention: Duration,
}

impl SpotCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            spots: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            retention: Duration::from_secs(1800), // 30 minutes
        }
    }
    
    pub fn add_spot(&self, spot: Spot) {
        let mut spots = self.spots.write().unwrap();
        
        // Clean up expired spots
        self.cleanup(&mut spots);
        
        // Add new spot
        spots.insert(spot.callsign.clone(), spot);
    }
    
    pub fn get_spots(&self) -> Vec<Spot> {
        let spots = self.spots.read().unwrap();
        
        // Filter and collect active spots
        spots.values()
            .filter(|spot| self.is_active(spot))
            .cloned()
            .collect()
    }
    
    fn is_active(&self, spot: &Spot) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        now.saturating_sub(spot.timestamp) < self.retention.as_secs()
    }
    
    fn cleanup(&self, spots: &mut HashMap<String, Spot>) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Remove expired spots
        spots.retain(|_, spot| now.saturating_sub(spot.timestamp) < self.retention.as_secs());
        
        // Enforce size limit
        if spots.len() > self.max_size {
            // Collect keys to remove first, then remove them
            let mut keys_with_timestamps: Vec<(String, u64)> = spots
                .iter()
                .map(|(key, spot)| (key.clone(), spot.timestamp))
                .collect();
            
            keys_with_timestamps.sort_by_key(|(_, timestamp)| *timestamp);
            
            let to_remove = spots.len() - self.max_size;
            for (key, _) in keys_with_timestamps.iter().take(to_remove) {
                spots.remove(key);
            }
        }
    }
}