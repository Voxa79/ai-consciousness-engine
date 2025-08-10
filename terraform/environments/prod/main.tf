# Production Environment Configuration
terraform {
  backend "s3" {
    # Backend configuration will be provided via backend config file
  }
}

# Use the root module
module "consciousness_engine" {
  source = "../../"
  
  # Environment-specific variables
  environment = "prod"
  aws_region  = "us-west-2"
  
  # Production VPC configuration
  vpc_cidr = "10.0.0.0/16"
  availability_zones = [
    "us-west-2a",
    "us-west-2b", 
    "us-west-2c"
  ]
  
  # Production EKS configuration
  cluster_version = "1.28"
  node_groups = {
    consciousness_prod = {
      instance_types = ["m6i.2xlarge", "m6i.4xlarge"]
      capacity_type  = "ON_DEMAND"
      min_size      = 3
      max_size      = 50
      desired_size  = 5
      disk_size     = 200
      labels = {
        workload = "consciousness"
        tier     = "compute"
        env      = "prod"
      }
      taints = []
    }
    analytics_prod = {
      instance_types = ["r6i.xlarge", "r6i.2xlarge"]
      capacity_type  = "SPOT"
      min_size      = 2
      max_size      = 20
      desired_size  = 3
      disk_size     = 500
      labels = {
        workload = "analytics"
        tier     = "memory"
        env      = "prod"
      }
      taints = []
    }
    system_prod = {
      instance_types = ["t3.large", "t3.xlarge"]
      capacity_type  = "ON_DEMAND"
      min_size      = 2
      max_size      = 10
      desired_size  = 3
      disk_size     = 100
      labels = {
        workload = "system"
        tier     = "general"
        env      = "prod"
      }
      taints = []
    }
  }
  
  # Production RDS configuration
  rds_config = {
    engine_version    = "15.4"
    instance_class    = "db.r6g.2xlarge"
    allocated_storage = 500
    max_allocated_storage = 5000
    backup_retention_period = 30
    backup_window     = "03:00-04:00"
    maintenance_window = "sun:04:00-sun:05:00"
    multi_az          = true
    deletion_protection = true
  }
  
  # Production Redis configuration
  redis_config = {
    node_type           = "cache.r7g.xlarge"
    num_cache_clusters  = 3
    parameter_group_name = "default.redis7"
    port               = 6379
    at_rest_encryption_enabled = true
    transit_encryption_enabled = true
  }
  
  # Production monitoring configuration
  monitoring_config = {
    enable_prometheus = true
    enable_grafana   = true
    enable_jaeger    = true
    enable_elk       = true
    retention_days   = 365
  }
  
  # Production security configuration
  security_config = {
    enable_vault        = true
    enable_falco        = true
    enable_opa          = true
    enable_istio        = true
    vault_ha           = true
  }
  
  # Production cost optimization
  cost_optimization = {
    enable_spot_instances = true
    enable_autoscaling   = true
    enable_rightsizing   = true
    spot_percentage      = 60  # Conservative for production
  }
}