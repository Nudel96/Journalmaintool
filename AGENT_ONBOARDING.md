# 🤖 AGENT ONBOARDING GUIDE

**Für:** Neue Augment Coding Agents  
**Projekt:** Trading Journal Tool  
**Letzte Aktualisierung:** 2025-10-08

---

## 📚 SCHRITT 1: DOKUMENTATION LESEN (Empfohlene Reihenfolge)

### 1.1 Zuerst lesen (PFLICHT):
1. **`README.md`** (5 min)
   - Projekt-Übersicht
   - Technologie-Stack
   - Quick-Start-Anleitung
   - Aktueller Status

2. **`KNOWLEDGE.md`** (15 min)
   - Vollständige Projekt-Wissensdatenbank
   - Technische Entscheidungen
   - Datenmodelle
   - Design-System
   - Bekannte Probleme

3. **`INKOMPATIBILITAETEN.md`** (10 min)
   - Gelöste und bekannte Konflikte
   - Wichtige Workarounds
   - **NEU:** TIMESTAMPTZ-Problem
   - **NEU:** Svelte class:-Direktiven-Problem

### 1.2 Danach lesen (EMPFOHLEN):
4. **`PROJEKTPLAN.md`** (10 min)
   - Meilensteine und Status
   - Zeitschätzungen
   - Nächste Schritte

5. **`CORRECTED_REBUILD.md`** (10 min)
   - Korrigierte Spezifikation
   - Unterschiede zur Original rebuild.md
   - **WICHTIG:** Verwende DIESE Datei, NICHT die Original rebuild.md!

### 1.3 Bei Bedarf lesen (OPTIONAL):
6. **`STRIPE_SETUP.md`** - Wenn du an Stripe-Integration arbeitest
7. **`THREE_JS_INTEGRATION_ANALYSIS.md`** - Wenn du an 3D-Animationen arbeitest
8. **`TOOL_COMPATIBILITY_ANALYSIS.md`** - Für Tool-Entscheidungen

---

## 🔍 SCHRITT 2: BACKEND-STRUKTUR VERSTEHEN

### 2.1 Technologie-Stack
- **Framework:** Actix-Web 4.x (NICHT Axum wie ursprünglich geplant!)
- **Datenbank:** PostgreSQL 15+ (NICHT SurrealDB!)
- **ORM:** sqlx 0.7.x
- **Auth:** JWT + Argon2

### 2.2 Wichtige Backend-Dateien
```
backend/
├── src/
│   ├── main.rs              # Entry Point, Server-Setup
│   ├── config.rs            # Konfiguration, Env-Vars
│   ├── db.rs                # Datenbank-Verbindung, Migrations
│   ├── error.rs             # Error-Handling
│   ├── handlers/            # API-Endpoints
│   │   ├── auth.rs          # Register, Login
│   │   ├── trades.rs        # Trade-CRUD
│   │   └── analytics.rs     # Statistiken
│   ├── models/              # Datenmodelle
│   │   ├── user.rs          # User-Model
│   │   └── trade.rs         # Trade-Model
│   ├── repositories/        # Datenbank-Queries
│   │   ├── user_repository.rs
│   │   └── trade_repository.rs
│   └── services/            # Business-Logik
│       ├── auth_service.rs
│       └── analytics_service.rs
└── migrations/              # SQL-Migrations
    ├── 20250107_001_create_users.sql
    └── 20250107_002_create_trades.sql
```

### 2.3 Backend-Architektur-Muster
- **Repository-Pattern:** Datenbank-Zugriff in `repositories/`
- **Service-Layer:** Business-Logik in `services/`
- **Handler-Layer:** HTTP-Endpoints in `handlers/`
- **Error-Handling:** Custom `AppError` mit `thiserror`

### 2.4 Wichtige Backend-Konventionen
- **Ports:** Backend läuft auf Port 3000 (NICHT 8080!)
- **Database URL:** `postgresql://postgres:postgres@localhost:5433/trading_journal`
- **Migrations:** Laufen automatisch beim Server-Start
- **Timestamps:** IMMER `TIMESTAMPTZ` verwenden, NIEMALS `TIMESTAMP`!

---

## 🎨 SCHRITT 3: FRONTEND-STRUKTUR VERSTEHEN

### 3.1 Technologie-Stack
- **Framework:** SvelteKit 2.x
- **Styling:** TailwindCSS 3.4.x
- **Validation:** Zod 3.x
- **3D Graphics:** Three.js + @threlte/core (noch nicht implementiert)

### 3.2 Wichtige Frontend-Dateien
```
frontend/
├── src/
│   ├── routes/              # SvelteKit-Routes
│   │   ├── +layout.svelte   # Haupt-Layout
│   │   ├── +page.svelte     # Landing-Page
│   │   ├── login/           # Login-Page
│   │   ├── register/        # Register-Page
│   │   └── dashboard/       # Dashboard (in Progress)
│   ├── lib/
│   │   ├── components/      # Wiederverwendbare Komponenten
│   │   ├── stores/          # Svelte Stores (Auth, etc.)
│   │   ├── api/             # API-Client
│   │   └── types/           # TypeScript-Types
│   └── app.css              # Global Styles (Tailwind)
└── package.json
```

### 3.3 Frontend-Architektur-Muster
- **File-based Routing:** SvelteKit-Konvention
- **Svelte Stores:** Für globalen State (Auth, User)
- **API-Client:** Zentraler Fetch-Wrapper in `lib/api/`
- **Component-Struktur:** Atomic Design (atoms, molecules, organisms)

### 3.4 Wichtige Frontend-Konventionen
- **Ports:** Frontend läuft auf Port 5173
- **API-URL:** `http://localhost:3000` (Backend)
- **Tailwind-Opacity:** NIEMALS `class:bg-accent/20` verwenden! Stattdessen Template-Literals!
- **Dark Theme:** Standard-Theme, definiert in `app.css`

---

## ⚙️ SCHRITT 4: ARCHITEKTUR-ENTSCHEIDUNGEN

### 4.1 Kritische Entscheidungen (BEREITS GETROFFEN)
1. **Datenbank:** PostgreSQL (NICHT SurrealDB)
   - Grund: Render-Kompatibilität, Managed Service
   
2. **Backend-Framework:** Actix-Web (NICHT Axum)
   - Grund: Bereits implementiert
   
3. **Package-Versionen:** Aktuelle stabile Versionen
   - Vite 5.4.x (NICHT 7.0.6)
   - TailwindCSS 3.4.x (NICHT 4.1.11)
   - Zod 3.23.x (NICHT 4.0.15)

4. **Authentifizierung:** Eigenständiges System
   - JWT-basiert, KEINE PriceActionTalk-Integration

5. **Stripe-Tiers:** 1/6/12 Monate ($7/$5/$4)
   - KEIN Lifetime-Plan mehr

### 4.2 Design-Entscheidungen
- **Farbpalette:** Aus HTML (NICHT aus rebuild.md)
- **Border Radius:** Rounded corners ERLAUBT (NICHT "sharp edges only")
- **Danger-Color:** `#ef4444` (NICHT `#ff4444`)

---

## 🐛 SCHRITT 5: BEKANNTE PROBLEME & WORKAROUNDS

### Problem 1: TIMESTAMP vs TIMESTAMPTZ ✅ GELÖST
**Symptom:** `ColumnDecode` Error: "Rust type `DateTime<Utc>` is not compatible with SQL type `TIMESTAMP`"

**Ursache:** PostgreSQL `TIMESTAMP` hat keine Zeitzone, Rust `DateTime<Utc>` erwartet `TIMESTAMPTZ`

**Lösung:** IMMER `TIMESTAMPTZ` in Migrationen verwenden:
```sql
created_at TIMESTAMPTZ DEFAULT NOW()  -- ✅ RICHTIG
created_at TIMESTAMP DEFAULT NOW()    -- ❌ FALSCH
```

**Commit:** `3dd2025`

---

### Problem 2: Svelte class: mit Schrägstrichen ✅ GELÖST
**Symptom:** Kompilierungsfehler "Expected token >" bei `class:bg-accent/20={condition}`

**Ursache:** Svelte erlaubt KEINE Schrägstriche in `class:` Direktiven

**Lösung:** Template-Literals verwenden:
```svelte
<!-- ❌ FALSCH -->
<span class:bg-accent/20={condition}>

<!-- ✅ RICHTIG -->
<span class={`inline-flex ${condition ? 'bg-accent/20' : 'bg-danger/20'}`}>
```

**Commit:** `3dd2025`

---

### Problem 3: rebuild.md enthält Fehler ⚠️ WICHTIG
**Symptom:** Nicht-existierende Package-Versionen, widersprüchliche Anforderungen

**Lösung:** Verwende `CORRECTED_REBUILD.md` statt der Original rebuild.md!

---

## 🚀 SCHRITT 6: SETUP & ENTWICKLUNG

### 6.1 Lokale Entwicklungsumgebung
```bash
# 1. PostgreSQL starten
docker run --name trading-journal-db \
  -e POSTGRES_PASSWORD=postgres \
  -p 5433:5432 \
  -d postgres:15

# 2. Datenbank erstellen
docker exec trading-journal-db psql -U postgres -c "CREATE DATABASE trading_journal;"

# 3. Backend starten (Terminal 1)
cd backend
cargo run  # Läuft auf Port 3000

# 4. Frontend starten (Terminal 2)
cd frontend
npm run dev  # Läuft auf Port 5173
```

### 6.2 Wichtige Ports
- **Frontend:** 5173
- **Backend:** 3000
- **PostgreSQL:** 5433 (lokal), 5432 (Docker intern)

### 6.3 Environment Variables
**Backend (.env):**
```
DATABASE_URL=postgresql://postgres:postgres@localhost:5433/trading_journal
JWT_SECRET=your-secret-key-here
RUST_LOG=info
```

**Frontend (.env):**
```
VITE_API_URL=http://localhost:3000
```

---

## 📝 SCHRITT 7: CODING-STANDARDS

### 7.1 Rust-Konventionen
- **Error-Handling:** Verwende `AppError` aus `error.rs`
- **Async:** Alle Handler sind `async fn`
- **Logging:** Verwende `tracing::info!`, `tracing::error!`, etc.
- **Naming:** Snake_case für Funktionen, PascalCase für Structs

### 7.2 Svelte-Konventionen
- **Stores:** Verwende `$` für Store-Zugriff
- **Reactivity:** Verwende `$:` für reaktive Statements
- **Props:** Verwende `export let` für Component-Props
- **Naming:** camelCase für Variablen, PascalCase für Components

### 7.3 Git-Konventionen
- **Commit-Messages:** `Fix:`, `Feat:`, `Docs:`, `Refactor:`, etc.
- **Branch-Naming:** `feature/`, `bugfix/`, `hotfix/`

---

## ✅ CHECKLISTE FÜR NEUE AGENTS

- [ ] README.md gelesen
- [ ] KNOWLEDGE.md gelesen
- [ ] INKOMPATIBILITAETEN.md gelesen
- [ ] Backend-Struktur verstanden
- [ ] Frontend-Struktur verstanden
- [ ] Lokale Umgebung aufgesetzt
- [ ] Backend läuft (Port 3000)
- [ ] Frontend läuft (Port 5173)
- [ ] PostgreSQL läuft (Port 5433)
- [ ] TIMESTAMPTZ-Problem verstanden
- [ ] Svelte class:-Problem verstanden
- [ ] CORRECTED_REBUILD.md vs rebuild.md verstanden

---

**Viel Erfolg! 🚀**

Bei Fragen: Lese zuerst KNOWLEDGE.md und INKOMPATIBILITAETEN.md!

