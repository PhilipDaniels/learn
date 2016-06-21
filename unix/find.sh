#!/bin/bash

# Examples of the find command.

# Name: extension, regex, case insensitive.
# Contents:
# Date:
# Size:
# Type:

# Including directories or files matching.
# Excluding directories or files matching.

# Doing things with the found files.
# Listing, sedding, git blame, compressing.

find . -name obj -prune -o -iname '*.cs' -execdir git blame '{}' \; | grep 'TestMethod'
