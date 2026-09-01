---
name: devcontainer-environment
description: Load this skill when you need to understand the development container environment, available tools, or base image details.
---

# Devcontainer Environment

To understand your development environment and which tools are available:

1. Read `.devcontainer/Dockerfile` in the project root
2. Identify the base image (FROM line) and read its definition using the link in the comment at the top of `.devcontainer/Dockerfile`
3. The tools and packages installed in that base image, plus any additional installations in the Dockerfile, define your available environment
