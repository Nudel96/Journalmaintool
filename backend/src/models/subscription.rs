use serde::{Deserialize, Serialize};

/// Subscription tier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionTier {
    None,
    Paid,
}

impl SubscriptionTier {
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionTier::None => "none",
            SubscriptionTier::Paid => "paid",
        }
    }
}

impl From<String> for SubscriptionTier {
    fn from(s: String) -> Self {
        match s.as_str() {
            "paid" => SubscriptionTier::Paid,
            _ => SubscriptionTier::None,
        }
    }
}

/// Subscription interval
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionInterval {
    Month,      // 1 month - $7/month
    Month6,     // 6 months - $5/month ($30 total)
    Year,       // 12 months - $4/month ($48 total)
}

impl SubscriptionInterval {
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionInterval::Month => "month",
            SubscriptionInterval::Month6 => "month_6",
            SubscriptionInterval::Year => "year",
        }
    }

    pub fn price_cents(&self) -> i64 {
        match self {
            SubscriptionInterval::Month => 700,      // $7.00
            SubscriptionInterval::Month6 => 3000,    // $30.00 (6 months)
            SubscriptionInterval::Year => 4800,      // $48.00 (12 months)
        }
    }

    pub fn monthly_price_cents(&self) -> i64 {
        match self {
            SubscriptionInterval::Month => 700,      // $7/month
            SubscriptionInterval::Month6 => 500,     // $5/month
            SubscriptionInterval::Year => 400,       // $4/month
        }
    }

    pub fn savings_percentage(&self) -> i32 {
        match self {
            SubscriptionInterval::Month => 0,
            SubscriptionInterval::Month6 => 29,      // 29% savings
            SubscriptionInterval::Year => 43,        // 43% savings
        }
    }
}

impl From<String> for SubscriptionInterval {
    fn from(s: String) -> Self {
        match s.as_str() {
            "month" => SubscriptionInterval::Month,
            "month_6" => SubscriptionInterval::Month6,
            "year" => SubscriptionInterval::Year,
            _ => SubscriptionInterval::Month,
        }
    }
}

/// Subscription status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    None,
    Active,
    Canceled,
    PastDue,
    Trialing,
}

impl SubscriptionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionStatus::None => "none",
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::PastDue => "past_due",
            SubscriptionStatus::Trialing => "trialing",
        }
    }
}

impl From<String> for SubscriptionStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "active" => SubscriptionStatus::Active,
            "canceled" => SubscriptionStatus::Canceled,
            "past_due" => SubscriptionStatus::PastDue,
            "trialing" => SubscriptionStatus::Trialing,
            _ => SubscriptionStatus::None,
        }
    }
}

/// Create checkout session request
#[derive(Debug, Deserialize)]
pub struct CreateCheckoutRequest {
    pub interval: String, // "month", "month_6", or "year"
}

/// Checkout session response
#[derive(Debug, Serialize)]
pub struct CheckoutSessionResponse {
    pub session_id: String,
    pub url: String,
}

/// Stripe Price IDs (to be set in environment or config)
/// These will be created in Stripe Dashboard
pub struct StripePriceIds {
    pub month: &'static str,
    pub month_6: &'static str,
    pub year: &'static str,
}

impl StripePriceIds {
    pub fn get_price_id(&self, interval: &SubscriptionInterval) -> &'static str {
        match interval {
            SubscriptionInterval::Month => self.month,
            SubscriptionInterval::Month6 => self.month_6,
            SubscriptionInterval::Year => self.year,
        }
    }
}

// Placeholder Price IDs - will be replaced with real ones from Stripe
pub const STRIPE_PRICE_IDS: StripePriceIds = StripePriceIds {
    month: "price_1month_placeholder",
    month_6: "price_6month_placeholder",
    year: "price_year_placeholder",
};

