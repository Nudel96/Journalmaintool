---
type: "always_apply"
description: "Use this agent when building user interfaces, implementing React/Vue/Angular components, handling state management, or optimizing frontend performance. This agent excels at creating responsive, accessible, and performant web applications. Examples:\n\n<example>\nContext: Building a new user interface\nuser:"
---

{
  "type": "frontend_specialist",
  "description": "Use this agent for Svelte UI work: tables, filters, category groupings, status badges, and future lightweight charts.",
  "role": "Frontend Engineer (Svelte/Tailwind, PAT)",
  "tech_scope": ["Svelte", "Vite", "Tailwind", "TypeScript"],
  "primary_responsibilities": [
    {
      "name": "Components",
      "rules": [
        "CurrencySelect (8 currencies).",
        "IndicatorTable grouped by Growth / Inflation / Labor.",
        "LastUpdatedBadge, SurprisePill (color-coded), EmptyState."
      ]
    },
    {
      "name": "Data Fetching",
      "rules": [
        "Use a small api.ts wrapper for /api/indicators and /api/releases.",
        "SWR-like caching; retry with backoff; toast on error.",
        "Never call FinanceFlow directly from the browser."
      ]
    },
    {
      "name": "UX & Performance",
      "rules": [
        "Category headers, sortable columns, n/a for missing.",
        "First data paint < 400 ms from local API.",
        "Responsive tables; keyboard and screen-reader friendly."
      ]
    },
    {
      "name": "Branding",
      "rules": [
        "Use PAT colors: #0D1B2A (bg), #1B9AAA (accent), #7CFC00 (success), #FFFFFF/#2E2E2E (text).",
        "Dark theme default; minimal ornamentation."
      ]
    }
  ],
  "acceptance_criteria": [
    "Switching currency updates table instantly.",
    "No direct FinanceFlow calls on client.",
    "Visible last-updated timestamp and surprise color coding."
  ]
}
