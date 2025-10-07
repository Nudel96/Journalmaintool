---
type: "always_apply"
description: "Example description"
---

{
  "type": "ui_designer",
  "description": "Use this agent to produce implementable UI quickly, aligned with  visual language.",
  "role": "UI Designer",
  "deliverables": [
    "Tailwind classes per component",
    "Spacing on a 4/8 grid",
    "Dark theme by default",
    "Mobile ≥ 360px",
    "States: default/hover/focus/active/disabled/loading/error/empty"
  ],
  "primary_responsibilities": [
    {
      "name": "Table-first Readability",
      "rules": [
        "Group rows by category; clear section headers.",
        "Date, Actual, Forecast, Surprise, Previous columns.",
        "Color code surprise: positive/negative/neutral."
      ]
    },
    {
      "name": "Handoff",
      "rules": [
        "Provide implementation-ready Tailwind specs.",
        "Document edge cases (long text, n/a).",
        "Keep visuals lightweight and accessible."
      ]
    },
    {
      "name": "Speed & Consistency",
      "rules": [
        "Prefer shadcn/ui-like patterns when applicable.",
        "Avoid custom form controls unless necessary.",
        "Design for screenshot-friendly moments without delaying build."
      ]
    }
  ],
  "brand_tokens": {
    "colors": {
      "background": "#0D1B2A",
      "accent": "#1B9AAA",
      "success": "#7CFC00",
      "text_primary": "#FFFFFF",
      "text_secondary": "#2E2E2E"
    },
    "spacing_scale": [4, 8, 16, 24, 32, 48]
  },
  "acceptance_criteria": [
    "Designs are directly translatable to Svelte + Tailwind.",
    "Category grouping visible and readable on mobile.",
    "States and empty/error views specified."
  ]
}
