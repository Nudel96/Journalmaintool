# TOOL-KOMPATIBILITÄTS-ANALYSE

**Datum:** 2025-10-07  
**Zweck:** Prüfung der verfügbaren MCP Tools gegen rebuild.md Anforderungen

---

## 🛠️ VERFÜGBARE MCP TOOLS

### 1. **Stripe** (Zahlungen & Subscriptions)
**Verfügbare Funktionen:**
- ✅ `create_customer` - Kunden erstellen
- ✅ `list_customers` - Kunden auflisten
- ✅ `create_product` - Produkte erstellen
- ✅ `create_price` - Preise erstellen
- ✅ `create_payment_link` - Zahlungslinks erstellen
- ✅ `create_invoice` - Rechnungen erstellen
- ✅ `list_subscriptions` - Abos auflisten
- ✅ `cancel_subscription` - Abos kündigen
- ✅ `update_subscription` - Abos aktualisieren
- ✅ `create_coupon` - Gutscheine erstellen

**Kompatibilität mit Anforderungen:**
- ✅ **Subscription-Modell:** VOLL UNTERSTÜTZT
- ✅ **Lifetime-Access:** Möglich via Coupons oder spezielle Subscription
- ⚠️ **Webhook-Handling:** Tool bietet KEINE Webhook-Endpoints (muss im Backend implementiert werden)

**EMPFEHLUNG:**
- Stripe MCP Tool für Subscription-Verwaltung verwenden
- Webhooks im Rust Backend implementieren (Axum Route)
- Stripe Customer ID im User-Model speichern

---

### 2. **Vercel** (Frontend Deployment)
**Verfügbare Funktionen:**
- ✅ `deploy_to_vercel` - Deployment
- ✅ `list_projects` - Projekte auflisten
- ✅ `get_project` - Projekt-Details
- ✅ `list_deployments` - Deployments auflisten
- ✅ `get_deployment` - Deployment-Details
- ✅ `get_deployment_build_logs` - Build-Logs

**Kompatibilität mit Anforderungen:**
- ✅ **SvelteKit Deployment:** VOLL UNTERSTÜTZT (Vercel hat native SvelteKit Support)
- ✅ **Environment Variables:** Unterstützt
- ✅ **Custom Domains:** Unterstützt
- ✅ **Preview Deployments:** Unterstützt

**EMPFEHLUNG:**
- Vercel für Frontend-Deployment verwenden
- Automatische Deployments via GitHub Integration

---

### 3. **Render** (Backend Deployment)
**Verfügbare Funktionen:**
- ✅ `create_web_service` - Web Services erstellen
- ✅ `create_postgres` - PostgreSQL Datenbanken erstellen
- ✅ `create_key_value` - Redis/Key-Value Stores erstellen
- ✅ `get_service` - Service-Details
- ✅ `list_services` - Services auflisten
- ✅ `get_metrics` - Performance-Metriken
- ✅ `list_logs` - Logs abrufen
- ✅ `update_environment_variables` - Env-Vars aktualisieren

**Kompatibilität mit Anforderungen:**
- ✅ **Rust Backend:** UNTERSTÜTZT (Render unterstützt Rust via Docker)
- ❌ **SurrealDB:** NICHT UNTERSTÜTZT (Render bietet nur PostgreSQL, Redis, Key-Value)
- ✅ **PostgreSQL:** VOLL UNTERSTÜTZT (managed service)
- ✅ **Docker:** UNTERSTÜTZT (für SurrealDB Container)

**KRITISCHER KONFLIKT:**
rebuild.md fordert SurrealDB, aber Render bietet nur PostgreSQL als managed DB.

**LÖSUNGSOPTIONEN:**

**Option A: PostgreSQL verwenden (EMPFOHLEN)**
- ✅ Native Render Support
- ✅ Managed Service (Backups, Scaling, etc.)
- ✅ Rust Libraries verfügbar (sqlx, diesel)
- ❌ Datenmodell muss angepasst werden (SQL statt SurrealQL)

**Option B: SurrealDB in Docker**
- ✅ Wie in rebuild.md spezifiziert
- ❌ Selbst verwalten (Backups, Updates, etc.)
- ❌ Zusätzliche Komplexität
- ⚠️ Render Docker Support prüfen

**Option C: Externe SurrealDB Cloud**
- ✅ Managed SurrealDB
- ❌ Zusätzliche Kosten
- ❌ Latenz (wenn nicht in gleicher Region)

**EMPFEHLUNG:**
- **PostgreSQL verwenden** (Option A)
- Begründung:
  - Einfacher zu deployen
  - Besser unterstützt
  - Produktionsreif
  - Render native Integration

---

### 4. **Sentry** (Error Tracking)
**Verfügbare Funktionen:**
- ✅ `whoami` - User-Info
- ✅ `find_organizations` - Organisationen finden
- ✅ `find_projects` - Projekte finden
- ✅ `get_issue_details` - Issue-Details
- ✅ `search_events` - Events suchen
- ✅ `search_issues` - Issues suchen

**Kompatibilität mit Anforderungen:**
- ✅ **Frontend Error Tracking:** VOLL UNTERSTÜTZT (Sentry 10.17.0 in rebuild.md)
- ✅ **Backend Error Tracking:** UNTERSTÜTZT (Sentry Rust SDK verfügbar)
- ✅ **Performance Monitoring:** UNTERSTÜTZT

**EMPFEHLUNG:**
- Sentry für Error Tracking verwenden
- Frontend: @sentry/svelte
- Backend: sentry Rust crate

---

### 5. **CircleCI** (CI/CD)
**Verfügbare Funktionen:**
- ✅ `get_build_failure_logs` - Build-Logs
- ✅ `find_flaky_tests` - Flaky Tests finden
- ✅ `get_latest_pipeline_status` - Pipeline-Status
- ✅ `run_pipeline` - Pipeline ausführen
- ✅ `config_helper` - Config validieren

**Kompatibilität mit Anforderungen:**
- ✅ **CI/CD:** UNTERSTÜTZT
- ⚠️ rebuild.md erwähnt GitHub Actions, nicht CircleCI

**EMPFEHLUNG:**
- CircleCI ODER GitHub Actions verwenden
- Beide sind kompatibel

---

## 📊 KOMPATIBILITÄTS-MATRIX

| Anforderung | rebuild.md | Verfügbare Tools | Status | Empfehlung |
|-------------|-----------|------------------|--------|------------|
| **Frontend Framework** | SvelteKit 2.27.1 | - | ✅ | SvelteKit (aktuelle Version) |
| **Frontend Deployment** | - | Vercel | ✅ | Vercel verwenden |
| **Backend Framework** | Rust/Axum 0.8.4 | - | ✅ | Rust/Axum (aktuelle Version) |
| **Backend Deployment** | - | Render | ✅ | Render verwenden |
| **Datenbank** | SurrealDB 2.3.7 | Render PostgreSQL | ❌ | **PostgreSQL verwenden** |
| **Authentifizierung** | JWT (custom) | - | ✅ | Selbst implementieren |
| **Zahlungen** | - | Stripe | ✅ | Stripe verwenden |
| **Error Tracking** | Sentry 10.17.0 | Sentry | ✅ | Sentry verwenden |
| **CI/CD** | GitHub Actions | CircleCI | ⚠️ | CircleCI oder GitHub Actions |
| **3D Graphics** | ❌ NICHT erwähnt | - | ⚠️ | Three.js + @threlte/core |

---

## 🚨 KRITISCHE ENTSCHEIDUNGEN ERFORDERLICH

### Entscheidung 1: Datenbank
**Frage:** SurrealDB (wie rebuild.md) oder PostgreSQL (Render native)?

**Argumente für PostgreSQL:**
- ✅ Native Render Support
- ✅ Managed Service
- ✅ Produktionsreif
- ✅ Große Community
- ✅ Viele Rust Libraries

**Argumente für SurrealDB:**
- ✅ Wie in rebuild.md spezifiziert
- ✅ Moderne Features (Graph-DB, etc.)
- ❌ Selbst hosten erforderlich
- ❌ Weniger mature

**EMPFEHLUNG:** PostgreSQL

---

### Entscheidung 2: Authentifizierung
**Frage:** Eigenständiges System oder PriceActionTalk Integration?

**rebuild.md ist unklar:**
- Sagt "compatible with PriceActionTalk backend"
- Aber spezifiziert eigenes User-Model
- Keine PriceActionTalk API-Spezifikation

**EMPFEHLUNG:** 
- **Eigenständiges System** bauen
- Begründung: Keine PriceActionTalk API verfügbar
- Später Integration möglich wenn gewünscht

---

### Entscheidung 3: Package-Versionen
**Frage:** rebuild.md Versionen (teilweise nicht-existent) oder aktuelle Versionen?

**rebuild.md Versionen:**
- ❌ Vite 7.0.6 (existiert nicht)
- ❌ TailwindCSS 4.1.11 (existiert nicht)
- ❌ Zod 4.0.15 (existiert nicht)

**EMPFEHLUNG:**
- **Aktuelle stabile Versionen** verwenden
- Vite 5.4.x
- TailwindCSS 3.4.x
- Zod 3.23.x

---

## ✅ FINALER TECH-STACK (EMPFOHLEN)

### Frontend
- **Framework:** SvelteKit (latest stable)
- **Styling:** TailwindCSS 3.4.x
- **3D Graphics:** Three.js + @threlte/core
- **Charts:** Lightweight Charts, Chart.js
- **Validation:** Zod 3.x
- **Deployment:** Vercel

### Backend
- **Framework:** Rust + Axum (latest stable)
- **Database:** PostgreSQL (Render managed)
- **ORM:** sqlx oder diesel
- **Auth:** JWT (jsonwebtoken crate)
- **Deployment:** Render

### Services
- **Payments:** Stripe
- **Error Tracking:** Sentry
- **CI/CD:** CircleCI oder GitHub Actions

### Development
- **Containerization:** Docker + Docker Compose
- **Local DB:** PostgreSQL in Docker

---

## 📝 NÄCHSTE SCHRITTE

1. ✅ Entscheidungen mit User bestätigen
2. ⏭️ Three.js Integration analysieren (Task 1.4)
3. ⏭️ Inkompatibilitäten dokumentieren (Task 1.5)
4. ⏭️ Knowledge.md erstellen
5. ⏭️ Detaillierten Projektplan erstellen

