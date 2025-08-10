# Local values for resource naming and tagging
locals {
  name_prefix = "${var.project_name}-${var.environment}"
  
  common_tags = {
    Project     = var.project_name
    Environment = var.environment
    ManagedBy   = "terraform"
    Owner       = "platform-team"
    CostCenter  = "engineering"
  }
  
  # Consciousness-specific tags
  consciousness_tags = merge(local.common_tags, {
    Component = "consciousness-engine"
    Tier      = "core"
  })
}

# Data sources
data "aws_caller_identity" "current" {}
data "aws_region" "current" {}

# VPC and Networking
module "vpc" {
  source = "./modules/networking"
  
  name_prefix        = local.name_prefix
  vpc_cidr          = var.vpc_cidr
  availability_zones = var.availability_zones
  
  # Enable DNS hostnames and resolution for EKS
  enable_dns_hostnames = true
  enable_dns_support   = true
  
  # Enable VPC Flow Logs for security monitoring
  enable_flow_log = true
  flow_log_destination_type = "cloud-watch-logs"
  
  tags = local.common_tags
}

# EKS Cluster
module "eks" {
  source = "./modules/kubernetes"
  
  cluster_name    = "${local.name_prefix}-cluster"
  cluster_version = var.cluster_version
  
  vpc_id          = module.vpc.vpc_id
  subnet_ids      = module.vpc.private_subnet_ids
  
  # Enable cluster logging for security and debugging
  cluster_enabled_log_types = [
    "api", "audit", "authenticator", "controllerManager", "scheduler"
  ]
  
  # Node groups configuration
  node_groups = var.node_groups
  
  # Enable IRSA (IAM Roles for Service Accounts)
  enable_irsa = true
  
  # Security group rules
  cluster_security_group_additional_rules = {
    ingress_consciousness_api = {
      description = "Consciousness API access"
      protocol    = "tcp"
      from_port   = 8080
      to_port     = 8080
      type        = "ingress"
      cidr_blocks = [var.vpc_cidr]
    }
  }
  
  tags = local.consciousness_tags
}

# RDS PostgreSQL for primary data storage
module "rds" {
  source = "./modules/storage"
  
  identifier = "${local.name_prefix}-postgres"
  
  engine         = "postgres"
  engine_version = var.rds_config.engine_version
  instance_class = var.rds_config.instance_class
  
  allocated_storage     = var.rds_config.allocated_storage
  max_allocated_storage = var.rds_config.max_allocated_storage
  
  db_name  = "consciousness"
  username = "consciousness_admin"
  
  vpc_id     = module.vpc.vpc_id
  subnet_ids = module.vpc.database_subnet_ids
  
  backup_retention_period = var.rds_config.backup_retention_period
  backup_window          = var.rds_config.backup_window
  maintenance_window     = var.rds_config.maintenance_window
  
  multi_az            = var.rds_config.multi_az
  deletion_protection = var.rds_config.deletion_protection
  
  # Enable encryption at rest
  storage_encrypted = true
  kms_key_id       = aws_kms_key.rds.arn
  
  # Enable performance insights
  performance_insights_enabled = true
  
  # Enable enhanced monitoring
  monitoring_interval = 60
  monitoring_role_arn = aws_iam_role.rds_enhanced_monitoring.arn
  
  tags = local.consciousness_tags
}

# Redis for caching and session storage
module "redis" {
  source = "./modules/cache"
  
  cluster_id = "${local.name_prefix}-redis"
  
  node_type          = var.redis_config.node_type
  num_cache_clusters = var.redis_config.num_cache_clusters
  parameter_group_name = var.redis_config.parameter_group_name
  port              = var.redis_config.port
  
  subnet_group_name = aws_elasticache_subnet_group.redis.name
  security_group_ids = [aws_security_group.redis.id]
  
  at_rest_encryption_enabled = var.redis_config.at_rest_encryption_enabled
  transit_encryption_enabled = var.redis_config.transit_encryption_enabled
  
  # Enable automatic failover for HA
  automatic_failover_enabled = true
  
  tags = local.consciousness_tags
}

# S3 buckets for object storage
resource "aws_s3_bucket" "consciousness_data" {
  bucket = "${local.name_prefix}-consciousness-data"
  
  tags = local.consciousness_tags
}

resource "aws_s3_bucket_versioning" "consciousness_data" {
  bucket = aws_s3_bucket.consciousness_data.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_encryption" "consciousness_data" {
  bucket = aws_s3_bucket.consciousness_data.id
  
  server_side_encryption_configuration {
    rule {
      apply_server_side_encryption_by_default {
        kms_master_key_id = aws_kms_key.s3.arn
        sse_algorithm     = "aws:kms"
      }
    }
  }
}

# KMS keys for encryption
resource "aws_kms_key" "rds" {
  description             = "KMS key for RDS encryption"
  deletion_window_in_days = 7
  
  tags = merge(local.consciousness_tags, {
    Name = "${local.name_prefix}-rds-key"
  })
}

resource "aws_kms_alias" "rds" {
  name          = "alias/${local.name_prefix}-rds"
  target_key_id = aws_kms_key.rds.key_id
}

resource "aws_kms_key" "s3" {
  description             = "KMS key for S3 encryption"
  deletion_window_in_days = 7
  
  tags = merge(local.consciousness_tags, {
    Name = "${local.name_prefix}-s3-key"
  })
}

resource "aws_kms_alias" "s3" {
  name          = "alias/${local.name_prefix}-s3"
  target_key_id = aws_kms_key.s3.key_id
}

# IAM role for RDS enhanced monitoring
resource "aws_iam_role" "rds_enhanced_monitoring" {
  name = "${local.name_prefix}-rds-monitoring-role"
  
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "monitoring.rds.amazonaws.com"
        }
      }
    ]
  })
  
  tags = local.consciousness_tags
}

resource "aws_iam_role_policy_attachment" "rds_enhanced_monitoring" {
  role       = aws_iam_role.rds_enhanced_monitoring.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonRDSEnhancedMonitoringRole"
}

# Security groups
resource "aws_security_group" "redis" {
  name_prefix = "${local.name_prefix}-redis-"
  vpc_id      = module.vpc.vpc_id
  
  ingress {
    from_port   = var.redis_config.port
    to_port     = var.redis_config.port
    protocol    = "tcp"
    cidr_blocks = [var.vpc_cidr]
  }
  
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  tags = merge(local.consciousness_tags, {
    Name = "${local.name_prefix}-redis-sg"
  })
}

# ElastiCache subnet group
resource "aws_elasticache_subnet_group" "redis" {
  name       = "${local.name_prefix}-redis-subnet-group"
  subnet_ids = module.vpc.private_subnet_ids
  
  tags = local.consciousness_tags
}