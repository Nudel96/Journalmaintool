# Stripe Integration Setup Guide

## 1. Create Stripe Account

1. Go to https://stripe.com and create an account
2. Activate your account (you can use test mode for development)

## 2. Get API Keys

1. Go to https://dashboard.stripe.com/test/apikeys
2. Copy your **Secret key** (starts with `sk_test_`)
3. Add it to `backend/.env`:
   ```
   STRIPE_SECRET_KEY=sk_test_your_actual_key_here
   ```

## 3. Create Products and Prices

### Option A: Using Stripe Dashboard (Recommended)

1. Go to https://dashboard.stripe.com/test/products
2. Click **"+ Add product"**

**Product 1: Trading Journal - 1 Month**
- Name: `Trading Journal - 1 Month`
- Description: `Monthly subscription to Trading Journal`
- Pricing:
  - Model: `Recurring`
  - Price: `$7.00 USD`
  - Billing period: `Monthly`
- Click **Save product**
- Copy the **Price ID** (starts with `price_`)

**Product 2: Trading Journal - 6 Months**
- Name: `Trading Journal - 6 Months`
- Description: `6-month subscription to Trading Journal (29% savings)`
- Pricing:
  - Model: `Recurring`
  - Price: `$30.00 USD`
  - Billing period: `Every 6 months`
- Click **Save product**
- Copy the **Price ID**

**Product 3: Trading Journal - 12 Months**
- Name: `Trading Journal - 12 Months`
- Description: `Annual subscription to Trading Journal (43% savings)`
- Pricing:
  - Model: `Recurring`
  - Price: `$48.00 USD`
  - Billing period: `Yearly`
- Click **Save product**
- Copy the **Price ID**

### Option B: Using Stripe CLI

```bash
# Install Stripe CLI: https://stripe.com/docs/stripe-cli

# 1 Month Plan
stripe products create \
  --name="Trading Journal - 1 Month" \
  --description="Monthly subscription"

stripe prices create \
  --product=prod_xxx \
  --unit-amount=700 \
  --currency=usd \
  --recurring[interval]=month

# 6 Month Plan
stripe products create \
  --name="Trading Journal - 6 Months" \
  --description="6-month subscription (29% savings)"

stripe prices create \
  --product=prod_yyy \
  --unit-amount=3000 \
  --currency=usd \
  --recurring[interval]=month \
  --recurring[interval_count]=6

# 12 Month Plan
stripe products create \
  --name="Trading Journal - 12 Months" \
  --description="Annual subscription (43% savings)"

stripe prices create \
  --product=prod_zzz \
  --unit-amount=4800 \
  --currency=usd \
  --recurring[interval]=year
```

## 4. Update Price IDs in Code

Edit `backend/src/models/subscription.rs`:

```rust
pub const STRIPE_PRICE_IDS: StripePriceIds = StripePriceIds {
    month: "price_1xxxxxxxxxxxxx",      // Replace with your 1-month price ID
    month_6: "price_2xxxxxxxxxxxxx",    // Replace with your 6-month price ID
    year: "price_3xxxxxxxxxxxxx",       // Replace with your 12-month price ID
};
```

## 5. Setup Webhook Endpoint

### For Local Development (using Stripe CLI):

1. Install Stripe CLI: https://stripe.com/docs/stripe-cli
2. Login: `stripe login`
3. Forward webhooks to local server:
   ```bash
   stripe listen --forward-to localhost:3000/webhooks/stripe
   ```
4. Copy the webhook signing secret (starts with `whsec_`)
5. Add to `backend/.env`:
   ```
   STRIPE_WEBHOOK_SECRET=whsec_your_webhook_secret_here
   ```

### For Production:

1. Go to https://dashboard.stripe.com/webhooks
2. Click **"+ Add endpoint"**
3. Endpoint URL: `https://your-domain.com/webhooks/stripe`
4. Select events to listen to:
   - `checkout.session.completed`
   - `customer.subscription.updated`
   - `customer.subscription.deleted`
5. Copy the **Signing secret**
6. Add to production environment variables

## 6. Test the Integration

1. Start your backend: `cd backend && cargo run`
2. Start your frontend: `cd frontend && npm run dev`
3. Go to http://localhost:5173/pricing
4. Click "Subscribe" on any plan
5. Use Stripe test card: `4242 4242 4242 4242`
   - Expiry: Any future date
   - CVC: Any 3 digits
   - ZIP: Any 5 digits

## 7. Verify Webhook Events

When using Stripe CLI (`stripe listen`), you'll see webhook events in the terminal:

```
✔ Received event checkout.session.completed
✔ Received event customer.subscription.created
```

Check your backend logs to confirm the subscription was updated in the database.

## Troubleshooting

### "Invalid API Key"
- Make sure you're using the correct key from https://dashboard.stripe.com/test/apikeys
- Ensure the key starts with `sk_test_` for test mode

### "Webhook signature verification failed"
- Make sure `STRIPE_WEBHOOK_SECRET` matches the secret from `stripe listen` or your webhook endpoint
- Check that the signature header is being passed correctly

### "Price not found"
- Verify the Price IDs in `backend/src/models/subscription.rs` match your Stripe dashboard
- Make sure you're using Price IDs, not Product IDs

## Next Steps

Once everything works in test mode:

1. Switch to live mode in Stripe Dashboard
2. Create live products and prices
3. Update `STRIPE_SECRET_KEY` with live key (starts with `sk_live_`)
4. Update webhook endpoint to production URL
5. Update Price IDs in code with live price IDs

