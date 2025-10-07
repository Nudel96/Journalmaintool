# 🚀 TRADING JOURNAL - CORRECTED REBUILD SPECIFICATION

**Version:** 2.1 (KORRIGIERT)  
**Datum:** 2025-10-07  
**Basis:** AGENT_VIBECODE_COMPLETE_REBUILD.md  
**Status:** BEREIT FÜR IMPLEMENTIERUNG

---

## ⚠️ KORREKTUREN ZUR ORIGINAL rebuild.md

Diese Datei korrigiert kritische Fehler aus `AGENT_VIBECODE_COMPLETE_REBUILD.md`:

### Hauptänderungen:
1. ✅ **Datenbank:** PostgreSQL statt SurrealDB (Render-Kompatibilität)
2. ✅ **Package-Versionen:** Aktuelle, existierende Versionen
3. ✅ **Stripe-Tiers:** Neue Preisstruktur (1/6/12 Monate, kein Lifetime)
4. ✅ **Three.js:** Explizit im Tech-Stack aufgenommen
5. ✅ **Design:** HTML-Farbpalette als Standard definiert

---

## 🛠️ TECHNOLOGIE-STACK (KORRIGIERT)

### Frontend Stack
- **Framework:** SvelteKit 2.x (latest stable)
- **Language:** TypeScript 5.x
- **Build Tool:** Vite 5.4.x ⚠️ KORRIGIERT (war: 7.0.6)
- **Styling:** TailwindCSS 3.4.x ⚠️ KORRIGIERT (war: 4.1.11)
- **State Management:** Svelte Stores
- **3D Graphics:** ⚠️ NEU HINZUGEFÜGT
  - Three.js 0.160.x
  - @threlte/core 7.x (Svelte-Integration)
  - @threlte/extras 8.x
- **Charts:**
  - Lightweight Charts by TradingView
  - Chart.js 4.x
  - D3.js 7.x (optional)
- **Forms:** Svelte Native + Zod 3.23.x Validation ⚠️ KORRIGIERT (war: 4.0.15)
- **Icons:** Lucide Svelte
- **Testing:** Vitest + Playwright
- **Error Tracking:** @sentry/svelte

### Backend Stack
- **Framework:** Axum 0.7.x (latest stable)
- **Database:** PostgreSQL 15+ ⚠️ KORRIGIERT (war: SurrealDB 2.3.7)
  - **Deployment:** Render Managed PostgreSQL
  - **Local:** PostgreSQL in Docker
- **ORM:** sqlx 0.7.x ⚠️ NEU (für PostgreSQL)
- **Authentication:** JWT tokens (jsonwebtoken 9.x)
- **Password Hashing:** argon2 0.5.x
- **Serialization:** Serde 1.0.x
- **Async Runtime:** Tokio 1.x (full features)
- **Validation:** validator 0.18.x
- **API Documentation:** utoipa 4.x + utoipa-redoc 4.x
- **Logging:** tracing 0.1.x + tracing-subscriber 0.3.x
- **Trading-Specific:**
  - rust_decimal 1.x (precise decimal calculations)
  - chrono 0.4.x (timestamp handling)
  - uuid 1.x (v4, serde features)

### Infrastructure
- **Frontend Deployment:** Vercel
- **Backend Deployment:** Render
- **Database:** Render Managed PostgreSQL
- **Payments:** Stripe
- **Monitoring:** Sentry
- **CI/CD:** GitHub Actions
- **Containerization:** Docker + Docker Compose (local dev)

---

## 💳 STRIPE SUBSCRIPTION-TIERS (KORRIGIERT)

⚠️ **NEUE PREISSTRUKTUR** (Lifetime-Plan entfernt):

| Tier | Preis | Intervall | Jährliche Kosten | Ersparnis |
|------|-------|-----------|------------------|-----------|
| **1 Monat** | $7/Monat | Monatlich | $84/Jahr | - |
| **6 Monate** | $5/Monat | Alle 6 Monate | $60/Jahr | 29% |
| **12 Monate** | $4/Monat | Jährlich | $48/Jahr | 43% |

### Features (alle Tiers gleich):
- ✅ Unlimited Trades
- ✅ Advanced Analytics
- ✅ Export-Funktionen
- ✅ Screenshot-Upload
- ✅ Custom Tags & Notes
- ✅ Performance-Charts

### Stripe-Implementierung:
```javascript
// 3 separate Stripe Products/Prices erstellen:
const prices = [
  { name: "1 Monat", amount: 700, interval: "month" },
  { name: "6 Monate", amount: 3000, interval: "month", interval_count: 6 },
  { name: "12 Monate", amount: 4800, interval: "year" }
];
```

---

## 🎨 DESIGN-SYSTEM (KORRIGIERT)

⚠️ **WICHTIG:** Verwende die Farbpalette aus dem HTML, NICHT aus der Original rebuild.md!

### Farbpalette (aus HTML)
```css
:root {
  --bg: #0d1b2a;               /* Deep Navy - Main Background */
  --bg-2: #2e2e2e;             /* Dark Gray - Cards/Surfaces */
  --brand: #1b9aaa;            /* Turquoise - Brand Color */
  --accent: #7cfc00;           /* Lime Green - Accent/Success */
  --success: #7cfc00;          /* Success States */
  --danger: #ef4444;           /* Error States ⚠️ KORRIGIERT (war: #ff4444) */
  --text: #ffffff;             /* White - Primary Text */
  --muted: #cfd6dd;            /* Light Muted - Secondary Text ⚠️ KORRIGIERT (war: #a0a0a0) */
  
  /* Zusätzliche Variablen aus HTML: */
  --ring: rgba(27, 154, 170, .45);
  --glass: rgba(46, 46, 46, 0.55);
  --glass-strong: rgba(46, 46, 46, 0.85);
  --card-border: rgba(27, 154, 170, .22);
  --shadow: 0 10px 40px rgba(0,0,0,.55);
}
```

### Typografie
- **Font Family:** Inter (Google Fonts)
- **H1 (Logo):** 24px, font-weight: 700
- **H2 (Section Headers):** 14px, font-weight: 700, uppercase
- **Body Text:** 14px, font-weight: 400
- **Small Text:** 12px, font-weight: 400

### Border Radius ⚠️ KORRIGIERT
**WICHTIG:** Rounded corners sind ERLAUBT (nicht "sharp edges only"):
- **Cards:** 24px
- **Buttons/Tabs:** 12px
- **Inputs:** 14px
- **Badges:** 999px (pill shape)

---

## 📊 DATENMODELLE (KORRIGIERT FÜR POSTGRESQL)

### User Model (PostgreSQL)
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    password_hash VARCHAR(255) NOT NULL,
    
    -- Stripe Integration ⚠️ NEU HINZUGEFÜGT
    stripe_customer_id VARCHAR(255) UNIQUE,
    subscription_status VARCHAR(50) DEFAULT 'none',
    subscription_tier VARCHAR(50) DEFAULT 'none',
    subscription_interval VARCHAR(20), -- 'month', 'month_6', 'year'
    
    permissions TEXT[] DEFAULT '{}',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_stripe_customer ON users(stripe_customer_id);
```

**Subscription Status:** `active`, `canceled`, `past_due`, `trialing`, `none`  
**Subscription Tier:** `none`, `paid`  
**Subscription Interval:** `month`, `month_6`, `year`

---

### Trade Model (PostgreSQL)
```sql
CREATE TABLE trades (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Trade Details
    symbol VARCHAR(20) NOT NULL,
    direction VARCHAR(10) NOT NULL CHECK (direction IN ('long', 'short')),
    entry_price DECIMAL(20, 8) NOT NULL,
    exit_price DECIMAL(20, 8),
    quantity DECIMAL(20, 8) NOT NULL,
    
    -- Timestamps
    entry_time TIMESTAMP NOT NULL,
    exit_time TIMESTAMP,
    
    -- P&L
    pnl DECIMAL(20, 8),
    pnl_percentage DECIMAL(10, 4),
    fees DECIMAL(20, 8) DEFAULT 0,
    
    -- Metadata
    notes TEXT,
    tags TEXT[] DEFAULT '{}',
    setup_type VARCHAR(100),
    mistakes TEXT[] DEFAULT '{}',
    emotions TEXT[] DEFAULT '{}',
    screenshots TEXT[] DEFAULT '{}',
    
    -- Additional Fields ⚠️ NEU HINZUGEFÜGT
    broker VARCHAR(100),
    account_id VARCHAR(100),
    status VARCHAR(20) DEFAULT 'open' CHECK (status IN ('open', 'closed', 'pending')),
    
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_trades_user_id ON trades(user_id);
CREATE INDEX idx_trades_entry_time ON trades(entry_time DESC);
CREATE INDEX idx_trades_symbol ON trades(symbol);
CREATE INDEX idx_trades_status ON trades(status);
```

---

## 🎭 THREE.JS 3D-ANIMATIONEN (NEU HINZUGEFÜGT)

⚠️ **WICHTIG:** Diese Komponenten waren in der Original rebuild.md NICHT erwähnt!

### Komponenten (aus Login-HTML)
1. **Glow Gradient Background Sphere**
   - Custom ShaderMaterial
   - SphereGeometry(60, 64, 64)
   - Zeitbasierte Noise-Animation

2. **Particle Field**
   - 4500 Partikel (Desktop)
   - Custom ShaderMaterial
   - Sinusförmige Bewegung
   - Additive Blending

3. **Wireframe Waves Plane**
   - PlaneGeometry(160, 100, 120, 80)
   - Animierte Z-Displacement
   - LineSegments

4. **Mouse Parallax**
   - Interaktive Kamera-Bewegung
   - Smooth Lerp (0.03 factor)

### Svelte-Integration
```bash
npm install three @threlte/core @threlte/extras
```

**Komponenten-Struktur:**
```
src/lib/components/3d/
├── BackgroundScene.svelte       # Haupt-Canvas
├── GlowSphere.svelte            # Shader-Sphere
├── ParticleField.svelte         # Partikel-System
├── WireframePlane.svelte        # Wireframe-Ebene
├── CameraController.svelte      # Mouse Parallax
└── shaders/
    ├── glowSphere.vert.glsl
    ├── glowSphere.frag.glsl
    ├── particles.vert.glsl
    └── particles.frag.glsl
```

### Performance-Optimierungen
```typescript
// Responsive Partikel-Anzahl
const particleCount = {
  desktop: 4500,
  tablet: 2000,
  mobile: 500 // oder deaktiviert
};
```

**Entwicklungszeit:** ~22 Stunden

---

## 🔧 BACKEND-SETUP (KORRIGIERT)

### Dependencies (Cargo.toml)
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "migrate", "uuid", "chrono"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
jsonwebtoken = "9"
argon2 = "0.5"
validator = { version = "0.18", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
rust_decimal = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
thiserror = "1"
dotenv = "0.15"
tower-http = { version = "0.5", features = ["cors"] }
```

### Database Connection (sqlx)
```rust
use sqlx::postgres::PgPoolOptions;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
```

---

## 📁 PROJEKT-STRUKTUR (UNVERÄNDERT)

```
trading-journal/
├── frontend/                    # SvelteKit App
│   ├── src/
│   │   ├── routes/
│   │   ├── lib/
│   │   │   ├── components/
│   │   │   │   ├── 3d/          # ⚠️ NEU: Three.js Komponenten
│   │   │   │   ├── ui/
│   │   │   │   ├── charts/
│   │   │   │   └── trades/
│   │   │   ├── stores/
│   │   │   ├── api/
│   │   │   └── types/
│   │   └── app.css
│   └── package.json
├── backend/                     # Rust Backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── handlers/
│   │   ├── models/
│   │   └── db/
│   ├── migrations/              # ⚠️ PostgreSQL Migrations
│   └── Cargo.toml
├── docker-compose.yml
├── KNOWLEDGE.md
├── PROJEKTPLAN.md
├── CORRECTED_REBUILD.md         # Diese Datei
└── README.md
```

---

## ✅ ZUSAMMENFASSUNG DER KORREKTUREN

| Original rebuild.md | CORRECTED_REBUILD.md | Grund |
|---------------------|----------------------|-------|
| SurrealDB 2.3.7 | PostgreSQL 15+ | Render-Kompatibilität |
| Vite 7.0.6 | Vite 5.4.x | Version existiert nicht |
| TailwindCSS 4.1.11 | TailwindCSS 3.4.x | Version existiert nicht |
| Zod 4.0.15 | Zod 3.23.x | Version existiert nicht |
| Lifetime-Plan $299 | 1/6/12 Monate $7/$5/$4 | Neue Preisstruktur |
| Keine Three.js Erwähnung | Three.js + @threlte/core | HTML verwendet 3D |
| Sharp edges only | Rounded corners erlaubt | HTML-Design |
| --error: #ff4444 | --danger: #ef4444 | HTML-Farbpalette |

---

**Diese korrigierte Spezifikation ist die Grundlage für die Implementierung.**  
**Verwende diese Datei statt der Original rebuild.md!**

