# .agent Framework Manifest

Version: 1.0.0
Last Updated: 2026-01-23
Status: Draft

Purpose
-------
The .agent directory is the single source of truth for project context. Follow the reading order below at the start of every session.

Reading Order (mandatory)
1) 00_manifest.md
2) 10_spec.md
3) 21_api_contract.md
4) 20_architecture.md
5) 22_standards.md
6) 30_adrs/ (latest ADRs)
7) 35_research.md
8) 40_tasks.md
9) 45_implementation_plan.md
10) 50_worklog.md (last 50-100 lines)
11) 60_open_questions.md

File ownership (suggested)
- Human controlled: 10_spec.md, 21_api_contract.md, 22_standards.md, 30_adrs/* after approval.
- AI writable (with review): 35_research.md, 40_tasks.md, 45_implementation_plan.md, 50_worklog.md, 60_open_questions.md.

Context budget
- Keep total context under 40% of your model budget. Archive older sections into .agent/archive when needed.
- Prefer concise, factual entries; avoid duplication.

Workflow
- Research -> Plan -> Implement. Log each session in 50_worklog.md.
- Capture open questions that need human answers in 60_open_questions.md.
