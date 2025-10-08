# KNOWLEDGE BASE - Trading Journal Tool

**Projekt:** PriceActionTalk Trading Journal
**Repository:** https://github.com/Nudel96/Journalmaintool
**Letzte Aktualisierung:** 2025-10-08
**Status:** 🚀 IN ENTWICKLUNG - Backend & Frontend funktionsfähig

---

## 📋 PROJEKT-ÜBERSICHT

### Ziel
Entwicklung eines professionellen Trading Journal Tools mit:
- Benutzer-Authentifizierung
- Stripe-Subscription-Modell
- Trading-Daten-Management
- Analytics & Visualisierung
- 3D-animierter Login-Screen

### Technologie-Stack (EMPFOHLEN)
**Frontend:**
- SvelteKit (latest stable)
- TailwindCSS 3.4.x
- Three.js + @threlte/core (für 3D-Animationen)
- Lightweight Charts, Chart.js
- Zod 3.x (Validation)

**Backend:**
- Rust + Actix-Web 4.x (✅ IMPLEMENTIERT)
- PostgreSQL 15+ (Docker lokal, Render für Production)
- sqlx 0.7.x (ORM) ✅ IMPLEMENTIERT
- JWT Authentication (✅ IMPLEMENTIERT)

**Services:**
- Stripe (Zahlungen)
- Vercel (Frontend Deployment)
- Render (Backend Deployment)
- Sentry (Error Tracking)

---

## 🚨 KRITISCHE BEFUNDE

### 1. rebuild.md stammt aus anderem Projekt
**Problem:** Die Datei `AGENT_VIBECODE_COMPLETE_REBUILD.md` stammt aus einem anderen Projekt (PriceActionTalk) und enthält:
- ❌ Nicht-existierende Package-Versionen (Vite 7.0.6, TailwindCSS 4.1.11, Zod 4.0.15)
- ❌ Widersprüchliche Authentifizierungs-Anforderungen
- ❌ Fehlende Three.js Spezifikation (obwohl im HTML verwendet)
- ❌ Unklare "Microservice-ready" Architektur (ist eigentlich Monolith)

**Lösung:** rebuild.md als Referenz verwenden, aber kritisch hinterfragen

---

### 2. Datenbank-Inkompatibilität
**Problem:** 
- rebuild.md fordert: SurrealDB 2.3.7
- Render bietet: PostgreSQL, Redis, Key-Value (NICHT SurrealDB)

**Entscheidung:** PostgreSQL verwenden
- ✅ Native Render Support
- ✅ Managed Service (Backups, Scaling)
- ✅ Produktionsreif
- ✅ Große Community

**Auswirkung:** Datenmodell muss für SQL angepasst werden

---

### 3. Three.js NICHT in rebuild.md erwähnt
**Problem:** Login-HTML verwendet komplexe Three.js 3D-Animationen, aber rebuild.md erwähnt dies NICHT

**Komponenten:**
1. Glow Gradient Background Sphere (Custom Shaders)
2. Particle Field mit 4500 Partikeln (Custom Shaders)
3. Wireframe Waves Plane (Animierte Geometrie)
4. Mouse Parallax (Interaktive Kamera)

**Lösung:** @threlte/core für Svelte-Integration verwenden

**Aufwand:** ~22 Stunden zusätzlich

---

### 4. Design-Konflikte
**Problem:** rebuild.md vs. HTML haben unterschiedliche Design-Vorgaben

**Anweisung:** "Nutze die Farbpaletten aus den html, ignoriere designvorgaben aus rebuild.md"

**Lösung:**
- ✅ HTML-Farbpalette verwenden
- ✅ Rounded corners erlaubt (nicht "sharp edges only")
- ✅ Alle CSS-Variablen aus HTML übernehmen

---

## 📊 DATENMODELLE (ERWEITERT)

### User Model (PostgreSQL) ✅ IMPLEMENTIERT
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    password_hash VARCHAR(255) NOT NULL,
    stripe_customer_id VARCHAR(255),
    subscription_status VARCHAR(50) DEFAULT 'none',
    subscription_tier VARCHAR(50) DEFAULT 'none',
    subscription_interval VARCHAR(20),
    permissions TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),  -- ⚠️ KORRIGIERT: TIMESTAMPTZ statt TIMESTAMP
    updated_at TIMESTAMPTZ DEFAULT NOW(),  -- ⚠️ KORRIGIERT: TIMESTAMPTZ statt TIMESTAMP
    last_login TIMESTAMPTZ                 -- ⚠️ KORRIGIERT: TIMESTAMPTZ statt TIMESTAMP
);
```

**Subscription Status:** active, canceled, past_due, trialing, none  
**Subscription Tier:** free, pro, lifetime

---

### Trade Model (PostgreSQL) ✅ IMPLEMENTIERT
```sql
CREATE TABLE trades (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    symbol VARCHAR(20) NOT NULL,
    direction VARCHAR(10) NOT NULL CHECK (direction IN ('long', 'short')),
    entry_price DECIMAL(20, 8) NOT NULL,
    exit_price DECIMAL(20, 8),
    quantity DECIMAL(20, 8) NOT NULL,
    entry_time TIMESTAMPTZ NOT NULL,    -- ⚠️ KORRIGIERT: TIMESTAMPTZ statt TIMESTAMP
    exit_time TIMESTAMPTZ,              -- ⚠️ KORRIGIERT: TIMESTAMPTZ statt TIMESTAMP
    pnl DECIMAL(20, 8),
    pnl_percentage DECIMAL(10, 4),
    fees DECIMAL(20, 8) DEFAULT 0,
    notes TEXT,
    tags TEXT[],
    setup_type VARCHAR(100),
    mistakes TEXT[],
    emotions TEXT[],
    screenshots TEXT[],
    broker VARCHAR(100),
    account_id VARCHAR(100),
    status VARCHAR(20) DEFAULT 'open' CHECK (status IN ('open', 'closed', 'pending')),
    created_at TIMESTAMPTZ DEFAULT NOW(),  -- ⚠️ KORRIGIERT: TIMESTAMPTZ statt TIMESTAMP
    updated_at TIMESTAMPTZ DEFAULT NOW()   -- ⚠️ KORRIGIERT: TIMESTAMPTZ statt TIMESTAMP
);

CREATE INDEX idx_trades_user_id ON trades(user_id);
CREATE INDEX idx_trades_entry_time ON trades(entry_time DESC);
CREATE INDEX idx_trades_symbol ON trades(symbol);
CREATE INDEX idx_trades_status ON trades(status);
```

---

## 🎨 DESIGN-SYSTEM

### Farbpalette (aus HTML)
```css
:root {
  --bg: #0d1b2a;               /* deep navy */
  --bg-2: #2e2e2e;             /* dark gray */
  --brand: #1b9aaa;            /* turquoise */
  --accent: #7cfc00;           /* lime green */
  --success: #7cfc00;          /* success */
  --danger: #ef4444;           /* error */
  --text: #ffffff;             /* white text */
  --muted: #cfd6dd;            /* light muted */
  --ring: rgba(27, 154, 170, .45);
  --glass: rgba(46, 46, 46, 0.55);
  --glass-strong: rgba(46, 46, 46, 0.85);
  --card-border: rgba(27, 154, 170, .22);
  --shadow: 0 10px 40px rgba(0,0,0,.55);
}
```

### Typografie
- **Font:** Inter (Google Fonts)
- **Sizes:** 12px (small), 14px (body), 16px (large), 24px (headings)

### Border Radius
- Cards: 24px
- Buttons/Tabs: 12px
- Inputs: 14px
- Badges: 999px (pill)

---

## 🔧 STRIPE-INTEGRATION

### Subscription-Tiers (✅ BESTÄTIGT)
| Tier | Preis | Intervall | Jährliche Kosten | Ersparnis |
|------|-------|-----------|------------------|-----------|
| **1 Monat** | $7/Monat | Monatlich | $84/Jahr | - |
| **6 Monate** | $5/Monat | Alle 6 Monate | $60/Jahr | 29% |
| **12 Monate** | $4/Monat | Jährlich | $48/Jahr | 43% |

**Features (alle Tiers gleich):**
- ✅ Unlimited Trades
- ✅ Advanced Analytics
- ✅ Export-Funktionen
- ✅ Screenshot-Upload
- ✅ Custom Tags & Notes
- ✅ Performance-Charts

### Webhooks (zu implementieren)
- `checkout.session.completed` - Neue Subscription
- `customer.subscription.updated` - Subscription geändert
- `customer.subscription.deleted` - Subscription gekündigt
- `invoice.payment_succeeded` - Zahlung erfolgreich
- `invoice.payment_failed` - Zahlung fehlgeschlagen

### Stripe-Implementierung
**3 separate Stripe Products/Prices erstellen:**
```javascript
const prices = [
  { name: "1 Monat", amount: 700, interval: "month" },
  { name: "6 Monate", amount: 3000, interval: "month", interval_count: 6 },
  { name: "12 Monate", amount: 4800, interval: "year" }
];
```

**User-Model Felder:**
- `subscription_status`: active, canceled, past_due, trialing, none
- `subscription_tier`: none, paid
- `subscription_interval`: month, month_6, year

---

## 📁 PROJEKT-STRUKTUR

```
trading-journal/
├── frontend/                    # SvelteKit App
│   ├── src/
│   │   ├── routes/
│   │   │   ├── +layout.svelte
│   │   │   ├── +page.svelte     # Dashboard
│   │   │   ├── login/
│   │   │   ├── register/
│   │   │   ├── trades/
│   │   │   └── analytics/
│   │   ├── lib/
│   │   │   ├── components/
│   │   │   │   ├── 3d/          # Three.js Komponenten
│   │   │   │   ├── ui/          # UI Komponenten
│   │   │   │   ├── charts/      # Chart Komponenten
│   │   │   │   └── trades/      # Trade Komponenten
│   │   │   ├── stores/          # Svelte Stores
│   │   │   ├── api/             # API Client
│   │   │   ├── utils/
│   │   │   └── types/
│   │   └── app.css
│   ├── package.json
│   └── vite.config.ts
├── backend/                     # Rust Backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── error.rs
│   │   ├── router.rs
│   │   ├── middleware.rs
│   │   ├── db/
│   │   ├── handlers/
│   │   │   ├── auth.rs
│   │   │   ├── trades.rs
│   │   │   ├── stats.rs
│   │   │   └── stripe_webhooks.rs
│   │   └── models/
│   ├── Cargo.toml
│   └── migrations/
├── docker-compose.yml           # Lokale Entwicklung
├── KNOWLEDGE.md                 # Diese Datei
├── ANALYSIS_REBUILD_MD.md       # Detaillierte rebuild.md Analyse
├── TOOL_COMPATIBILITY_ANALYSIS.md
├── THREE_JS_INTEGRATION_ANALYSIS.md
├── INKOMPATIBILITAETEN.md
└── README.md
```

---

## ⚠️ BEKANNTE PROBLEME

### 1. Package-Versionen in rebuild.md
**Problem:** Nicht-existierende Versionen spezifiziert  
**Lösung:** Aktuelle stabile Versionen verwenden

### 2. Lokale Stripe Webhooks
**Problem:** Webhooks funktionieren nicht lokal ohne Tools  
**Lösung:** Stripe CLI verwenden (`stripe listen --forward-to localhost:3000/webhooks/stripe`)

### 3. Three.js Performance auf Mobile
**Problem:** 4500 Partikel + Shaders sind GPU-intensiv  
**Lösung:** Responsive Partikel-Anzahl (Desktop: 4500, Tablet: 2000, Mobile: 500 oder deaktiviert)

### 4. Bundle-Size mit Three.js
**Problem:** Three.js ist ~600KB (minified)  
**Lösung:** Code-Splitting, Lazy Loading

---

## ✅ ENTSCHEIDUNGEN GETROFFEN & BESTÄTIGT

### Bestätigt am 2025-10-07:

1. ✅ **Datenbank:** PostgreSQL (Render managed)
   - Verwende sqlx als ORM
   - PostgreSQL 15+ in Docker für lokale Entwicklung
   - Migrations mit sqlx-cli

2. ✅ **Package-Versionen:** Aktuelle stabile Versionen
   - Vite 5.4.x (statt nicht-existierender 7.0.6)
   - TailwindCSS 3.4.x (statt nicht-existierender 4.1.11)
   - Zod 3.23.x (statt nicht-existierender 4.0.15)

3. ✅ **Design:** HTML-Farbpalette und -Design verwenden
   - Rounded corners ERLAUBT (nicht "sharp edges only")
   - Alle CSS-Variablen aus HTML übernehmen
   - --danger: #ef4444 (nicht #ff4444)

4. ✅ **Three.js:** Mit @threlte/core implementieren
   - Alle 4 Komponenten portieren (Glow Sphere, Particles, Wireframe, Parallax)
   - Performance-Optimierungen für Mobile
   - ~22 Stunden Entwicklungszeit

5. ✅ **Authentifizierung:** Eigenständiges System
   - JWT-basiert
   - Keine PriceActionTalk-Integration

6. ✅ **Stripe Subscription-Tiers:** NEUE PREISSTRUKTUR
   - **1 Monat:** $7/Monat (monatlich kündbar)
   - **6 Monate:** $5/Monat ($30 alle 6 Monate) - Spare 29%
   - **12 Monate:** $4/Monat ($48 jährlich) - Spare 43%
   - ❌ KEIN Lifetime-Plan mehr

---

## 🎯 NÄCHSTE SCHRITTE

### Phase 1-3: ABGESCHLOSSEN ✅
- ✅ Repository-Verbindung
- ✅ Anforderungsanalyse
- ✅ Tool-Kompatibilität
- ✅ Three.js Integration identifiziert
- ✅ Inkompatibilitäten dokumentiert
- ✅ Projektplanung
- ✅ KNOWLEDGE.md erstellt
- ✅ CORRECTED_REBUILD.md erstellt

### Phase 4: Technologie-Stack Setup ✅ ABGESCHLOSSEN
- [x] CORRECTED_REBUILD.md erstellen
- [x] KNOWLEDGE.md aktualisieren
- [x] Folder-Struktur erstellen
- [x] SvelteKit initialisieren
- [x] Rust Backend initialisieren (Actix-Web)
- [x] Docker Compose Setup
- [x] PostgreSQL Migrations (mit TIMESTAMPTZ-Fix)

### Phase 5: Authentifizierung ✅ ABGESCHLOSSEN
- [x] Backend Auth-Handlers (Register, Login)
- [x] JWT-Token-Generation
- [x] Password-Hashing (Argon2)
- [x] Frontend Auth-Store
- [x] Login/Register Pages

### Phase 6-7: IN PROGRESS
- [/] Trading-Features (Dashboard implementiert)
- [ ] Stripe Integration (M5)
- [ ] 3D-Animationen (M7)
- [ ] Analytics & Charts (M8)
- [ ] Testing & Deployment (M9-M10)

---

## 📞 OFFENE FRAGEN

~~1. **Stripe-Tiers:** Welche Subscription-Pläne? (Preise, Features?)~~ ✅ BEANTWORTET
~~2. **TailwindCSS:** v3.4 (stable) oder v4.0-alpha?~~ ✅ BEANTWORTET (3.4.x)
~~3. **Datenbank:** PostgreSQL oder SurrealDB?~~ ✅ BEANTWORTET (PostgreSQL)
~~4. **Three.js:** Implementieren oder weglassen?~~ ✅ BEANTWORTET (Implementieren)

**Alle kritischen Entscheidungen sind getroffen! Bereit für Implementierung.**

---

## 📚 REFERENZEN

- **CORRECTED_REBUILD.md:** `D:\Testen\CORRECTED_REBUILD.md` ⚠️ VERWENDE DIESE STATT rebuild.md!
- **rebuild.md (Original):** `D:\Testen\Instructions\AGENT_VIBECODE_COMPLETE_REBUILD.md` (NUR als Referenz, enthält Fehler!)
- **Login HTML:** `D:\Testen\Instructions\price_action_talk_3_d_login_register_one_pager_single_html (1).html`
- **Dashboard HTML:** `D:\Testen\Instructions\trading_dashboard_canvas_1.html`
- **Repository:** https://github.com/Nudel96/Journalmaintool

---

**Für zukünftige Agents:**
- ⚠️ **WICHTIG:** Verwende `CORRECTED_REBUILD.md`, NICHT die Original rebuild.md!
- Lese ZUERST diese Knowledge.md
- Prüfe ANALYSIS_REBUILD_MD.md für Details zu Fehlern in rebuild.md
- Beachte INKOMPATIBILITAETEN.md für Konflikte
- Verwende TOOL_COMPATIBILITY_ANALYSIS.md für Tool-Entscheidungen
- Verwende THREE_JS_INTEGRATION_ANALYSIS.md für 3D-Implementierung
- Verwende PROJEKTPLAN.md für Meilensteine und Zeitschätzungen

