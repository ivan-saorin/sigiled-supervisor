# SIGILED-SUPERVISOR — Documento dei Requisiti

**Versione:** 0.2 · **Data:** 2026-08-03 · **Stato:** proposta, da ratificare
**Origine:** sessione di design MGR v2 del 2026-08-02; ratificata dal Re come DEC-13 in `sigiled` (`docs/mgr-v2.md` §7). v0.2 = sweep di rinomina (la piattaforma è SIGILED) sul testo v0.1 ereditato dal repo archiviato `seal-supervisor`.
**Memoria operativa:** `docs/log-operativo.md`.

---

## 0. Sintesi

> **SIGILED gestisce tutto di sé tranne la propria resurrezione. La resurrezione è questo servizio.**

Supervisor esterno minimale: **~100 righe** (se cresce, sta sbagliando). Repo e deploy propri. Espone una sua API: **chiamarla = restart di sigiled** (pull allo sha pinnato → build → restart → health check → report). È l'out-of-band dell'operatore reso API — raggiungibile dai driver LLM anche quando sigiled è morto, che è esattamente quando serve.

## 1. Vincoli

- **Dimensione**: ~100 righe. Un solo file è la forma giusta.
- **Deploy indipendente da SIGILED**: systemd unit o compose sul box. **Mai `[app]` di MGR** — è nel percorso di resurrezione, non può dipendere da ciò che resuscita.
- **Zero dipendenze runtime dallo stack**: niente SIGILED, niente Authentik, niente MGR, niente DB esterno. Deve funzionare a stack mezzo morto.
- **Auth propria e semplice**: bearer statico in env sul box (mai in git). Non OIDC: l'IdP potrebbe essere giù insieme al resto.
- **Segreti**: chiavi in env del box, mai nel repo (regola 8).

## 2. API

Base proposta: `https://supervisor.016180.xyz` (vhost edge da aggiungere — azione operatore sullo stack).

| Endpoint | Auth | Contratto |
|---|---|---|
| `GET /health` | no | `{status: "ok"}` — liveness del supervisor |
| `GET /sigiled/status` | sì | `{deployed_sha, container_state, last_restart: {ts, sha, result}, healthy}` |
| `POST /sigiled/restart` | sì | body opzionale `{sha}` — default: sha attualmente pinnato; sha esplicito = rollback. Esegue pull → build → restart → attesa health → risponde `{previous_sha, new_sha, healthy, duration_secs, log_tail}` |

Timeout e idempotenza: un restart in corso rifiuta il secondo (`409`). Il report include il tail del log di build/restart per diagnosi immediata in chat.

## 3. Osservabilità

- Log **append-only locale** di ogni chiamata: chi (bearer id), quando, sha, esito, durata.
- `hc_ping` opzionale su completamento restart (stesso pattern dei job MGR).
- Il log locale è la memoria del supervisor: semplice, sul box, leggibile via SSH anche a tutto morto.

## 4. Sviluppo e deploy

- Il **codice** si sviluppa via sessioni MGR normali su questo repo (quando sigiled è vivo). Sessioni su `sigiled-supervisor` richiedono **approval valida** (sigiled DEC-15).
- Il **deploy** è operatore-side: pull manuale + restart della unit systemd. Auto-aggiornarsi è fuori questione come per sigiled — il supervisor non tocca se stesso.
- Config in env: `SIGILED_REPO_DIR`, `SIGILED_COMPOSE_SERVICE`, `SUPERVISOR_TOKEN`, `HC_PING_URL` (opzionale).

## 5. Out of scope (per sempre, salvo decreto del Re)

- Gestione di altri servizi dello stack (caddy, authentik, …). Il supervisor resuscita **solo sigiled**: un solo dovere.
- UI, webhook, scheduling, metriche: no. `GET /sigiled/status` basta.
- Auto-remediation (restart automatico se sigiled non risponde): **no in v1** — un loop automatico che restarta il control plane è un modo elegante di mascherare i bug. Il Re decide quando restartare, direttamente o via driver.

## 6. Questioni aperte

1. ~~Linguaggio: Go vs Python+stdlib~~ **Risolta 2026-08-02: Rust** — il Re ha deciso Rust per tutto il control plane (sigiled DEC-16).
2. Esposizione: vhost pubblico `supervisor.016180.xyz` vs solo rete interna + SSH tunnel. Candidato: esposto con auth — i driver devono poterlo chiamare a sigiled morto.
3. Il vhost edge richiede caddy configurato a mano sullo stack (azione operatore).

## 7. Registro delle decisioni

| # | Decisione |
|---|---|
| DEC-01 | Un solo dovere: restart di sigiled. ~100 righe, un file. |
| DEC-02 | Deploy indipendente, mai `[app]` di MGR, zero dipendenze runtime dallo stack. |
| DEC-03 | Auth = bearer statico in env; niente OIDC nel percorso di resurrezione. |
| DEC-04 | Restart con sha opzionale = meccanismo di rollback incorporato. |
| DEC-05 | Niente auto-remediation in v1: restart solo su chiamata esplicita. |
| DEC-06 | Linguaggio: **Rust**, come tutto il control plane (sigiled DEC-16). **Ratificata 2026-08-02.** |
| DEC-07 | Nome: **sigiled-supervisor**, progetto fresco (2026-08-03); il predecessore `seal-supervisor` è archiviato come fondazione. **Ratificata 2026-08-03.** |
