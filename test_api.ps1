# Trading Journal API Test Script
# Make sure backend is running: cd backend && cargo run

$API_URL = "http://localhost:3000"
$TOKEN = ""

Write-Host "=== TRADING JOURNAL API TEST ===" -ForegroundColor Cyan
Write-Host ""

# 1. Health Check
Write-Host "1. Testing Health Check..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$API_URL/health" -Method Get
    Write-Host "✓ Health Check: $response" -ForegroundColor Green
} catch {
    Write-Host "✗ Health Check Failed: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""

# 2. Register User
Write-Host "2. Testing User Registration..." -ForegroundColor Yellow
$registerData = @{
    name = "Test Trader"
    email = "trader@test.com"
    password = "testpassword123"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$API_URL/auth/register" -Method Post -Body $registerData -ContentType "application/json"
    $TOKEN = $response.token
    Write-Host "✓ User Registered: $($response.user.name) ($($response.user.email))" -ForegroundColor Green
    Write-Host "  Token: $($TOKEN.Substring(0, 20))..." -ForegroundColor Gray
} catch {
    # User might already exist, try login
    Write-Host "  User exists, trying login..." -ForegroundColor Yellow
    
    $loginData = @{
        email = "trader@test.com"
        password = "testpassword123"
    } | ConvertTo-Json
    
    try {
        $response = Invoke-RestMethod -Uri "$API_URL/auth/login" -Method Post -Body $loginData -ContentType "application/json"
        $TOKEN = $response.token
        Write-Host "✓ User Logged In: $($response.user.name)" -ForegroundColor Green
    } catch {
        Write-Host "✗ Login Failed: $_" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""

# 3. Get Current User
Write-Host "3. Testing Get Current User..." -ForegroundColor Yellow
$headers = @{
    "Authorization" = "Bearer $TOKEN"
}

try {
    $response = Invoke-RestMethod -Uri "$API_URL/auth/me" -Method Get -Headers $headers
    Write-Host "✓ Current User: $($response.name) ($($response.email))" -ForegroundColor Green
    Write-Host "  Subscription: $($response.subscription_tier) - $($response.subscription_status)" -ForegroundColor Gray
} catch {
    Write-Host "✗ Get User Failed: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""

# 4. Create Trade
Write-Host "4. Testing Create Trade..." -ForegroundColor Yellow
$tradeData = @{
    symbol = "EURUSD"
    direction = "long"
    entry_price = 1.0850
    exit_price = 1.0920
    quantity = 10000
    entry_time = (Get-Date).AddHours(-2).ToString("o")
    exit_time = (Get-Date).ToString("o")
    fees = 5.0
    notes = "Good breakout trade"
    tags = @("breakout", "trend-following")
    setup_type = "Support/Resistance"
    mistakes = @()
    emotions = @("confident")
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$API_URL/trades" -Method Post -Body $tradeData -ContentType "application/json" -Headers $headers
    $TRADE_ID = $response.id
    Write-Host "✓ Trade Created: $($response.symbol) $($response.direction)" -ForegroundColor Green
    Write-Host "  P&L: $$($response.pnl) ($($response.pnl_percentage)%)" -ForegroundColor Gray
    Write-Host "  Trade ID: $TRADE_ID" -ForegroundColor Gray
} catch {
    Write-Host "✗ Create Trade Failed: $_" -ForegroundColor Red
    Write-Host $_.Exception.Response.StatusCode -ForegroundColor Red
    exit 1
}

Write-Host ""

# 5. Create Another Trade (Losing)
Write-Host "5. Creating Losing Trade..." -ForegroundColor Yellow
$tradeData2 = @{
    symbol = "GBPUSD"
    direction = "short"
    entry_price = 1.2650
    exit_price = 1.2680
    quantity = 5000
    entry_time = (Get-Date).AddHours(-1).ToString("o")
    exit_time = (Get-Date).ToString("o")
    fees = 3.0
    notes = "Stopped out"
    tags = @("reversal")
    setup_type = "Trendline"
    mistakes = @("too early")
    emotions = @("fearful")
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$API_URL/trades" -Method Post -Body $tradeData2 -ContentType "application/json" -Headers $headers
    Write-Host "✓ Trade Created: $($response.symbol) $($response.direction)" -ForegroundColor Green
    Write-Host "  P&L: $$($response.pnl) ($($response.pnl_percentage)%)" -ForegroundColor Gray
} catch {
    Write-Host "✗ Create Trade Failed: $_" -ForegroundColor Red
}

Write-Host ""

# 6. List All Trades
Write-Host "6. Testing List Trades..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$API_URL/trades" -Method Get -Headers $headers
    Write-Host "✓ Found $($response.Count) trades" -ForegroundColor Green
    foreach ($trade in $response) {
        $pnlColor = if ($trade.pnl -gt 0) { "Green" } else { "Red" }
        Write-Host "  - $($trade.symbol) $($trade.direction): $$($trade.pnl)" -ForegroundColor $pnlColor
    }
} catch {
    Write-Host "✗ List Trades Failed: $_" -ForegroundColor Red
}

Write-Host ""

# 7. Get Single Trade
Write-Host "7. Testing Get Single Trade..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$API_URL/trades/$TRADE_ID" -Method Get -Headers $headers
    Write-Host "✓ Trade Details:" -ForegroundColor Green
    Write-Host "  Symbol: $($response.symbol)" -ForegroundColor Gray
    Write-Host "  Direction: $($response.direction)" -ForegroundColor Gray
    Write-Host "  Entry: $$($response.entry_price)" -ForegroundColor Gray
    Write-Host "  Exit: $$($response.exit_price)" -ForegroundColor Gray
    Write-Host "  P&L: $$($response.pnl) ($($response.pnl_percentage)%)" -ForegroundColor Gray
} catch {
    Write-Host "✗ Get Trade Failed: $_" -ForegroundColor Red
}

Write-Host ""

# 8. Filter Trades
Write-Host "8. Testing Trade Filters..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$API_URL/trades?symbol=EURUSD" -Method Get -Headers $headers
    Write-Host "✓ Filtered by EURUSD: $($response.Count) trades" -ForegroundColor Green
} catch {
    Write-Host "✗ Filter Trades Failed: $_" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== ALL TESTS COMPLETED ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Implement Analytics Service (Win-Rate, Profit-Factor, etc.)" -ForegroundColor White
Write-Host "2. Build Frontend (Trade Form, List, Dashboard)" -ForegroundColor White
Write-Host "3. Add Charts with Chart.js" -ForegroundColor White

