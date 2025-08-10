# Global Variables
variable "environment" {
  description = "Environment name (dev, staging, prod)"
  type        = string
  validation {
    condition     = contains(["dev", "staging", "prod", "dr"], var.environment)
    error_message = "Environment must be one of: dev, staging, prod, dr."
  }
}

variable "aws_region" {
  description = "AWS region for resources"
  type        = string
  default     = "us-west-2"
}

variable "project_name" {
  description = "Project name for resource naming"
  type        = string
  default     = "consciousness-engine"
}

# Networking Variables
variable "vpc_cidr" {
  description = "CIDR block for VPC"
  type        = string
  default     = "10.0.0.0/16"
}

variable "availability_zones" {
  description = "List of availability zones"
  type        = list(string)
  default     = ["us-west-2a", "us-west-2b", "us-west-2c"]
}

# EKS Variables
variable "cluster_version" {
  description = "Kubernetes cluster version"
  type        = string
  default     = "1.28"
}

variable "node_groups" {
  description = "EKS node group configurations"
  type = map(object({
    instance_types = list(string)
    capacity_type  = string
    min_size      = number
    max_size      = number
    desired_size  = number
    disk_size     = number
    labels        = map(string)
    taints        = list(object({
      key    = string
      value  = string
      effect = string
    }))
  }))
  default = {
    consciousness = {
      instance_types = ["m6i.xlarge", "m6i.2xlarge"]
      capacity_type  = "ON_DEMAND"
      min_size      = 2
      max_size      = 20
      desired_size  = 3
      disk_size     = 100
      labels = {
        workload = "consciousness"
        tier     = "compute"
      }
      taints = []
    }
    analytics = {
      instance_types = ["r6i.large", "r6i.xlarge"]
      capacity_type  = "SPOT"
      min_size      = 1
      max_size      = 10
      desired_size  = 2
      disk_size     = 200
      labels = {
        workload = "analytics"
        tier     = "memory"
      }
      taints = []
    }
    system = {
      instance_types = ["t3.medium", "t3.large"]
      capacity_type  = "ON_DEMAND"
      min_size      = 1
      max_size      = 5
      desired_size  = 2
      disk_size     = 50
      labels = {
        workload = "system"
        tier     = "general"
      }
      taints = []
    }
  }
}

# Database Variables
variable "rds_config" {
  description = "RDS configuration"
  type = object({
    engine_version    = string
    instance_class    = string
    allocated_storage = number
    max_allocated_storage = number
    backup_retention_period = number
    backup_window     = string
    maintenance_window = string
    multi_az          = bool
    deletion_protection = bool
  })
  default = {
    engine_version    = "15.4"
    instance_class    = "db.r6g.xlarge"
    allocated_storage = 100
    max_allocated_storage = 1000
    backup_retention_period = 30
    backup_window     = "03:00-04:00"
    maintenance_window = "sun:04:00-sun:05:00"
    multi_az          = true
    deletion_protection = true
  }
}

# Redis Variables
variable "redis_config" {
  description = "Redis cluster configuration"
  type = object({
    node_type           = string
    num_cache_clusters  = number
    parameter_group_name = string
    port               = number
    at_rest_encryption_enabled = bool
    transit_encryption_enabled = bool
  })
  default = {
    node_type           = "cache.r7g.large"
    num_cache_clusters  = 3
    parameter_group_name = "default.redis7"
    port               = 6379
    at_rest_encryption_enabled = true
    transit_encryption_enabled = true
  }
}

# Monitoring Variables
variable "monitoring_config" {
  description = "Monitoring and observability configuration"
  type = object({
    enable_prometheus = bool
    enable_grafana   = bool
    enable_jaeger    = bool
    enable_elk       = bool
    retention_days   = number
  })
  default = {
    enable_prometheus = true
    enable_grafana   = true
    enable_jaeger    = true
    enable_elk       = true
    retention_days   = 90
  }
}

# Security Variables
variable "security_config" {
  description = "Security configuration"
  type = object({
    enable_vault        = bool
    enable_falco        = bool
    enable_opa          = bool
    enable_istio        = bool
    vault_ha           = bool
  })
  default = {
    enable_vault        = true
    enable_falco        = true
    enable_opa          = true
    enable_istio        = true
    vault_ha           = true
  }
}

variable "vault_address" {
  description = "Vault server address"
  type        = string
  default     = ""
}

# Cost Optimization Variables
variable "cost_optimization" {
  description = "Cost optimization settings"
  type = object({
    enable_spot_instances = bool
    enable_autoscaling   = bool
    enable_rightsizing   = bool
    spot_percentage      = number
  })
  default = {
    enable_spot_instances = true
    enable_autoscaling   = true
    enable_rightsizing   = true
    spot_percentage      = 70
  }
}