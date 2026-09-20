<img src="assets/logo.jpeg" alt="Logo">

# Terraform Versions Tracker

A CLI tool that scans a Terraform codebase, extracts provider versions declared in every `versions.tf` file, compares them against the latest versions published on the public Terraform Registry, and renders a colored summary table.

## Overview

Managing Terraform stacks spread across many directories makes it easy to lose track of provider versions. This tool walks a root directory, finds every `versions.tf`, and reports:

- Which providers are used
- Which version is pinned locally
- What the latest published version is
- Which directories are affected

## How It Works
1. **Collector** recursively walks the target directory, parses every `versions.tf`, and extracts `source` and `version` for each provider under `terraform → required_providers`.
2. **RegistryClient** queries `https://registry.terraform.io/v1/providers/{namespace}/{type}` for each unique provider source to fetch the latest published version.
3. **ReportBuilder** joins local and remote data and produces a summary.
4. The result is rendered as a table, with outdated providers highlighted.

## Installation
- Just download the binary file from GitHub release page

## Usage
```bash
terraform-versions-tracker --path /path/to/terraform/root
```

## Example Input
A typical `versions.tf`:
```hcl
terraform {
  required_version = ">= 1.14.2"

  required_providers {
    vault = {
      source  = "hashicorp/vault"
      version = "5.1.0"
    }
    local = {
      source  = "hashicorp/local"
      version = "2.5.3"
    }
  }
}
```
