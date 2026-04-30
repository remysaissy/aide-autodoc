#!/usr/bin/env bash

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

show_help() {
    cat << 'EOF'
Usage: ./release.sh [OPTION]

Perform a full release from the current -dev version in Cargo.toml:
strips the -dev suffix, runs tests, commits (signed), tags, pushes,
publishes to crates.io, creates a GitHub release, then bumps to the
next patch -dev version and pushes to main.

Use ./bump-version.sh --minor/--major/--revision first if you need to
change the planned release target before releasing.

Options:
    --dry-run     Run all steps except push, publish, and GitHub release
    --help        Display this help message

Requirements:
    - git-cliff   (cargo install git-cliff)
    - gh           (GitHub CLI, brew install gh)
    - cargo        (Rust toolchain)
    - GPG key configured for signed commits (git commit -S)
    - CARGO_REGISTRY_TOKEN env var or cargo login (for crates.io publishing)

EOF
}

print_info()    { echo -e "${GREEN}[INFO]${NC} $1"; }
print_error()   { echo -e "${RED}[ERROR]${NC} $1" >&2; }
print_warning() { echo -e "${YELLOW}[WARN]${NC} $1"; }
print_step()    { echo -e "${BLUE}[STEP]${NC} $1"; }

check_dependencies() {
    local missing=0
    for cmd in git-cliff gh cargo; do
        if ! command -v "$cmd" &> /dev/null; then
            print_error "$cmd is not installed."
            missing=1
        fi
    done
    if [ "$missing" -eq 1 ]; then
        exit 1
    fi
}

check_clean_workdir() {
    if [ -n "$(git status --porcelain)" ]; then
        print_error "Working directory is not clean. Commit or stash your changes first."
        exit 1
    fi
}

check_on_main() {
    local branch
    branch=$(git rev-parse --abbrev-ref HEAD)
    if [ "$branch" != "main" ]; then
        print_error "Releases must be made from the 'main' branch (currently on '$branch')."
        exit 1
    fi
}

get_current_version() {
    grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
}

strip_dev() {
    echo "$1" | sed 's/-dev$//'
}

bump_patch() {
    local version=$1
    local major minor patch
    read -r major minor patch <<< "$(echo "$version" | sed 's/\./ /g')"
    patch=$((patch + 1))
    echo "$major.$minor.$patch"
}

update_cargo_version() {
    local new_version=$1
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    else
        sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    fi
}

generate_changelog() {
    local release_version=$1
    git-cliff --unreleased --tag "v$release_version" -o CHANGELOG.md
}

run_tests() {
    print_step "Running tests before release..."
    cargo test --workspace
}

publish_crate() {
    local crate_name=$1
    local dry_run=$2

    print_step "Publishing $crate_name to crates.io..."

    if [ "$dry_run" = "true" ]; then
        cargo publish --dry-run -p "$crate_name"
    else
        cargo publish -p "$crate_name"

        print_info "Waiting for crates.io to index $crate_name..."
        sleep 30
    fi
}

main() {
    if [ $# -eq 0 ]; then
        show_help
        exit 0
    fi

    local dry_run="false"

    while [ $# -gt 0 ]; do
        case "$1" in
            --dry-run)  dry_run="true" ;;
            --help|-h)  show_help; exit 0 ;;
            *)          print_error "Unknown option: $1"; show_help; exit 1 ;;
        esac
        shift
    done

    check_dependencies
    check_clean_workdir
    check_on_main

    local current_version release_version next_dev_version
    current_version=$(get_current_version)

    if [[ "$current_version" != *-dev ]]; then
        print_error "Current version '$current_version' does not have a -dev suffix."
        print_error "Run ./bump-version.sh first to set the planned release version."
        exit 1
    fi

    release_version=$(strip_dev "$current_version")
    next_dev_version="$(bump_patch "$release_version")-dev"

    echo ""
    print_info "Current version:    $current_version"
    print_info "Release version:    $release_version"
    print_info "Next dev version:   $next_dev_version"
    if [ "$dry_run" = "true" ]; then
        print_warning "DRY RUN — no push, publish, or GitHub release will be performed"
    fi
    echo ""

    read -p "Proceed with release v${release_version}? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Release cancelled."
        exit 0
    fi

    echo ""

    print_step "1/8 Running tests..."
    run_tests
    echo ""

    print_step "2/8 Setting release version $release_version..."
    update_cargo_version "$release_version"
    print_info "Updated Cargo.toml"

    print_step "3/8 Generating changelog..."
    generate_changelog "$release_version"
    print_info "Updated CHANGELOG.md"

    print_step "4/8 Creating signed release commit and tag..."
    git add Cargo.toml CHANGELOG.md
    git commit -S -m "chore(release): prepare for v$release_version"
    git tag -a "v$release_version" -m "Release v$release_version"
    print_info "Created signed commit and tag v$release_version"

    if [ "$dry_run" = "true" ]; then
        echo ""
        print_step "5/8 [DRY RUN] Skipping push"
        print_step "6/8 [DRY RUN] Validating crate packages..."
        publish_crate "aide-autodoc" "true"
        print_step "7/8 [DRY RUN] Skipping GitHub release"
        print_step "8/8 [DRY RUN] Skipping post-release dev bump"
        echo ""
        print_info "Dry run complete. To finalize:"
        print_info "  git push && git push --tags"
        print_info "  cargo publish -p aide-autodoc"
        print_info "  gh release create v$release_version --generate-notes"
        print_info "  (then bump to $next_dev_version)"
        return
    fi

    print_step "5/8 Pushing release to remote..."
    git push
    git push --tags
    print_info "Pushed commit and tag"

    print_step "6/8 Publishing crates..."
    publish_crate "aide-autodoc" "false"
    print_info "Crate published"

    print_step "7/8 Creating GitHub release..."
    gh release create "v$release_version" \
        --title "v$release_version" \
        --notes-file CHANGELOG.md
    print_info "GitHub release created"

    print_step "8/8 Bumping to next dev version $next_dev_version..."
    update_cargo_version "$next_dev_version"
    git add Cargo.toml
    git commit -S -m "chore: begin $next_dev_version development cycle"
    git push
    print_info "Main branch is now at $next_dev_version"

    echo ""
    print_info "Release v$release_version complete!"
    print_info ""
    print_info "The release workflow will now run on the pushed tag to:"
    print_info "  - Verify the build"
    print_info "  - Generate build attestations (gh attestation)"
    print_info "  - Link the release to the provenance chain"
}

main "$@"
