//! Rate limiter implementation for the Kraken API

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Rate limiter for the Kraken API
///
/// Kraken API has different rate limits for different endpoints:
/// - Tier 1: 15 calls per 45 seconds
/// - Tier 2: 20 calls per 60 seconds
/// - Tier 3: 20 calls per 60 seconds
/// - Tier 4: 15 calls per 60 seconds
///
/// This rate limiter uses a token bucket algorithm to enforce these limits.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Rate limit tiers
    tiers: Arc<Mutex<HashMap<Tier, TokenBucket>>>,
}

/// Rate limit tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tier {
    /// Tier 1: Public endpoints
    Tier1,
    
    /// Tier 2: Private endpoints
    Tier2,
    
    /// Tier 3: Private endpoints with higher limits
    Tier3,
    
    /// Tier 4: Private endpoints with lower limits
    Tier4,
}

/// Token bucket for rate limiting
#[derive(Debug, Clone)]
struct TokenBucket {
    /// Maximum number of tokens
    max_tokens: u32,
    
    /// Current number of tokens
    tokens: u32,
    
    /// Time between token refills
    refill_time: Duration,
    
    /// Last refill time
    last_refill: Instant,
}

impl TokenBucket {
    /// Create a new token bucket
    fn new(max_tokens: u32, refill_time: Duration) -> Self {
        Self {
            max_tokens,
            tokens: max_tokens,
            refill_time,
            last_refill: Instant::now(),
        }
    }
    
    /// Take a token from the bucket
    fn take(&mut self) -> bool {
        self.refill();
        
        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
    
    /// Refill the bucket
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        
        if elapsed >= self.refill_time {
            let refills = (elapsed.as_secs_f64() / self.refill_time.as_secs_f64()) as u32;
            let new_tokens = self.tokens + refills;
            self.tokens = new_tokens.min(self.max_tokens);
            self.last_refill = now;
        }
    }
    
    /// Get the time until the next token is available
    fn time_until_next_token(&mut self) -> Duration {
        self.refill();
        
        if self.tokens > 0 {
            Duration::from_secs(0)
        } else {
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_refill);
            
            if elapsed >= self.refill_time {
                Duration::from_secs(0)
            } else {
                self.refill_time - elapsed
            }
        }
    }
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new() -> Self {
        let mut tiers = HashMap::new();
        
        // Tier 1: 15 calls per 45 seconds
        tiers.insert(Tier::Tier1, TokenBucket::new(15, Duration::from_secs(45)));
        
        // Tier 2: 20 calls per 60 seconds
        tiers.insert(Tier::Tier2, TokenBucket::new(20, Duration::from_secs(60)));
        
        // Tier 3: 20 calls per 60 seconds
        tiers.insert(Tier::Tier3, TokenBucket::new(20, Duration::from_secs(60)));
        
        // Tier 4: 15 calls per 60 seconds
        tiers.insert(Tier::Tier4, TokenBucket::new(15, Duration::from_secs(60)));
        
        Self {
            tiers: Arc::new(Mutex::new(tiers)),
        }
    }
    
    /// Acquire a token for the given tier
    pub async fn acquire(&self, tier: Tier) -> Duration {
        let mut tiers = self.tiers.lock().await;
        
        let bucket = tiers.get_mut(&tier).unwrap();
        
        if bucket.take() {
            Duration::from_secs(0)
        } else {
            bucket.time_until_next_token()
        }
    }
    
    /// Wait for a token to be available
    pub async fn wait(&self, tier: Tier) {
        let wait_time = self.acquire(tier).await;
        
        if wait_time > Duration::from_secs(0) {
            tokio::time::sleep(wait_time).await;
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
