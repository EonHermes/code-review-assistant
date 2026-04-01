#!/bin/bash
# Recreate the Smart Homelab Dashboard project structure
set -e

mkdir -p smart-homelab-dashboard/backend/{src/{routes,config,models,database},migrations}
mkdir -p smart-homelab-dashboard/frontend/{src/{api,types,test,components}}

echo "Directory structure created. Now writing files..."