# SIGILED-SUPERVISOR — Log operativo

**Contratto.** Questo file è la memoria operativa del progetto. Ogni sessione che chiude lavoro coerente **aggiunge una voce in cima** (la più recente prima) e **aggiorna lo Stato attuale**. Ogni voce risponde a tre domande: **dove eravamo**, **dove prevedevamo di andare**, **cosa è stato fatto** — più gli **scarti**, lo **stato a fine sessione** e il **prossimo passo previsto**. Le voci non si cancellano: si correggono con voci nuove.

**Nota (SIGILED DEC-04):** file project-owned; il layer macchina della storia sarà esposto da SIGILED via API, senza scrivere nei repo.

---

## Stato attuale

_aggiornato: 2026-08-02, sessione 13568118 (nasce il supervisor — codice v0.1)_

- **Su master**: `docs/requisiti.md` v0.2, questo log, e **il supervisor**: `supervisor/` (crate Rust, un file `src/main.rs` da 147 righe, axum+tokio+serde_json e basta) + `tools/dev-toolchain.sh` (portato da sigiled per le sessioni su questo repo).
- **API conformi ai requisiti §2**: `GET /health` (no auth), `GET /sigiled/status` `{deployed_sha, healthy, last_restart}` e `POST /sigiled/restart {sha?}` (bearer statico `SUPERVISOR_TOKEN`, confronto constant-time; sha esplicito = rollback DEC-04; 409 su restart concorrente; report `{previous_sha, new_sha, healthy, duration_secs, log_tail}`); log append-only locale (`SUPERVISOR_LOG`). Config: `SIGILED_REPO_DIR`, `SUPERVISOR_RESTART_CMD` (default `docker compose up -d --build sigiled`, sostituzione `{sha}`), `SIGILED_HEALTH_URL`.
- **Verificato in run locale** (il workspace non ha docker: repo finto + restart_cmd echo): health 200, 401/200 su auth, 409 sul secondo restart, report con `healthy:true` in 5s, log scritto. Bug reale trovato e fixato: i comandi bloccanti dentro gli handler async passavano sul worker tokio — su un box a 1 CPU il poll di health verso se stessi era un deadlock a tempo; ora tutto via `spawn_blocking`.
- **Genesi**: progetto fresco nato dalla rinomina universale — la piattaforma è SIGILED («seal» eradicato ovunque per ordine del Re). Il predecessore `seal-supervisor` (requisiti v0.1, DEC-01…06) è archiviato come fondazione; la sostanza delle decisioni è ereditata qui invariata.
- **Decisioni**: DEC-01…07 registrate; DEC-06 (Rust) e DEC-07 (nome) ratificate; DEC-01…05 da ratificare. Ratifica madre: DEC-13 di `sigiled`.
- **Prossimo passo previsto**: ratifica DEC-01…05; poi prima sessione di codice in **Rust** e vhost edge `supervisor.016180.xyz` (azione operatore, §6.3 dei requisiti).

---

## Voci

### 2026-08-02 · nasce il supervisor: un file, un dovere — driver: Claude Code (sessione 13568118, dentro la sessione 4/4 di sigiled)

- **Dove eravamo**: requisiti v0.2 su master, nessun codice.
- **Previsione**: il servizio ~100 righe dei requisiti (DEC-01..07), sviluppato in sessione normale, deploy operatore-side.
- **Fatto**: `supervisor/src/main.rs` (147 righe con commenti — nello spirito di DEC-01): le tre API di §2 esatte, bearer statico constant-time (DEC-03), rollback via sha (DEC-04), 409 su restart concorrente, log append-only, `hc_ping` rimandato (opzionale nei requisiti). Toolchain di sviluppo copiata da sigiled. Smoke run locale completo con repo finto e comando di restart mockato via `SUPERVISOR_RESTART_CMD` — env che serve anche ai deploy non-compose. Fix da campo: comandi bloccanti fuori dai worker tokio (`spawn_blocking`), altrimenti su 1 CPU il poll di health verso il proprio processo non risponde mai.
- **Scarti**: (a) `hc_ping` su completamento non implementato — opzionale, arriva se serve; (b) niente unit test nel file (la brevità è un requisito): la verifica è lo smoke run, ripetibile con le env del log di questa voce; (c) il deploy reale (unit systemd/compose sul box, vhost `supervisor.016180.xyz`) resta azione operatore — istruzioni nel runbook di sigiled (`docs/runbook-deploy.md`).
- **Stato a fine sessione**: vedi «Stato attuale» sopra.
- **Prossimo passo previsto**: deploy sul box (operatore, runbook sigiled); ratifica DEC-01..05; questione aperta §6.2 (esposizione vhost) al Re.

### 2026-08-03 · genesi: rinomina universale, eredità dal predecessore — driver: Kimi K3

- **Dove eravamo**: la piattaforma si è rinominata SIGILED; il Re ha ordinato l'eradicazione totale della stringa precedente, supervisor compreso.
- **Previsione**: progetto fresco con i requisiti sweepati.
- **Fatto**: progetto MGR `sigiled-supervisor` creato (repo `ivan-saorin/sigiled-supervisor`); `docs/requisiti.md` v0.2 — identico nella sostanza alla v0.1 ereditata (sintesi, vincoli ~100 righe, mai `[app]`, zero dipendenze, bearer statico; API `/health`, `/sigiled/status`, `/sigiled/restart` con sha = rollback; env `SIGILED_*`), più DEC-07 (nome, ratificata). Vecchio progetto archiviato con voce conclusiva sul suo log.
- **Scarti**: nessuno — travaso pulito.
- **Stato a fine sessione**: vedi «Stato attuale» sopra.
- **Prossimo passo previsto**: ratifica DEC-01…05; poi prima sessione di codice.
