# Production Backend Configuration
bucket         = "consciousness-engine-terraform-state-prod"
key            = "prod/terraform.tfstate"
region         = "us-west-2"
encrypt        = true
dynamodb_table = "consciousness-engine-terraform-locks"

# Enable versioning and MFA delete for production
versioning = true

# Workspace isolation
workspace_key_prefix = "env"