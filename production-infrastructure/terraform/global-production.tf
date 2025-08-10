# Global Production Infrastructure for Consciousness Engine
# Expert CTO Next Gen - Worldwide Production Deployment

terraform {
  required_version = ">= 1.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.23"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.11"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }
  
  backend "s3" {
    bucket         = "consciousness-engine-terraform-state"
    key            = "global-production/terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "consciousness-terraform-locks"
  }
}

# Global Variables
variable "environment" {
  description = "Environment name"
  type        = string
  default     = "production"
}

variable "project_name" {
  description = "Project name"
  type        = string
  default     = "consciousness-engine"
}

variable "regions" {
  description = "AWS regions for global deployment"
  type        = list(string)
  default = [
    "us-east-1",      # North America East
    "us-west-2",      # North America West
    "eu-west-1",      # Europe West
    "eu-central-1",   # Europe Central
    "ap-southeast-1", # Asia Pacific Southeast
    "ap-northeast-1", # Asia Pacific Northeast
    "ap-south-1",     # Asia Pacific South
    "sa-east-1"       # South America
  ]
}

variable "availability_zones_per_region" {
  description = "Number of AZs per region"
  type        = number
  default     = 3
}

variable "instance_types" {
  description = "Instance types for different workloads"
  type = object({
    api_gateway     = string
    consciousness   = string
    ml_training     = string
    quantum         = string
    neural          = string
    nano            = string
    space           = string
    database        = string
    cache           = string
  })
  default = {
    api_gateway     = "c6i.2xlarge"
    consciousness   = "m6i.4xlarge"
    ml_training     = "p4d.24xlarge"
    quantum         = "c6i.8xlarge"
    neural          = "r6i.2xlarge"
    nano            = "c6i.xlarge"
    space           = "m6i.2xlarge"
    database        = "r6i.8xlarge"
    cache           = "r6i.4xlarge"
  }
}

# Global Data Sources
data "aws_caller_identity" "current" {}
data "aws_partition" "current" {}

# Global Resources
resource "aws_s3_bucket" "global_assets" {
  bucket = "${var.project_name}-global-assets-${random_id.bucket_suffix.hex}"
  
  tags = {
    Name        = "Global Assets Bucket"
    Environment = var.environment
    Project     = var.project_name
  }
}

resource "aws_s3_bucket_versioning" "global_assets" {
  bucket = aws_s3_bucket.global_assets.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_encryption" "global_assets" {
  bucket = aws_s3_bucket.global_assets.id
  
  server_side_encryption_configuration {
    rule {
      apply_server_side_encryption_by_default {
        sse_algorithm = "AES256"
      }
    }
  }
}

resource "random_id" "bucket_suffix" {
  byte_length = 4
}

# CloudFront Distribution for Global CDN
resource "aws_cloudfront_distribution" "global_cdn" {
  origin {
    domain_name = aws_s3_bucket.global_assets.bucket_regional_domain_name
    origin_id   = "S3-${aws_s3_bucket.global_assets.id}"
    
    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.global_cdn.cloudfront_access_identity_path
    }
  }
  
  enabled             = true
  is_ipv6_enabled     = true
  default_root_object = "index.html"
  
  aliases = [
    "consciousness.yourdomain.com",
    "api.consciousness.yourdomain.com",
    "quantum.consciousness.yourdomain.com",
    "neural.consciousness.yourdomain.com",
    "nano.consciousness.yourdomain.com",
    "space.consciousness.yourdomain.com"
  ]
  
  default_cache_behavior {
    allowed_methods        = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "S3-${aws_s3_bucket.global_assets.id}"
    compress               = true
    viewer_protocol_policy = "redirect-to-https"
    
    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
    
    min_ttl     = 0
    default_ttl = 3600
    max_ttl     = 86400
  }
  
  # API Cache Behavior
  ordered_cache_behavior {
    path_pattern           = "/api/*"
    allowed_methods        = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods         = ["GET", "HEAD", "OPTIONS"]
    target_origin_id       = "S3-${aws_s3_bucket.global_assets.id}"
    compress               = true
    viewer_protocol_policy = "https-only"
    
    forwarded_values {
      query_string = true
      headers      = ["Authorization", "Content-Type"]
      cookies {
        forward = "none"
      }
    }
    
    min_ttl     = 0
    default_ttl = 0
    max_ttl     = 0
  }
  
  price_class = "PriceClass_All"
  
  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }
  
  viewer_certificate {
    acm_certificate_arn      = aws_acm_certificate.global_cert.arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }
  
  tags = {
    Name        = "Global CDN"
    Environment = var.environment
    Project     = var.project_name
  }
}

resource "aws_cloudfront_origin_access_identity" "global_cdn" {
  comment = "Global CDN OAI"
}

# SSL Certificate
resource "aws_acm_certificate" "global_cert" {
  provider          = aws.us_east_1
  domain_name       = "consciousness.yourdomain.com"
  validation_method = "DNS"
  
  subject_alternative_names = [
    "*.consciousness.yourdomain.com"
  ]
  
  lifecycle {
    create_before_destroy = true
  }
  
  tags = {
    Name        = "Global SSL Certificate"
    Environment = var.environment
    Project     = var.project_name
  }
}

# Route53 Hosted Zone
resource "aws_route53_zone" "main" {
  name = "consciousness.yourdomain.com"
  
  tags = {
    Name        = "Main Hosted Zone"
    Environment = var.environment
    Project     = var.project_name
  }
}

# Route53 Records
resource "aws_route53_record" "main" {
  zone_id = aws_route53_zone.main.zone_id
  name    = "consciousness.yourdomain.com"
  type    = "A"
  
  alias {
    name                   = aws_cloudfront_distribution.global_cdn.domain_name
    zone_id                = aws_cloudfront_distribution.global_cdn.hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "api" {
  zone_id = aws_route53_zone.main.zone_id
  name    = "api.consciousness.yourdomain.com"
  type    = "A"
  
  alias {
    name                   = aws_cloudfront_distribution.global_cdn.domain_name
    zone_id                = aws_cloudfront_distribution.global_cdn.hosted_zone_id
    evaluate_target_health = false
  }
}

# Global WAF
resource "aws_wafv2_web_acl" "global_waf" {
  name  = "${var.project_name}-global-waf"
  scope = "CLOUDFRONT"
  
  default_action {
    allow {}
  }
  
  # Rate limiting rule
  rule {
    name     = "RateLimitRule"
    priority = 1
    
    action {
      block {}
    }
    
    statement {
      rate_based_statement {
        limit              = 2000
        aggregate_key_type = "IP"
      }
    }
    
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "RateLimitRule"
      sampled_requests_enabled   = true
    }
  }
  
  # SQL injection protection
  rule {
    name     = "SQLInjectionRule"
    priority = 2
    
    action {
      block {}
    }
    
    statement {
      sqli_match_statement {
        field_to_match {
          body {}
        }
        text_transformation {
          priority = 0
          type     = "URL_DECODE"
        }
        text_transformation {
          priority = 1
          type     = "HTML_ENTITY_DECODE"
        }
      }
    }
    
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "SQLInjectionRule"
      sampled_requests_enabled   = true
    }
  }
  
  # XSS protection
  rule {
    name     = "XSSRule"
    priority = 3
    
    action {
      block {}
    }
    
    statement {
      xss_match_statement {
        field_to_match {
          body {}
        }
        text_transformation {
          priority = 0
          type     = "URL_DECODE"
        }
        text_transformation {
          priority = 1
          type     = "HTML_ENTITY_DECODE"
        }
      }
    }
    
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "XSSRule"
      sampled_requests_enabled   = true
    }
  }
  
  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "GlobalWAF"
    sampled_requests_enabled   = true
  }
  
  tags = {
    Name        = "Global WAF"
    Environment = var.environment
    Project     = var.project_name
  }
}

# Regional Infrastructure Module
module "regional_infrastructure" {
  source = "./modules/regional-infrastructure"
  
  for_each = toset(var.regions)
  
  region                      = each.value
  environment                = var.environment
  project_name               = var.project_name
  availability_zones_per_region = var.availability_zones_per_region
  instance_types             = var.instance_types
  
  # Global resources
  global_assets_bucket = aws_s3_bucket.global_assets.id
  route53_zone_id     = aws_route53_zone.main.zone_id
  
  providers = {
    aws = aws.region[each.value]
  }
}

# Global Database (Aurora Global)
resource "aws_rds_global_cluster" "consciousness_global" {
  global_cluster_identifier = "${var.project_name}-global-cluster"
  engine                   = "aurora-postgresql"
  engine_version           = "14.9"
  database_name            = "consciousness"
  master_username          = "consciousness_admin"
  manage_master_user_password = true
  
  deletion_protection = true
  
  tags = {
    Name        = "Global Database Cluster"
    Environment = var.environment
    Project     = var.project_name
  }
}

# Global ElastiCache Replication Group
resource "aws_elasticache_global_replication_group" "consciousness_global" {
  global_replication_group_id_suffix = "consciousness-global"
  description                        = "Global Redis cluster for Consciousness Engine"
  
  primary_replication_group_id = module.regional_infrastructure["us-east-1"].elasticache_replication_group_id
  
  tags = {
    Name        = "Global Cache Cluster"
    Environment = var.environment
    Project     = var.project_name
  }
}

# Global Monitoring and Alerting
resource "aws_cloudwatch_dashboard" "global_dashboard" {
  dashboard_name = "${var.project_name}-global-dashboard"
  
  dashboard_body = jsonencode({
    widgets = [
      {
        type   = "metric"
        x      = 0
        y      = 0
        width  = 12
        height = 6
        
        properties = {
          metrics = [
            ["AWS/CloudFront", "Requests", "DistributionId", aws_cloudfront_distribution.global_cdn.id],
            [".", "BytesDownloaded", ".", "."],
            [".", "4xxErrorRate", ".", "."],
            [".", "5xxErrorRate", ".", "."]
          ]
          view    = "timeSeries"
          stacked = false
          region  = "us-east-1"
          title   = "Global CDN Metrics"
          period  = 300
        }
      },
      {
        type   = "metric"
        x      = 0
        y      = 6
        width  = 12
        height = 6
        
        properties = {
          metrics = [
            ["AWS/Route53", "QueryCount", "HostedZoneId", aws_route53_zone.main.zone_id],
            ["AWS/WAFv2", "AllowedRequests", "WebACL", aws_wafv2_web_acl.global_waf.name, "Region", "CloudFront", "Rule", "ALL"],
            [".", "BlockedRequests", ".", ".", ".", ".", ".", "."]
          ]
          view    = "timeSeries"
          stacked = false
          region  = "us-east-1"
          title   = "Global DNS and Security Metrics"
          period  = 300
        }
      }
    ]
  })
}

# Global SNS Topic for Alerts
resource "aws_sns_topic" "global_alerts" {
  name = "${var.project_name}-global-alerts"
  
  tags = {
    Name        = "Global Alerts Topic"
    Environment = var.environment
    Project     = var.project_name
  }
}

# CloudWatch Alarms
resource "aws_cloudwatch_metric_alarm" "high_error_rate" {
  alarm_name          = "${var.project_name}-high-error-rate"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "2"
  metric_name         = "4xxErrorRate"
  namespace           = "AWS/CloudFront"
  period              = "300"
  statistic           = "Average"
  threshold           = "5"
  alarm_description   = "This metric monitors CloudFront 4xx error rate"
  alarm_actions       = [aws_sns_topic.global_alerts.arn]
  
  dimensions = {
    DistributionId = aws_cloudfront_distribution.global_cdn.id
  }
  
  tags = {
    Name        = "High Error Rate Alarm"
    Environment = var.environment
    Project     = var.project_name
  }
}

resource "aws_cloudwatch_metric_alarm" "high_latency" {
  alarm_name          = "${var.project_name}-high-latency"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "2"
  metric_name         = "OriginLatency"
  namespace           = "AWS/CloudFront"
  period              = "300"
  statistic           = "Average"
  threshold           = "5000"
  alarm_description   = "This metric monitors CloudFront origin latency"
  alarm_actions       = [aws_sns_topic.global_alerts.arn]
  
  dimensions = {
    DistributionId = aws_cloudfront_distribution.global_cdn.id
  }
  
  tags = {
    Name        = "High Latency Alarm"
    Environment = var.environment
    Project     = var.project_name
  }
}

# Outputs
output "global_cdn_domain" {
  description = "CloudFront distribution domain name"
  value       = aws_cloudfront_distribution.global_cdn.domain_name
}

output "global_cdn_id" {
  description = "CloudFront distribution ID"
  value       = aws_cloudfront_distribution.global_cdn.id
}

output "route53_zone_id" {
  description = "Route53 hosted zone ID"
  value       = aws_route53_zone.main.zone_id
}

output "global_database_cluster_id" {
  description = "Global database cluster ID"
  value       = aws_rds_global_cluster.consciousness_global.id
}

output "global_cache_cluster_id" {
  description = "Global cache cluster ID"
  value       = aws_elasticache_global_replication_group.consciousness_global.id
}

output "regional_endpoints" {
  description = "Regional API endpoints"
  value = {
    for region, infrastructure in module.regional_infrastructure :
    region => infrastructure.api_endpoint
  }
}

output "waf_web_acl_id" {
  description = "WAF Web ACL ID"
  value       = aws_wafv2_web_acl.global_waf.id
}

# Provider configurations for each region
provider "aws" {
  alias  = "us_east_1"
  region = "us-east-1"
}

provider "aws" {
  alias  = "us_west_2"
  region = "us-west-2"
}

provider "aws" {
  alias  = "eu_west_1"
  region = "eu-west-1"
}

provider "aws" {
  alias  = "eu_central_1"
  region = "eu-central-1"
}

provider "aws" {
  alias  = "ap_southeast_1"
  region = "ap-southeast-1"
}

provider "aws" {
  alias  = "ap_northeast_1"
  region = "ap-northeast-1"
}

provider "aws" {
  alias  = "ap_south_1"
  region = "ap-south-1"
}

provider "aws" {
  alias  = "sa_east_1"
  region = "sa-east-1"
}
