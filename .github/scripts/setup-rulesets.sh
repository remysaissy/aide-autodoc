#!/usr/bin/env bash
set -euo pipefail

REPO="remysaissy/aide-autodoc"

EXISTING=$(gh api "repos/${REPO}/rulesets")

apply_ruleset() {
  local NAME="$1"
  local PAYLOAD="$2"
  local EXISTING_ID
  EXISTING_ID=$(echo "$EXISTING" | jq -r --arg name "$NAME" '.[] | select(.name==$name) | .id')
  if [ -n "$EXISTING_ID" ]; then
    echo "Updating ruleset '$NAME' (id=$EXISTING_ID)..."
    gh api -X PUT "repos/${REPO}/rulesets/${EXISTING_ID}" --input - <<< "$PAYLOAD"
  else
    echo "Creating ruleset '$NAME'..."
    gh api -X POST "repos/${REPO}/rulesets" --input - <<< "$PAYLOAD"
  fi
}

apply_ruleset "Protect main" '{
  "name": "Protect main",
  "target": "branch",
  "enforcement": "active",
  "conditions": {"ref_name": {"include": ["refs/heads/main"], "exclude": []}},
  "rules": [
    {"type": "pull_request", "parameters": {"required_approving_review_count": 0, "dismiss_stale_reviews_on_push": false, "required_reviewers": [], "require_code_owner_review": false, "require_last_push_approval": false, "required_review_thread_resolution": false, "allowed_merge_methods": ["merge","squash","rebase"]}},
    {"type": "required_status_checks", "parameters": {"strict_required_status_checks_policy": true, "do_not_enforce_on_create": false, "required_status_checks": [{"context":"Format"},{"context":"Check"},{"context":"Test Feature Combinations (default)"},{"context":"Code Coverage"},{"context":"Examples"},{"context":"Validate Commit Messages"}]}},
    {"type": "non_fast_forward"},
    {"type": "deletion"}
  ],
  "bypass_actors": []
}'

apply_ruleset "Signed commits everywhere (branches)" '{
  "name": "Signed commits everywhere (branches)",
  "target": "branch",
  "enforcement": "active",
  "conditions": {"ref_name": {"include": ["~ALL"], "exclude": []}},
  "rules": [{"type": "required_signatures"}],
  "bypass_actors": []
}'

apply_ruleset "Signed commits everywhere (tags)" '{
  "name": "Signed commits everywhere (tags)",
  "target": "tag",
  "enforcement": "active",
  "conditions": {"ref_name": {"include": ["~ALL"], "exclude": []}},
  "rules": [{"type": "required_signatures"}],
  "bypass_actors": []
}'

echo "Done. Rulesets configured."
