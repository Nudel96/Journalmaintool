# 📊 Trading Journal Tool - PriceActionTalk

> Professional Trading Journal mit Stripe-Subscriptions, 3D-Animationen und Advanced Analytics

[![Status](https://img.shields.io/badge/Status-In%20Development-green)]()
[![License](https://img.shields.io/badge/License-MIT-blue)]()
[![Backend](https://img.shields.io/badge/Backend-Rust%20%2B%20Actix--Web-orange)]()
[![Frontend](https://img.shields.io/badge/Frontend-SvelteKit-red)]()

---

## 🎯 Projekt-Übersicht

Ein modernes, professionelles Trading Journal Tool für Trader, die ihre Performance systematisch tracken und verbessern möchten.

**Aktueller Status:** Backend und Frontend funktionsfähig, Authentifizierung implementiert, Dashboard in Entwicklung.

### Kern-Features
- ✅ **Benutzer-Authentifizierung** - Sicheres Login/Register-System mit JWT
- ✅ **Stripe-Subscriptions** - Free, Pro und Lifetime-Pläne
- ✅ **Trading-Journal** - Umfassendes Trade-Management mit Tags, Notes, Screenshots
- ✅ **Advanced Analytics** - Win-Rate, Profit-Factor, Drawdown, Sharpe-Ratio
- ✅ **Interactive Charts** - Lightweight Charts, Chart.js, D3.js
- ✅ **3D-Animationen** - Beeindruckender Login-Screen mit Three.js
- ✅ **Responsive Design** - Mobile-First, Dark Theme

---

## 🏗️ Architektur

```
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│  SvelteKit Frontend │    │   Rust Backend      │    │   PostgreSQL        │
│   (Vercel)          │◄──►│   (Render)          │◄──►│   (Render)          │
│   Port: 5173        │    │   Port: 3000        │    │   Port: 5432        │
└─────────────────────┘    └─────────────────────┘    └─────────────────────┘
         │                           │                           
         ▼                           ▼                           
┌─────────────────────┐    ┌─────────────────────┐    
│   Stripe API        │    │   Sentry            │    
│   (Payments)        │    │   (Monitoring)      │    
└─────────────────────┘    └─────────────────────┘    
```

---

## 🛠️ Technologie-Stack

### Frontend
- **Framework:** SvelteKit (latest)
- **Styling:** TailwindCSS 3.4.x
- **3D Graphics:** Three.js + @threlte/core
- **Charts:** Lightweight Charts, Chart.js
- **Validation:** Zod 3.x
- **Deployment:** Vercel

### Backend
- **Framework:** Rust + Actix-Web 4.x ✅
- **Database:** PostgreSQL 15+ (Docker lokal, Render für Production) ✅
- **ORM:** sqlx 0.7.x ✅
- **Auth:** JWT (jsonwebtoken) + Argon2 ✅
- **Deployment:** Render (geplant)

### Services
- **Payments:** Stripe
- **Error Tracking:** Sentry
- **CI/CD:** GitHub Actions

---

## 📁 Projekt-Struktur

```
trading-journal/
├── frontend/                    # SvelteKit App
│   ├── src/
│   │   ├── routes/              # Pages
│   │   ├── lib/
│   │   │   ├── components/      # Svelte Components
│   │   │   │   ├── 3d/          # Three.js Components
│   │   │   │   ├── ui/          # UI Components
│   │   │   │   ├── charts/      # Chart Components
│   │   │   │   └── trades/      # Trade Components
│   │   │   ├── stores/          # Svelte Stores
│   │   │   ├── api/             # API Client
│   │   │   └── types/           # TypeScript Types
│   │   └── app.css              # Global Styles
│   └── package.json
├── backend/                     # Rust Backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── handlers/            # API Handlers
│   │   ├── models/              # Data Models
│   │   └── db/                  # Database Layer
│   ├── migrations/              # SQL Migrations
│   └── Cargo.toml
├── docker-compose.yml           # Local Development
├── KNOWLEDGE.md                 # Project Knowledge Base
├── PROJEKTPLAN.md               # Detailed Project Plan
└── README.md                    # This file
```

---

## 🚀 Quick Start

### Voraussetzungen
- Node.js 18+ (für Frontend)
- Rust 1.70+ (für Backend)
- Docker & Docker Compose (für lokale DB)
- Stripe Account (für Payments)

### 1. Repository klonen
```bash
git clone https://github.com/Nudel96/Journalmaintool.git
cd Journalmaintool
```

### 2. Frontend Setup
```bash
cd frontend
npm install
cp .env.example .env
# .env bearbeiten: VITE_API_URL, VITE_STRIPE_PUBLIC_KEY
npm run dev
```

Frontend läuft auf: http://localhost:5173

### 3. Backend Setup
```bash
cd backend
cp .env.example .env
# .env bearbeiten: DATABASE_URL, JWT_SECRET, STRIPE_SECRET_KEY
cargo build
cargo run
```

Backend läuft auf: http://localhost:3000

### 4. Datenbank Setup
```bash
# PostgreSQL in Docker starten
docker run --name trading-journal-db \
  -e POSTGRES_PASSWORD=postgres \
  -p 5433:5432 \
  -d postgres:15

# Datenbank erstellen
docker exec trading-journal-db psql -U postgres -c "CREATE DATABASE trading_journal;"

# Migrations ausführen (automatisch beim Backend-Start)
cd backend
cargo run
```

---

## 🎨 Design-System

### Farbpalette
```css
--bg: #0d1b2a;               /* Deep Navy */
--bg-2: #2e2e2e;             /* Dark Gray */
--brand: #1b9aaa;            /* Turquoise */
--accent: #7cfc00;           /* Lime Green */
--success: #7cfc00;          /* Success */
--danger: #ef4444;           /* Error */
--text: #ffffff;             /* White */
--muted: #cfd6dd;            /* Light Muted */
```

### Typografie
- **Font:** Inter (Google Fonts)
- **Sizes:** 12px, 14px, 16px, 24px

### Border Radius
- Cards: 24px
- Buttons: 12px
- Inputs: 14px

---

## 💳 Subscription-Pläne

| Plan | Preis | Features |
|------|-------|----------|
| **Free** | $0/Monat | 10 Trades/Monat, Basis-Analytics |
| **Pro** | $19/Monat | Unlimited Trades, Advanced Analytics, Export |
| **Lifetime** | $299 einmalig | Alle Pro-Features, lebenslang |

---

## 📊 Datenmodelle

### User
- ID, Name, Email, Password (Argon2)
- Stripe Customer ID
- Subscription Status & Tier
- Permissions

### Trade
- ID, User ID, Symbol, Direction (Long/Short)
- Entry/Exit Price & Time
- Quantity, PnL, Fees
- Notes, Tags, Screenshots
- Setup Type, Mistakes, Emotions

---

## 🧪 Testing

### Backend Tests
```bash
cd backend
cargo test
```

### Frontend Tests
```bash
cd frontend
npm run test          # Unit Tests (Vitest)
npm run test:e2e      # E2E Tests (Playwright)
```

---

## 🚢 Deployment

### Frontend (Vercel)
```bash
cd frontend
vercel --prod
```

### Backend (Render)
1. Render Web Service erstellen
2. PostgreSQL Datenbank erstellen
3. Environment Variables setzen
4. Deploy via Git Push

---

## 📚 Dokumentation

- **[KNOWLEDGE.md](KNOWLEDGE.md)** - Projekt-Wissensdatenbank
- **[PROJEKTPLAN.md](PROJEKTPLAN.md)** - Detaillierter Projektplan
- **[ANALYSIS_REBUILD_MD.md](ANALYSIS_REBUILD_MD.md)** - rebuild.md Analyse
- **[TOOL_COMPATIBILITY_ANALYSIS.md](TOOL_COMPATIBILITY_ANALYSIS.md)** - Tool-Kompatibilität
- **[THREE_JS_INTEGRATION_ANALYSIS.md](THREE_JS_INTEGRATION_ANALYSIS.md)** - 3D-Integration
- **[INKOMPATIBILITAETEN.md](INKOMPATIBILITAETEN.md)** - Bekannte Konflikte

---

## 🗺️ Roadmap

### Phase 1: Setup ✅ ABGESCHLOSSEN
- [x] Repository-Verbindung
- [x] Anforderungsanalyse
- [x] Kompatibilitätsprüfung
- [x] Projektplanung
- [x] Projekt-Setup (Frontend + Backend)
- [x] Docker PostgreSQL Setup

### Phase 2: Backend ✅ ABGESCHLOSSEN
- [x] Projekt-Setup (Actix-Web)
- [x] Datenbank-Migrations (PostgreSQL mit TIMESTAMPTZ)
- [x] Authentifizierung (JWT + Argon2)
- [x] Trade-CRUD (Backend-Handlers)
- [x] Error-Handling & Logging

### Phase 3: Frontend ✅ ABGESCHLOSSEN
- [x] Layout & UI-Komponenten (Tailwind)
- [x] Login/Register (mit Svelte Stores)
- [x] Dashboard (Basis-Implementierung)
- [x] Trade-Management (in Progress)

### Phase 4: Advanced Features 🔄 IN PROGRESS
- [ ] Stripe-Integration (nächster Schritt)
- [ ] 3D-Animationen (geplant)
- [ ] Analytics & Charts (geplant)

### Phase 5: Deployment ⏸️ GEPLANT
- [ ] Testing & QA
- [ ] Production-Deployment (Vercel + Render)
- [ ] Monitoring (Sentry)

---

## 🤝 Contributing

Dieses Projekt ist aktuell in aktiver Entwicklung. Contributions sind willkommen!

1. Fork das Repository
2. Erstelle einen Feature-Branch (`git checkout -b feature/AmazingFeature`)
3. Commit deine Änderungen (`git commit -m 'Add some AmazingFeature'`)
4. Push zum Branch (`git push origin feature/AmazingFeature`)
5. Öffne einen Pull Request

---

## 📝 License

Dieses Projekt ist unter der MIT License lizenziert.

---

## 📞 Kontakt

**Projekt-Link:** https://github.com/Nudel96/Journalmaintool

---

## 🙏 Acknowledgments

- [SvelteKit](https://kit.svelte.dev/) - Frontend Framework
- [Axum](https://github.com/tokio-rs/axum) - Backend Framework
- [Three.js](https://threejs.org/) - 3D Graphics
- [Stripe](https://stripe.com/) - Payment Processing
- [TailwindCSS](https://tailwindcss.com/) - Styling
- [Lightweight Charts](https://tradingview.github.io/lightweight-charts/) - Trading Charts

---

**Made with ❤️ for Traders**

