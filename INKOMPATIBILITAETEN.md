# INKOMPATIBILITÄTEN & KONFLIKTE

**Datum:** 2025-10-08
**Status:** GELÖST - Dokumentation aktualisiert

---

## 🚨 KRITISCHE INKOMPATIBILITÄTEN

### 1. DATENBANK: SurrealDB vs. PostgreSQL
**Konflikt:**
- rebuild.md fordert: SurrealDB 2.3.7
- Render bietet: PostgreSQL, Redis, Key-Value (NICHT SurrealDB)

**Auswirkung:** BLOCKIEREND für Deployment

**Lösungsoptionen:**
| Option | Vorteile | Nachteile | Empfehlung |
|--------|----------|-----------|------------|
| **A: PostgreSQL** | ✅ Native Render Support<br>✅ Managed Service<br>✅ Produktionsreif | ❌ Datenmodell anpassen<br>❌ SQL statt SurrealQL | ⭐ **EMPFOHLEN** |
| **B: SurrealDB in Docker** | ✅ Wie spezifiziert | ❌ Selbst verwalten<br>❌ Komplexer | ⚠️ Möglich |
| **C: SurrealDB Cloud** | ✅ Managed | ❌ Zusätzliche Kosten<br>❌ Latenz | ⚠️ Möglich |

**ENTSCHEIDUNG ERFORDERLICH:** Welche Option?

---

### 2. PACKAGE-VERSIONEN: Nicht-existierende Versionen
**Konflikt:**
rebuild.md spezifiziert Versionen, die NICHT existieren:

| Package | rebuild.md | Realität | Status |
|---------|-----------|----------|--------|
| Vite | 7.0.6 | 5.4.x (latest) | ❌ EXISTIERT NICHT |
| TailwindCSS | 4.1.11 | 3.4.x (stable), 4.0-alpha | ❌ EXISTIERT NICHT |
| Zod | 4.0.15 | 3.23.x (latest) | ❌ EXISTIERT NICHT |

**Auswirkung:** Build wird fehlschlagen

**Lösung:** Aktuelle stabile Versionen verwenden
- Vite: `^5.4.0`
- TailwindCSS: `^3.4.0` (oder `^4.0.0-alpha` wenn v4 gewünscht)
- Zod: `^3.23.0`

**ENTSCHEIDUNG ERFORDERLICH:** TailwindCSS v3 oder v4-alpha?

---

### 3. AUTHENTIFIZIERUNG: Eigenständig oder Integration?
**Konflikt:**
rebuild.md ist widersprüchlich:
- Sagt: "compatible with PriceActionTalk backend"
- Aber: Spezifiziert eigenes User-Model + JWT-Implementierung
- Keine PriceActionTalk API-Spezifikation vorhanden

**Auswirkung:** Unklar, was gebaut werden soll

**Lösungsoptionen:**
| Option | Beschreibung | Empfehlung |
|--------|--------------|------------|
| **A: Eigenständig** | Komplett eigenes Auth-System | ⭐ **EMPFOHLEN** (keine API verfügbar) |
| **B: Integration** | Mit PriceActionTalk Backend | ❌ Keine API-Spezifikation |

**ENTSCHEIDUNG ERFORDERLICH:** Eigenständig oder Integration?

---

### 4. THREE.JS: Komplett fehlend in rebuild.md
**Konflikt:**
- HTML verwendet: Three.js 0.160.0 mit komplexen 3D-Animationen
- rebuild.md erwähnt: ❌ NICHTS über Three.js

**Auswirkung:** 
- Zusätzliche 22h Entwicklungszeit
- ~600KB Bundle-Size
- Performance-Implikationen

**Lösung:** 
- @threlte/core für Svelte-Integration verwenden
- Performance-Optimierungen für Mobile

**ENTSCHEIDUNG ERFORDERLICH:** Three.js implementieren oder weglassen?

---

## ⚠️ DESIGN-KONFLIKTE

### 5. FARBPALETTEN: rebuild.md vs. HTML
**Konflikt:**
Unterschiedliche Farbwerte:
- `--error: #ff4444` (rebuild.md) vs `--danger: #ef4444` (HTML)
- `--text-secondary: #a0a0a0` (rebuild.md) vs `--muted: #cfd6dd` (HTML)

**Anweisung:** "Nutze die Farbpaletten aus den html"

**Lösung:** ✅ HTML-Farbpalette verwenden (wie angewiesen)

---

### 6. DESIGN-PHILOSOPHIE: Sharp Edges vs. Rounded Corners
**Konflikt:**
- rebuild.md: "NEVER use rounded corners (border-radius: 0)"
- HTML: Verwendet border-radius: 12px, 14px, 24px, 999px

**Anweisung:** "ignoriere designvorgaben aus rebuild.md"

**Lösung:** ✅ HTML-Design verwenden (rounded corners erlaubt)

---

## 📊 DATENMODELL-PROBLEME

### 7. USER-MODEL: Fehlende Stripe-Integration
**Problem:**
rebuild.md User-Model fehlt:
- ❌ Stripe Customer ID
- ❌ Subscription Status
- ❌ Subscription Tier
- ❌ Email Verification

**Lösung:** User-Model erweitern:
```rust
struct User {
    id: Uuid,
    name: String,
    email: String,
    email_verified: bool,              // NEU
    password: String,
    stripe_customer_id: Option<String>, // NEU
    subscription_status: SubscriptionStatus, // NEU
    subscription_tier: SubscriptionTier,     // NEU
    permissions: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_login: Option<DateTime<Utc>>, // NEU
}

enum SubscriptionStatus {
    Active,
    Canceled,
    PastDue,
    Trialing,
    None,
}

enum SubscriptionTier {
    Free,
    Pro,
    Lifetime,
}
```

---

### 8. TRADE-MODEL: Fehlende Felder
**Problem:**
rebuild.md Trade-Model fehlt:
- ❌ Broker Information
- ❌ Account ID (Multi-Account)
- ❌ Commission/Spread (separate)
- ❌ Trade Status

**Lösung:** Trade-Model erweitern

---

## 🏗️ ARCHITEKTUR-PROBLEME

### 9. MICROSERVICE-ARCHITEKTUR: Nur Monolith
**Problem:**
- rebuild.md sagt: "Microservice-ready"
- Aber: Nur 1 Backend, 1 DB, keine Service-Separation

**Lösung:** 
- "Microservice-ready" entfernen
- Als Monolith bauen (einfacher, schneller)

---

### 10. LOKALE ENTWICKLUNG: Stripe Webhooks
**Problem:**
- rebuild.md: "100% functional locally"
- Aber: Stripe Webhooks funktionieren NICHT lokal ohne Tools

**Lösung:**
- Stripe CLI für lokale Webhook-Tests
- Oder: Mock-Webhooks für Entwicklung

---

## 📝 FEHLENDE SPEZIFIKATIONEN

### 11. STRIPE-INTEGRATION: Keine Details
**Problem:**
rebuild.md sagt nur:
- "Subscription-Modell implementieren"
- "Lifetime-Access-Zuweisung"
- "Webhook-Handling"

**Fehlt:**
- ❌ Welche Subscription-Tiers? (Preise?)
- ❌ Welche Features pro Tier?
- ❌ Wie funktioniert Lifetime-Access technisch?
- ❌ Welche Webhooks genau?
- ❌ Wie wird Zugriff eingeschränkt?

**Lösung:** Detaillierte Stripe-Spezifikation erstellen

---

## ✅ EMPFOHLENE ENTSCHEIDUNGEN

### Datenbank
**Empfehlung:** PostgreSQL (Option A)
- Native Render Support
- Einfacher zu deployen
- Produktionsreif

### Package-Versionen
**Empfehlung:** Aktuelle stabile Versionen
- Vite 5.4.x
- TailwindCSS 3.4.x (oder 4.0-alpha wenn gewünscht)
- Zod 3.23.x

### Authentifizierung
**Empfehlung:** Eigenständiges System
- Keine PriceActionTalk API verfügbar
- Später Integration möglich

### Three.js
**Empfehlung:** Implementieren mit @threlte/core
- Ist in HTML-Referenz vorhanden
- Wichtig für Design
- Performance-Optimierungen für Mobile

### Design
**Empfehlung:** HTML-Design verwenden
- Wie angewiesen
- Farbpalette aus HTML
- Rounded corners erlaubt

---

## � NEU ENTDECKTE PROBLEME (2025-10-08)

### 11. TIMESTAMP vs TIMESTAMPTZ in PostgreSQL
**Problem:**
- Ursprüngliche Migrationen verwendeten `TIMESTAMP` ohne Zeitzone
- Rust-Code erwartet `DateTime<Utc>` (entspricht `TIMESTAMPTZ`)
- Führte zu Typ-Mismatch-Fehlern: "Rust type `DateTime<Utc>` is not compatible with SQL type `TIMESTAMP`"

**Betroffene Tabellen:**
- `users`: `created_at`, `updated_at`, `last_login`
- `trades`: `entry_time`, `exit_time`, `created_at`, `updated_at`

**Lösung:** ✅ GELÖST
```sql
-- Alle TIMESTAMP zu TIMESTAMPTZ geändert
created_at TIMESTAMPTZ DEFAULT NOW()
updated_at TIMESTAMPTZ DEFAULT NOW()
entry_time TIMESTAMPTZ NOT NULL
exit_time TIMESTAMPTZ
last_login TIMESTAMPTZ
```

**Commit:** `3dd2025` - "Fix: Database timestamp types and Svelte class directive syntax"

---

### 12. Svelte class: Direktiven mit Schrägstrichen
**Problem:**
- Svelte erlaubt **keine Schrägstriche (`/`) in CSS-Klassennamen** bei `class:` Direktiven
- Tailwind-Klassen wie `bg-accent/20` (Opacity-Modifier) funktionieren nicht mit `class:`
- Führte zu Kompilierungsfehler: "Expected token >"

**Beispiel (funktioniert NICHT):**
```svelte
<span class:bg-accent/20={condition}>
```

**Lösung:** ✅ GELÖST
Template-Literal-Syntax verwenden statt `class:` Direktiven:
```svelte
<span class={`inline-flex items-center ${
  trade.direction === 'long'
    ? 'bg-accent/20 text-accent'
    : 'bg-danger/20 text-danger'
}`}>
```

**Betroffene Dateien:**
- `frontend/src/routes/dashboard/+page.svelte`

**Commit:** `3dd2025` - "Fix: Database timestamp types and Svelte class directive syntax"

**Wichtig für zukünftige Entwicklung:**
- ⚠️ Verwende `class:` nur für Klassen OHNE Schrägstriche
- ⚠️ Für Tailwind-Opacity-Modifier (`/20`, `/50`, etc.) immer Template-Literals verwenden

---

## �🎯 NÄCHSTE SCHRITTE

1. ✅ **Entscheidungen mit User bestätigt**
2. ✅ **Knowledge.md erstellt und aktualisiert**
3. ✅ **Detaillierten Projektplan erstellt**
4. ✅ **Tech-Stack finalisiert**
5. ✅ **Implementierung gestartet**
6. ✅ **Backend & Frontend funktionsfähig**
7. ⏭️ **Stripe-Integration**
8. ⏭️ **3D-Animationen**
9. ⏭️ **Analytics & Charts**

---

## 📞 GELÖSTE FRAGEN

1. ✅ **Datenbank:** PostgreSQL (Render managed)
2. ✅ **TailwindCSS:** v3.4 (stable)
3. ✅ **Authentifizierung:** Eigenständig (JWT-basiert)
4. ⏭️ **Three.js:** Noch zu implementieren
5. ✅ **Stripe-Tiers:** 1/6/12 Monate ($7/$5/$4)

