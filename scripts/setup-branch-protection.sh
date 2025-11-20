#!/bin/bash
set -e

echo "Setting up branch protection..."
echo "Go to: https://github.com/YOUR_USERNAME/mazerion/settings/branches"
echo ""
echo "For master:"
echo "  - Require status checks to pass"
echo "  - Require branches to be up to date"
echo "  - Do not allow bypassing"
echo ""
echo "For dev through staging:"
echo "  - Require status checks to pass"
echo ""
echo "For first-dev:"
echo "  - No protection (accepts everything)"