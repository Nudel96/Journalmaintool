# 🚀 TRADING JOURNAL - COMPLETE REBUILD VIBECODE

**Version:** 2.0  
**Target Agent:** New AI Agent (Complete Rebuild)  
**Project:** PriceActionTalk Trading Journal  
**Methodology:** Spec-Driven Development  
**Estimated Time:** 40-60 hours total development

---

## 📋 EXECUTIVE SUMMARY

You are tasked with building a **professional Trading Journal application** from scratch. This is a complete rebuild - the user will delete the existing project and start fresh. Your mission is to create a production-ready, full-stack trading journal with:

- **Frontend:** SvelteKit 2.27.1 + TypeScript + TailwindCSS
- **Backend:** Rust (Axum 0.8.4) + SurrealDB 2.3.7
- **Design:** Cyber Grid Dashboard (dark theme, neon-green accents, sharp edges)
- **Architecture:** Microservice-ready, compatible with existing PriceActionTalk ecosystem
- **Development:** Spec-Driven Development with clear separation of "what" vs "how"

---

## 🎯 PROJECT GOALS

### Primary Objectives
1. **Trading Journal Core:** Comprehensive trade entry, management, and analytics
2. **User Management:** JWT-based authentication compatible with PriceActionTalk backend
3. **Analytics & Visualization:** Advanced charts, matrices, performance diagrams
4. **Local Development:** 100% functional locally during frontend development
5. **Future Integration:** 

### Success Criteria
- ✅ Frontend compiles and runs (`npm run dev`)
- ✅ Backend compiles and runs (`cargo run`)
- ✅ User can register, login, and manage trades
- ✅ Charts and analytics display correctly
- ✅ Responsive design works on mobile/tablet/desktop
- ✅ JWT authentication works end-to-end
- ✅ All tests pass (unit, integration, e2e)

---

## 🏗️ ARCHITECTURE OVERVIEW

### System Architecture
```
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│  SvelteKit Frontend │    │   Rust Backend      │    │   SurrealDB         │
│   (Trading UI)      │◄──►│   (Axum API)        │◄──►│   (Trading Data)    │
│   + TailwindCSS     │    │   + JWT Auth        │    │   NS: trading_jrnl  │
│   Port: 5173        │    │   Port: 3000        │    │   Port: 8000        │
└─────────────────────┘    └─────────────────────┘    └─────────────────────┘
         │                           │                           │
         ▼                           ▼                           ▼
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│ Lightweight Charts  │    │   tracing Logs      │    │   File Storage      │
│   (TradingView)     │    │   + Sentry          │    │   (Screenshots)     │
└─────────────────────┘    └─────────────────────┘    └─────────────────────┘
```

### Folder Structure (Spec-Driven Development)
```
trading-journal/
├── .specify/                    # Project-wide specs
│   ├── constitution.md          # Core principles
│   ├── plan.md                  # Technical architecture
│   ├── design_system.md         # Design system
│   └── tasks_overview.md        # Roadmap
├── frontend/                    # SvelteKit app
│   ├── frontend_spec.md         # Frontend requirements
│   ├── tasks_frontend.md        # Frontend tasks
│   └── src/                     # Source code
├── backend/                     # Rust backend
│   ├── backend_spec.md          # Backend requirements
│   ├── tasks_backend.md         # Backend tasks
│   └── src/                     # Source code
├── integration/                 # CI/CD & Infrastructure
│   ├── integration_spec.md      # Integration requirements
│   └── tasks_integration.md     # Integration tasks
├── error_handling/              # Error handling domain
│   ├── error_spec.md            # Error handling requirements
│   └── tasks_error.md           # Error handling tasks
├── superdesign/                 # Design iterations
│   └── design_iterations/       # HTML design files
├── docker-compose.yml           # Local development
└── README.md                    # Project overview
```

---

## 🛠️ TECHNOLOGY STACK

### Frontend Stack (SvelteKit)
- **Framework:** SvelteKit 2.27.1
- **Language:** TypeScript 5.0
- **Build Tool:** Vite 7.0.6
- **Styling:** TailwindCSS 4.1.11
- **State Management:** Svelte Stores (writable, derived, readable)
- **Charts:**
  - Lightweight Charts by TradingView (professional trading charts)
  - Chart.js 4.x (analytics charts)
  - D3.js 7.x (custom visualizations)
- **Forms:** Svelte Native + Zod 4.0.15 Validation
- **Icons:** Lucide Svelte
- **Testing:** Vitest + Playwright
- **Error Tracking:** Sentry 10.17.0

### Backend Stack (Rust)
- **Framework:** Axum 0.8.4
- **Database:** SurrealDB 2.3.7
  - Namespace: "trading_journal"
  - Database: "journal"
- **Authentication:** JWT tokens (jsonwebtoken 9.0)
- **Password Hashing:** argon2 0.5
- **Serialization:** Serde 1.0
- **Async Runtime:** Tokio 1.47.1 (full features)
- **Validation:** validator 0.20.0
- **API Documentation:** utoipa 5.4.0 + utoipa-redoc 6.0.0
- **Logging:** tracing 0.1.41 + tracing-subscriber 0.3.19
- **Trading-Specific:**
  - rust_decimal 1.36 (precise decimal calculations)
  - chrono 0.4 (timestamp handling)
  - uuid 1.0 (v4, serde features)

### Infrastructure
- **Containerization:** Docker + Docker Compose
- **CI/CD:** GitHub Actions
- **Monitoring:** Prometheus + Grafana + Sentry
- **Documentation:** OpenAPI/Swagger via utoipa

---



### Color Palette
```css
:root {
    --bg-primary: #0d1b2a;      /* Main background */
    --bg-surface: #2e2e2e;      /* Cards, surfaces */
    --accent-primary: #1b9aaa;  /* Turquoise accent */
    --accent-neon: #7cfc00;     /* Neon-green */
    --text-primary: #ffffff;    /* Main text */
    --text-secondary: #a0a0a0;  /* Gray text */
    --success: #7cfc00;         /* Profit/success */
    --error: #ff4444;           /* Loss/error */
}
```

### Typography
- **Font Family:** `-apple-system, 'Segoe UI', Arial, sans-serif`
- **H1 (Logo):** 24px, font-weight: 700
- **H2 (Section Headers):** 14px, font-weight: 700, uppercase, letter-spacing: 1px
- **Stat Numbers:** 36px, font-weight: 700
- **Body Text:** 14px, font-weight: 400
- **Small Text:** 12px, font-weight: 400



### Effects
- **Neon Border:** `border: 2px solid #7cfc00; box-shadow: 0 0 8px #7cfc00, inset 0 0 8px rgba(124, 252, 0, 0.1);`
- **Neon Glow:** `box-shadow: 0 0 16px rgba(124, 252, 0, 0.3);`
- **Grid Background:** 32px × 32px grid with neon-green lines (5% opacity)

**Design Reference:** `superdesign/design_iterations/trading_journal_dash_1.html`

---

## 📊 DATA MODELS

### User (Shared with PriceActionTalk)
```rust
struct User {
    id: RecordId,              // SurrealDB Record ID
    name: String,              // Username (3-50 chars)
    email: String,             // Email address
    password: String,          // Argon2 Hash
    permissions: Vec<String>,  // RBAC Permissions
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

### Trade (Core Entity)
```rust
struct Trade {
    id: Uuid,                          // Unique Trade ID
    user_id: RecordId,                 // Reference to User
    symbol: String,                    // Trading Symbol (e.g., "EURUSD")
    direction: TradeDirection,         // Long | Short
    entry_price: Decimal,              // Entry Price (precise)
    exit_price: Option<Decimal>,       // Exit Price (None if open)
    quantity: Decimal,                 // Position Size
    entry_time: DateTime<Utc>,         // Entry Timestamp
    exit_time: Option<DateTime<Utc>>,  // Exit Timestamp
    pnl: Option<Decimal>,              // Profit/Loss in Currency
    pnl_percentage: Option<Decimal>,   // P&L in Percentage
    fees: Decimal,                     // Trading Fees
    notes: Option<String>,             // Trade Notes (max 5000 chars)
    tags: Vec<String>,                 // Custom Tags (max 20)
    setup_type: Option<String>,        // Setup Type (e.g., "Breakout")
    mistakes: Vec<String>,             // Identified Mistakes
    emotions: Vec<String>,             // Emotional State
    screenshots: Vec<String>,          // Screenshot URLs
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

enum TradeDirection {
    Long,
    Short,
}
```

### TradingStatistics (Aggregated Data)
```rust
struct TradingStats {
    user_id: RecordId,
    period: StatsPeriod,           // Daily | Weekly | Monthly | AllTime
    total_trades: i32,
    winning_trades: i32,
    losing_trades: i32,
    win_rate: Decimal,             // Percentage
    profit_factor: Decimal,        // Total Win / Total Loss
    total_pnl: Decimal,
    average_win: Decimal,
    average_loss: Decimal,
    largest_win: Decimal,
    largest_loss: Decimal,
    sharpe_ratio: Option<Decimal>, // Risk-adjusted return
    max_drawdown: Decimal,         // Maximum Drawdown
    average_hold_time: Duration,   // Average Trade Duration
    calculated_at: DateTime<Utc>,
}
```

---

## 🔧 TOOLS YOU MUST USE

### Information Gathering
1. **codebase-retrieval** - Search codebase for high-level information
2. **grep-search** - Search for specific text/symbols across files
3. **view** - Read files and directories
4. **git-commit-retrieval** - Find how similar changes were made in the past

### File Operations
1. **save-file** - Create new files (max 300 lines)
2. **str-replace-editor** - Edit existing files (ALWAYS use this, never recreate files)
3. **remove-files** - Delete files

### Process Management
1. **launch-process** - Run shell commands (npm, cargo, git, etc.)
2. **read-process** - Read process output
3. **write-process** - Write to process stdin
4. **kill-process** - Kill running processes

### Task Management (for complex work)
1. **add_tasks** - Create new tasks
2. **update_tasks** - Update task states
3. **view_tasklist** - View current tasks
4. **reorganize_tasklist** - Restructure task list

### Package Management
- **ALWAYS use package managers** (npm, cargo) instead of manually editing package files
- **JavaScript:** `npm install <package>` or `npm uninstall <package>`
- **Rust:** `cargo add <crate>` or `cargo remove <crate>`

---

## 📝 DEVELOPMENT WORKFLOW

### Phase 1: Project Setup (2-3 hours)
**Goal:** Establish project structure and development environment

#### Step 1.1: Create Folder Structure
```bash
mkdir -p trading-journal/{.specify,frontend,backend,integration,error_handling,superdesign/design_iterations}
cd trading-journal
git init
```

#### Step 1.2: Create Spec-Driven Development Files
Create these files in `.specify/`:
- `constitution.md` - Core principles (copy from reference)
- `plan.md` - Technical architecture (copy from reference)
- `design_system.md` - Design system (copy from reference)
- `tasks_overview.md` - Roadmap

Create spec files in each domain:
- `frontend/frontend_spec.md`
- `frontend/tasks_frontend.md`
- `backend/backend_spec.md`
- `backend/tasks_backend.md`
- `integration/integration_spec.md`
- `integration/tasks_integration.md`
- `error_handling/error_spec.md`
- `error_handling/tasks_error.md`

#### Step 1.3: Initialize Frontend (SvelteKit)
```bash
cd frontend
npm create svelte@latest . -- --template skeleton --types typescript
npm install
npm install -D tailwindcss@4.1.11 @tailwindcss/postcss autoprefixer
npm install -D @sveltejs/kit@2.27.1 vite@7.0.6
npm install zod@4.0.15 clsx tailwind-merge
npm install -D vitest @playwright/test
npm install lucide-svelte
npm install lightweight-charts chart.js d3
```

Configure TailwindCSS with Cyber Grid theme (see design_system.md)

#### Step 1.4: Initialize Backend (Rust)
```bash
cd backend
cargo init --name trading-journal-backend
cargo add axum@0.8.4 tokio@1.47.1 --features full
cargo add surrealdb@2.3.7
cargo add serde@1.0 --features derive
cargo add serde_json@1.0
cargo add jsonwebtoken@9.0
cargo add argon2@0.5
cargo add validator@0.20.0 --features derive
cargo add utoipa@5.4.0 --features axum_extras
cargo add utoipa-redoc@6.0.0 --features axum
cargo add tracing@0.1.41
cargo add tracing-subscriber@0.3.19 --features env-filter
cargo add rust_decimal@1.36
cargo add chrono@0.4
cargo add uuid@1.0 --features v4,serde
cargo add thiserror@2.0
```


```

---

### Phase 2: Backend Development (12-16 hours)

#### Step 2.1: Project Structure
```
backend/src/
├── main.rs              # Entry point
├── lib.rs               # Library exports
├── config.rs            # Configuration
├── error.rs             # Error types
├── router.rs            # API routes
├── middleware.rs        # Middleware (auth, logging)
├── openapi.rs           # OpenAPI/Swagger docs
├── validation.rs        # Input validation
├── db/
│   ├── mod.rs           # Database module
│   └── migrations.rs    # Database migrations
├── handlers/
│   ├── mod.rs           # Handler module
│   ├── auth.rs          # Auth handlers (register, login)
│   ├── trades.rs        # Trade handlers (CRUD)
│   ├── stats.rs         # Statistics handlers
│   └── health.rs        # Health check
└── models/
    ├── mod.rs           # Models module
    ├── user.rs          # User model
    ├── trade.rs         # Trade model
    └── stats.rs         # Statistics model
```

#### Step 2.2: Core Implementation Order
1. **Error Handling** (`error.rs`) - Define custom error types
2. **Configuration** (`config.rs`) - Load environment variables
3. **Database** (`db/mod.rs`) - SurrealDB connection and queries
4. **Models** (`models/`) - Data structures with Serde
5. **Validation** (`validation.rs`) - Input validation with validator
6. **Middleware** (`middleware.rs`) - JWT authentication, logging
7. **Handlers** (`handlers/`) - API endpoint logic
8. **Router** (`router.rs`) - Route definitions
9. **OpenAPI** (`openapi.rs`) - API documentation
10. **Main** (`main.rs`) - Server startup

#### Step 2.3: Key Implementation Details

**JWT Authentication:**
```rust
// middleware.rs
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // user_id
    exp: usize,   // expiration
}

async fn verify_jwt(token: &str) -> Result<Claims, Error> {
    let secret = std::env::var("JWT_SECRET")?;
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
```

**SurrealDB Queries:**
```rust
// db/mod.rs
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn connect() -> Result<Surreal<Client>, Error> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;
    db.use_ns("trading_journal").use_db("journal").await?;
    Ok(db)
}
```

**Trade CRUD:**
```rust
// handlers/trades.rs
pub async fn create_trade(
    State(db): State<Surreal<Client>>,
    Extension(user_id): Extension<String>,
    Json(payload): Json<CreateTradeRequest>,
) -> Result<Json<Trade>, Error> {
    let trade = Trade {
        id: Uuid::new_v4(),
        user_id: user_id.into(),
        symbol: payload.symbol,
        direction: payload.direction,
        entry_price: payload.entry_price,
        // ... other fields
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let created: Option<Trade> = db
        .create("trades")
        .content(trade)
        .await?;
    
    Ok(Json(created.unwrap()))
}
```

#### Step 2.4: Testing
- Unit tests for models and validation
- Integration tests for API endpoints
- Use `cargo test` to run all tests

---

### Phase 3: Frontend Development (16-20 hours)

#### Step 3.1: Project Structure
```
frontend/src/
├── routes/
│   ├── +layout.svelte       # Root layout
│   ├── +page.svelte         # Dashboard
│   ├── login/
│   │   └── +page.svelte     # Login page
│   ├── register/
│   │   └── +page.svelte     # Register page
│   ├── trades/
│   │   ├── +page.svelte     # Trade list
│   │   ├── new/
│   │   │   └── +page.svelte # New trade form
│   │   └── [id]/
│   │       └── +page.svelte # Trade detail
│   └── analytics/
│       └── +page.svelte     # Analytics dashboard
├── lib/
│   ├── components/
│   │   ├── ui/              # UI components (Button, Card, Input, etc.)
│   │   ├── charts/          # Chart components
│   │   ├── layout/          # Layout components (Header, Sidebar)
│   │   └── trades/          # Trade-specific components
│   ├── stores/
│   │   ├── auth.ts          # Auth store
│   │   ├── trades.ts        # Trades store
│   │   └── stats.ts         # Statistics store
│   ├── api/
│   │   ├── client.ts        # API client
│   │   ├── auth.ts          # Auth API
│   │   ├── trades.ts        # Trades API
│   │   └── stats.ts         # Stats API
│   ├── utils/
│   │   ├── format.ts        # Formatting utilities
│   │   ├── validation.ts    # Validation schemas (Zod)
│   │   └── constants.ts     # Constants
│   └── types/
│       ├── user.ts          # User types
│       ├── trade.ts         # Trade types
│       └── stats.ts         # Statistics types
└── app.css                  # Global styles (Cyber Grid theme)
```

#### Step 3.2: Core Implementation Order
1. **Global Styles** (`app.css`) - Cyber Grid theme
2. **Types** (`lib/types/`) - TypeScript interfaces
3. **API Client** (`lib/api/client.ts`) - Fetch wrapper with JWT
4. **Auth Store** (`lib/stores/auth.ts`) - Authentication state
5. **UI Components** (`lib/components/ui/`) - Reusable components
6. **Layout** (`routes/+layout.svelte`) - Root layout with sidebar
7. **Auth Pages** (`routes/login/`, `routes/register/`) - Login/Register
8. **Dashboard** (`routes/+page.svelte`) - Main dashboard
9. **Trade Management** (`routes/trades/`) - Trade CRUD
10. **Analytics** (`routes/analytics/`) - Charts and statistics

#### Step 3.3: Key Implementation Details

**API Client with JWT:**
```typescript
// lib/api/client.ts
import { get } from 'svelte/store';
import { authStore } from '$lib/stores/auth';

export async function apiClient<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const { token } = get(authStore);
  
  const response = await fetch(`http://localhost:3000${endpoint}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...(token && { Authorization: `Bearer ${token}` }),
      ...options.headers,
    },
  });
  
  if (!response.ok) {
    throw new Error(`API Error: ${response.statusText}`);
  }
  
  return response.json();
}
```

**Auth Store:**
```typescript
// lib/stores/auth.ts
import { writable } from 'svelte/store';
import type { User } from '$lib/types/user';

interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
}

function createAuthStore() {
  const { subscribe, set, update } = writable<AuthState>({
    user: null,
    token: null,
    isAuthenticated: false,
  });
  
  return {
    subscribe,
    login: (user: User, token: string) => {
      localStorage.setItem('token', token);
      set({ user, token, isAuthenticated: true });
    },
    logout: () => {
      localStorage.removeItem('token');
      set({ user: null, token: null, isAuthenticated: false });
    },
    init: () => {
      const token = localStorage.getItem('token');
      if (token) {
        // Verify token and load user
        // ...
      }
    },
  };
}

export const authStore = createAuthStore();
```

**Cyber Grid Theme (TailwindCSS):**
```css
/* app.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  --bg-primary: #0d1b2a;
  --bg-surface: #2e2e2e;
  --accent-primary: #1b9aaa;
  --accent-neon: #7cfc00;
  --text-primary: #ffffff;
  --text-secondary: #a0a0a0;
}

body {
  @apply bg-[var(--bg-primary)] text-[var(--text-primary)];
  font-family: -apple-system, 'Segoe UI', Arial, sans-serif;
}

.stat-card {
  @apply bg-[var(--bg-surface)] border border-[rgba(124,252,0,0.3)] relative overflow-hidden;
}

.stat-card::before {
  content: '';
  @apply absolute top-0 left-0 w-1 h-full bg-[var(--accent-neon)];
}

.neon-border {
  @apply border-2 border-[var(--accent-neon)];
  box-shadow: 0 0 8px var(--accent-neon), inset 0 0 8px rgba(124, 252, 0, 0.1);
}

.grid-bg {
  background-image: 
    linear-gradient(rgba(124, 252, 0, 0.05) 1px, transparent 1px),
    linear-gradient(90deg, rgba(124, 252, 0, 0.05) 1px, transparent 1px);
  background-size: 32px 32px;
}
```

**Lightweight Charts Integration:**
```typescript
// lib/components/charts/TradingChart.svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { createChart } from 'lightweight-charts';
  
  export let data: any[];
  
  let chartContainer: HTMLDivElement;
  
  onMount(() => {
    const chart = createChart(chartContainer, {
      width: chartContainer.clientWidth,
      height: 400,
      layout: {
        background: { color: '#2e2e2e' },
        textColor: '#ffffff',
      },
      grid: {
        vertLines: { color: 'rgba(124, 252, 0, 0.1)' },
        horzLines: { color: 'rgba(124, 252, 0, 0.1)' },
      },
    });
    
    const candlestickSeries = chart.addCandlestickSeries({
      upColor: '#7cfc00',
      downColor: '#ff4444',
      borderVisible: false,
      wickUpColor: '#7cfc00',
      wickDownColor: '#ff4444',
    });
    
    candlestickSeries.setData(data);
    
    return () => chart.remove();
  });
</script>

<div bind:this={chartContainer} class="w-full"></div>
```

#### Step 3.4: Testing
- Unit tests with Vitest for utilities and stores
- Component tests with Vitest + Testing Library
- E2E tests with Playwright for critical user flows

---

### Phase 4: Integration & Testing (6-8 hours)

#### Step 4.1: Docker Compose Integration
- Ensure all services start correctly
- Test frontend-backend communication
- Verify database connections

#### Step 4.2: End-to-End Testing
- User registration flow
- Login flow
- Create/edit/delete trade
- View analytics
- Logout flow

#### Step 4.3: Error Handling
- Network errors
- Authentication errors
- Validation errors
- Database errors

#### Step 4.4: Performance Optimization
- Code splitting
- Lazy loading
- Image optimization
- Database query optimization

---

### Phase 5: Documentation & Deployment (4-6 hours)

#### Step 5.1: Documentation
- README.md with setup instructions
- API documentation (OpenAPI/Swagger)
- Architecture diagrams
- Deployment guide

#### Step 5.2: CI/CD Setup
- GitHub Actions for frontend (lint, test, build)
- GitHub Actions for backend (clippy, test, build)
- Automated deployment to staging

#### Step 5.3: Monitoring
- Sentry integration for error tracking
- Prometheus + Grafana for metrics
- Logging with tracing

---

## ⚠️ CRITICAL RULES

### 1. Spec-Driven Development
- **ALWAYS** create specification files BEFORE implementation
- Separate "what" (specs) from "how" (tasks)
- Review specs with user before implementation

### 2. Tool Usage
- **ALWAYS** use `str-replace-editor` for editing files (NEVER recreate files)
- **ALWAYS** use package managers (npm, cargo) for dependencies
- **ALWAYS** use parallel tool calls when possible

### 3. Code Quality
- Follow Rust best practices (clippy, rustfmt)
- Follow TypeScript best practices (ESLint, Prettier)
- Write tests for all business logic
- Document complex logic

### 4. Design System
- **ALWAYS** follow Cyber Grid design (sharp edges, neon-green accents)
- **NEVER** use rounded corners (border-radius: 0)
- **ALWAYS** use CSS custom properties from design_system.md

### 5. Error Handling
- Implement comprehensive error handling
- User-friendly error messages
- Detailed logging for debugging
- Graceful degradation

### 6. Security
- Validate all inputs
- Use parameterized queries
- Implement CSRF protection
- Secure JWT tokens

---

## 📚 REFERENCE DOCUMENTS

### Must Read Before Starting
1. `.specify/constitution.md` - Core principles
2. `.specify/plan.md` - Technical architecture
3. `.specify/design_system.md` - Design system
4. `superdesign/design_iterations/trading_journal_dash_1.html` - Design reference

### Tech Stack Reference
- `ONLYUSEFORTECHSTACKINFO/backend-main/` - Rust backend reference
- `ONLYUSEFORTECHSTACKINFO/website-main/` - SvelteKit frontend reference

---

## 🎯 STEP-BY-STEP EXECUTION PLAN

### Day 1: Setup (6-8 hours)
1. Create folder structure
2. Initialize Git repository
3. Create all spec files
4. Initialize SvelteKit frontend
5. Initialize Rust backend
6. Create Docker Compose setup
7. Verify all services start

### Day 2-3: Backend (12-16 hours)
1. Implement error handling
2. Implement configuration
3. Implement database layer
4. Implement models
5. Implement validation
6. Implement middleware (JWT auth)
7. Implement handlers (auth, trades, stats)
8. Implement router
9. Implement OpenAPI docs
10. Write tests

### Day 4-5: Frontend (16-20 hours)
1. Setup Cyber Grid theme
2. Create type definitions
3. Implement API client
4. Implement auth store
5. Create UI components
6. Implement layout (header, sidebar)
7. Implement auth pages (login, register)
8. Implement dashboard
9. Implement trade management
10. Implement analytics
11. Integrate Lightweight Charts
12. Write tests

### Day 6: Integration (6-8 hours)
1. Test frontend-backend integration
2. Implement error handling
3. Optimize performance
4. Fix bugs

### Day 7: Documentation & Deployment (4-6 hours)
1. Write documentation
2. Setup CI/CD
3. Setup monitoring
4. Deploy to staging

---

## ✅ SUCCESS CHECKLIST

### Backend
- [ ] Backend compiles with 0 errors (`cargo build`)
- [ ] Backend starts successfully (`cargo run`)
- [ ] All tests pass (`cargo test`)
- [ ] OpenAPI docs accessible at `/api/docs`
- [ ] JWT authentication works
- [ ] All CRUD endpoints functional

### Frontend
- [ ] Frontend compiles with 0 errors (`npm run build`)
- [ ] Frontend starts successfully (`npm run dev`)
- [ ] All tests pass (`npm test`)
- [ ] Cyber Grid design implemented correctly
- [ ] Responsive design works on mobile/tablet/desktop
- [ ] Charts display correctly

### Integration
- [ ] User can register
- [ ] User can login
- [ ] User can create trades
- [ ] User can view trades
- [ ] User can edit trades
- [ ] User can delete trades
- [ ] User can view analytics
- [ ] User can logout

### Quality
- [ ] No console errors
- [ ] No TypeScript errors
- [ ] No Rust warnings
- [ ] All tests pass
- [ ] Code follows style guides
- [ ] Documentation complete

---

## 🚨 COMMON PITFALLS TO AVOID

1. **Don't recreate files** - Always use `str-replace-editor`
2. **Don't manually edit package files** - Use package managers
3. **Don't skip specs** - Always create specs before implementation
4. **Don't ignore design system** - Follow Cyber Grid design strictly
5. **Don't skip tests** - Write tests as you go
6. **Don't hardcode values** - Use environment variables
7. **Don't ignore errors** - Implement proper error handling
8. **Don't skip documentation** - Document as you go

---

## 🎓 LEARNING RESOURCES

### SvelteKit
- https://kit.svelte.dev/docs
- https://svelte.dev/tutorial

### Rust + Axum
- https://docs.rs/axum/latest/axum/
- https://tokio.rs/tokio/tutorial

### SurrealDB
- https://surrealdb.com/docs

### Lightweight Charts
- https://tradingview.github.io/lightweight-charts/

---

## 📞 WHEN TO ASK FOR HELP

Ask the user for clarification when:
1. Specifications are unclear or contradictory
2. Design decisions need to be made
3. You encounter blocking technical issues
4. You need access to external resources
5. You're unsure about security implications
6. You need to make breaking changes

---

## 🎉 FINAL NOTES

This is a **complete rebuild** - start fresh with a clean slate. Follow the Spec-Driven Development methodology strictly. Create specifications before implementation. Use the provided design system. Write tests. Document your work. Ask for help when needed.

**Good luck! You've got this! 🚀**

---

**Total Estimated Time:** 40-60 hours  
**Complexity:** High  
**Priority:** Critical  
**Status:** Ready to Start

