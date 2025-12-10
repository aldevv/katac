app: build
	docker run --rm katac sh

build:
	docker build -t katac .

test: build-test
	docker run --rm katac_test

build-test:
	docker build -f Dockerfile.tests -t katac_test .

it: build-test
	docker run -it --rm katac_test sh

format:
	cargo fmt --all

# Get the current version from Cargo.toml
VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
TAG := v$(VERSION)

version:
	@echo "Current version: $(VERSION)"
	@echo "Tag: $(TAG)"

tag:
	@echo "Creating tag $(TAG)..."
	@if git rev-parse $(TAG) >/dev/null 2>&1; then \
		echo "Error: Tag $(TAG) already exists"; \
		exit 1; \
	fi
	git tag -a $(TAG) -m "Release $(TAG)"
	@echo "Tag $(TAG) created. Run 'make release' to push it."

# Create and push a release (triggers GitHub Actions)
release:
	@echo "Preparing release $(TAG)..."
	@if git diff-index --quiet HEAD --; then \
		echo "Working directory is clean"; \
	else \
		echo "Error: You have uncommitted changes. Please commit or stash them first."; \
		exit 1; \
	fi
	@if git rev-parse $(TAG) >/dev/null 2>&1; then \
		echo "Tag $(TAG) already exists locally"; \
	else \
		echo "Creating tag $(TAG)..."; \
		git tag -a $(TAG) -m "Release $(TAG)"; \
	fi
	@echo "Pushing tag $(TAG) to origin..."
	git push origin $(TAG)
	@echo ""
	@echo "✓ Release $(TAG) pushed!"
	@echo "GitHub Actions will now:"
	@echo "  1. Create a GitHub release"
	@echo "  2. Build binaries for multiple platforms"
	@echo "  3. Publish to crates.io"
	@echo ""
	@echo "Check progress at: https://github.com/aldevv/katac/actions"

untag:
	@echo "Deleting tag $(TAG)..."
	@if git rev-parse $(TAG) >/dev/null 2>&1; then \
		git tag -d $(TAG); \
		echo "Local tag $(TAG) deleted"; \
	else \
		echo "Tag $(TAG) does not exist locally"; \
	fi
	@if git ls-remote --tags origin | grep -q "refs/tags/$(TAG)"; then \
		git push origin :refs/tags/$(TAG); \
		echo "Remote tag $(TAG) deleted"; \
	else \
		echo "Tag $(TAG) does not exist on remote"; \
	fi

.PHONY: version tag release untag
