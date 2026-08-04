# Project Rules — Always Use These

This file lives at `D:\Vanompp\CLAUDE.md` — it auto-loads every new Claude session in this project.

## Mandatory MCP / Skills (always use, every session)

### 1. graphify — codebase knowledge graph
This project has a knowledge graph at `graphify-out/` with god nodes, community structure, and cross-file relationships.

Rules:
- For codebase questions, first run `graphify query "<question>"` when `graphify-out/graph.json` exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts.
- If `graphify-out/wiki/index.md` exists, use it for broad navigation instead of raw source browsing.
- Read `graphify-out/GRAPH_REPORT.md` only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).
- Always trigger via `/graphify` skill when user asks about codebase structure.

### 2. exa — web search (always use, never use training data for web)
- MCP server `exa` at `https://mcp.exa.ai/mcp` — tools: `mcp__exa__web_search_exa`, `mcp__exa__web_fetch_exa`
- ALWAYS use Exa for any web search, current info, docs lookup, news, people, company search.
- Flow: `web_search_exa` first → pick best URLs → `web_fetch_exa` for full content.
- Never answer web-related questions from memory — search first.

### 3. context7 — library/framework docs (always use for libs)
- MCP server `context7` at `https://mcp.context7.com/mcp`
- Tools: `mcp__context7__resolve-library-id` then `mcp__context7__query-docs`
- ALWAYS use Context7 when user asks about any library, framework, SDK, API, CLI, cloud service — even well-known ones (React, Next.js, Prisma, Express, Tailwind, etc.).
- Includes: API syntax, config, version migration, debugging, setup, CLI usage.
- Do NOT use for: refactoring, business logic, code review, general programming concepts.

Steps:
1. `resolve-library-id` with library name + what to lookup
2. Pick best match by exact name, description relevance, snippet count, benchmark score, reputation
3. `query-docs` with libraryId + single-concept query (split multi-concept questions into separate calls)

### 4. serena — semantic code analysis (always use for code understanding)
- MCP plugin `plugin:serena:serena` — semantic code analysis, LSP-based navigation
- Tools: `find_symbol`, `find_referencing_symbols`, `find_declaration`, `get_symbols_overview`, `search_for_pattern`, `read_file`, `replace_symbol_body`, etc.
- ALWAYS use Serena for:
  - Understanding codebase structure, finding symbols, references, implementations
  - Code navigation before editing (find declaration → check references → edit)
  - Refactoring, renaming, safe deletions
  - Getting high-level overview of files (`get_symbols_overview` first)
- Protocol: onboarding if needed → overview → find symbols → check references → edit safely

### 5. rtk — Rust Token Killer (token optimization, always active)
- **Usage**: Token-optimized CLI proxy — 60-90% savings on dev operations (v0.44.2 installed at `C:\Users\user\.local\bin\rtk`)
- **How it works**: All commands automatically rewritten by Claude Code hook. Example: `git status` → `rtk git status` (transparent, 0 tokens overhead)
- **Meta commands** — always use `rtk` directly:
  ```bash
  rtk gain              # Show token savings analytics
  rtk gain --history    # Show command usage history with savings
  rtk discover          # Analyze Claude Code history for missed opportunities
  rtk proxy <cmd>       # Execute raw command without filtering (debugging)
  rtk --version         # Verify: should show rtk X.Y.Z
  ```
- **Current stats**: 829 commands, 161.8K tokens saved (8.7%), avg 593ms exec
- **Name collision warning**: If `rtk gain` fails, you may have reachingforthejack/rtk (Rust Type Kit) installed instead — verify with `which rtk`
- ALWAYS mention RTK savings when relevant and check `rtk gain` periodically

## Workflow Combination
When working on a task, combine them:
1. **Exa** → search current info if needed
2. **Context7** → fetch latest library docs
3. **Serena** → navigate and understand existing code
4. **Graphify** → query knowledge graph for architecture context
5. Then implement

Never skip MCP usage — they are mandatory for every new session.
