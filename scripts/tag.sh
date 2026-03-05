#!/usr/bin/env bash
set -euo pipefail


git fetch --tags

LAST_BASE_TAG=$(git tag --list | grep -E "^[0-9]+.[0-9]+.[0-9]+$" | sort -V | tail -n1)

IFS='.' read -r MAJOR MINOR PATCH <<< "$LAST_BASE_TAG"

echo "Last base tag: ${LAST_BASE_TAG}"
echo "Current base version: $MAJOR.$MINOR.$PATCH"

COMMITS=$(git log "${LAST_BASE_TAG}..HEAD" --oneline)

echo "Commits since last base tag:"
echo "$COMMITS"

BUMP="patch"
if echo "$COMMITS" | grep -q "BREAKING CHANGE"; then
  BUMP="major"
elif echo "$COMMITS" | grep -q "feat:"; then
  BUMP="minor"
fi

echo "Determined bump: $BUMP"

case "$BUMP" in
  major)
    MAJOR=$((MAJOR+1))
    MINOR=0
    PATCH=0
    ;;
  minor)
    MINOR=$((MINOR+1))
    PATCH=0
    ;;
  patch)
    PATCH=$((PATCH+1))
    ;;
esac

NEXT_BASE_VERSION="${MAJOR}.${MINOR}.${PATCH}"
echo "Next base version: $NEXT_BASE_VERSION"

if [[ "$CI_BRANCH" == "prod" ]]; then
  NEW_TAG="$NEXT_BASE_VERSION"
else
  LAST_PRE_TAG=$(git tag --list "${NEXT_BASE_VERSION}-main.*" --sort=-v:refname | head -n1)
  if [[ -z "$LAST_PRE_TAG" ]]; then
    PRE_RELEASE_NUM=0
  else
    PRE_RELEASE_NUM=$(echo "$LAST_PRE_TAG" | sed -E "s/^${NEXT_BASE_VERSION}-main\.([0-9]+)/\1/")
    PRE_RELEASE_NUM=$((PRE_RELEASE_NUM + 1))
  fi
  NEW_TAG="${NEXT_BASE_VERSION}-main.${PRE_RELEASE_NUM}"
fi

echo "Next tag: $NEW_TAG"

git tag "$NEW_TAG"
git push origin "$NEW_TAG"

echo "VERSION=$NEW_TAG" >> $GITHUB_ENV
echo "version=$NEW_TAG" >> $GITHUB_OUTPUT