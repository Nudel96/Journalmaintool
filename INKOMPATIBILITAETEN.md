# INKOMPATIBILITÄTEN & KONFLIKTE

**Datum:** 2025-10-07  
**Status:** KRITISCHE ENTSCHEIDUNGEN ERFORDERLICH

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

## 🎯 NÄCHSTE SCHRITTE

1. ✅ **Entscheidungen mit User bestätigen**
2. ⏭️ Knowledge.md erstellen
3. ⏭️ Detaillierten Projektplan erstellen
4. ⏭️ Tech-Stack finalisieren
5. ⏭️ Implementierung starten

---

## 📞 FRAGEN AN USER

1. **Datenbank:** PostgreSQL oder SurrealDB (in Docker)?
2. **TailwindCSS:** v3.4 (stable) oder v4.0-alpha?
3. **Authentifizierung:** Eigenständig oder PriceActionTalk-Integration?
4. **Three.js:** Implementieren oder weglassen?
5. **Stripe-Tiers:** Welche Subscription-Pläne? (Preise, Features?)

