# Agents Context

This file is read automatically by Jules and other AI coding agents working in this repository.
Do not delete it. Update the config section when deployment targets change.

---

## Repository config

```
HERMES_URL=http://host.docker.internal:8088
HF_PROFILE=your-hf-username
HF_SPACE=your-space-name
RENDER_URL=https://your-app.onrender.com
```

---

## Deployment targets

| Target | Platform | URL |
|--------|----------|-----|
| Backend | HuggingFace Spaces | `https://{{HF_PROFILE}}-{{HF_SPACE}}.hf.space` |
| Frontend | Render | `{{RENDER_URL}}` |

**Do not push to main directly for new features.** Always open a PR from a feature branch.

Fix branches (`fix/hf-deploy-*`, `fix/render-deploy-*`, `fix/integration-*`) are auto-merged by CI when tests pass.

---

## CI/CD pipeline

Handled by GitHub Actions in `.github/workflows/`:

| Workflow | Trigger | Action |
|----------|---------|--------|
| `deploy-hf.yml` | push to `main` (backend paths) | Deploys `backend/` to HF Space |
| `deploy-render.yml` | push to `main` (frontend paths) | Triggers Render deploy |
| `auto-merge-jules.yml` | PR opened on Jules branches | Auto-merges when CI passes |

If a deploy fails, GitHub Actions notifies hermes automatically. **Jules is expected to pick up the failure** within 15 minutes via Jules' built-in CI detector. If 15 minutes pass with no activity, hermes will open a new session asking Jules to investigate.

---

## How to fix a deployment failure

### HuggingFace Space failure

1. Read the Space logs:
   ```bash
   curl -H "Authorization: Bearer $HF_TOKEN" \
     "https://huggingface.co/api/spaces/$HF_PROFILE/$HF_SPACE/logs/run"
   ```
2. Identify the cause: missing package, wrong port (must be 7860), Dockerfile issue
3. Create a fix branch: `fix/hf-deploy-$(date +%Y%m%d-%H%M)`
4. Fix and push — do NOT push to main
5. Open a PR with a clear description
6. CI auto-merges when tests pass → deploy re-triggers automatically

### Render failure

1. Check the deploy log via Render API:
   ```bash
   curl "https://api.render.com/v1/services/$RENDER_SERVICE_ID/deploys?limit=1" \
     -H "Authorization: Bearer $RENDER_API_KEY"
   ```
2. Common issues: wrong build command, wrong publish directory, missing env vars
3. Fix on branch `fix/render-deploy-$(date +%Y%m%d-%H%M)`
4. Open a PR — auto-merges when CI passes

---

## Reporting back to hermes

When you complete a task (deploy fix, component implementation, test pass), report to hermes:

```bash
# Advance the active pipeline phase
curl -X POST $HERMES_URL/api/pipelines/$PIPELINE_ID/advance \
  -H "Content-Type: application/json" \
  -d '{
    "phase_id": "deployment",
    "status": "done",
    "notes": "Describe what was fixed or completed"
  }'
```

The `PIPELINE_ID` for the current work is written to `.hermes/pipeline.json` if present.

---

## Component development protocol

When implementing a component assigned to you:

1. Read `.hermes/plan.json` for your component's spec and required tests
2. Create branch: `feat/component-{component-id}`
3. Write tests FIRST (TDD), then implement
4. All tests must pass before opening a PR
5. Report back with the advance endpoint above using `phase_id: "spec-coding"`

---

## Do not

- Push directly to `main` (except integration fixes)
- Create multiple PRs for one task
- Use `--no-verify` or skip CI checks
- Delete `.hermes/` files
- Change this file's structure without updating it consistently
