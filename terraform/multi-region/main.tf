terraform {
  required_version = ">= 1.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }
}

# Multi-region providers
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
  alias  = "ap_southeast_1"
  region = "ap-southeast-1"
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

# Global variables
locals {
  regions = {
    us_east_1      = { provider = "aws.us_east_1", name = "US East" }
    us_west_2      = { provider = "aws.us_west_2", name = "US West" }
    eu_west_1      = { provider = "aws.eu_west_1", name = "Europe" }
    ap_southeast_1 = { provider = "aws.ap_southeast_1", name = "Asia Pacific" }
  }
  
  project_name = "consciousness-engine"
  environment  = "production"
}

# Route53 Hosted Zone
resource "aws_route53_zone" "main" {
  provider = aws.us_east_1
  name     = var.domain_name

  tags = {
    Name        = "${local.project_name}-zone"
    Environment = local.environment
  }
}

# CloudFront Distribution for Global CDN
resource "aws_cloudfront_distribution" "main" {
  provider = aws.us_east_1

  origin {
    domain_name = var.primary_alb_dns_name
    origin_id   = "primary-alb"

    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  # Additional origins for each region
  dynamic "origin" {
    for_each = local.regions
    content {
      domain_name = var.regional_alb_dns_names[origin.key]
      origin_id   = "alb-${origin.key}"

      custom_origin_config {
        http_port              = 80
        https_port             = 443
        origin_protocol_policy = "https-only"
        origin_ssl_protocols   = ["TLSv1.2"]
      }
    }
  }

  enabled             = true
  is_ipv6_enabled     = true
  default_root_object = "index.html"
  aliases             = [var.domain_name, "www.${var.domain_name}"]

  # Default cache behavior
  default_cache_behavior {
    allowed_methods        = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "primary-alb"
    compress               = true
    viewer_protocol_policy = "redirect-to-https"

    forwarded_values {
      query_string = true
      headers      = ["Authorization", "CloudFront-Viewer-Country"]
      cookies {
        forward = "all"
      }
    }

    min_ttl     = 0
    default_ttl = 3600
    max_ttl     = 86400
  }

  # API cache behavior
  ordered_cache_behavior {
    path_pattern           = "/api/*"
    allowed_methods        = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "primary-alb"
    compress               = true
    viewer_protocol_policy = "redirect-to-https"

    forwarded_values {
      query_string = true
      headers      = ["*"]
      cookies {
        forward = "all"
      }
    }

    min_ttl     = 0
    default_ttl = 0
    max_ttl     = 0
  }

  # Static assets cache behavior
  ordered_cache_behavior {
    path_pattern           = "/static/*"
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "primary-alb"
    compress               = true
    viewer_protocol_policy = "redirect-to-https"

    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }

    min_ttl     = 31536000
    default_ttl = 31536000
    max_ttl     = 31536000
  }

  # Geographic restrictions
  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  # SSL Certificate
  viewer_certificate {
    acm_certificate_arn      = aws_acm_certificate.main.arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }

  # WAF
  web_acl_id = aws_wafv2_web_acl.main.arn

  tags = {
    Name        = "${local.project_name}-cloudfront"
    Environment = local.environment
  }
}

# ACM Certificate for CloudFront
resource "aws_acm_certificate" "main" {
  provider          = aws.us_east_1
  domain_name       = var.domain_name
  validation_method = "DNS"

  subject_alternative_names = [
    "*.${var.domain_name}"
  ]

  lifecycle {
    create_before_destroy = true
  }

  tags = {
    Name        = "${local.project_name}-cert"
    Environment = local.environment
  }
}

# WAF for DDoS protection
resource "aws_wafv2_web_acl" "main" {
  provider = aws.us_east_1
  name     = "${local.project_name}-waf"
  scope    = "CLOUDFRONT"

  default_action {
    allow {}
  }

  # Rate limiting rule
  rule {
    name     = "RateLimitRule"
    priority = 1

    override_action {
      none {}
    }

    statement {
      rate_based_statement {
        limit              = 2000
        aggregate_key_type = "IP"
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                 = "RateLimitRule"
      sampled_requests_enabled    = true
    }

    action {
      block {}
    }
  }

  # AWS Managed Rules
  rule {
    name     = "AWSManagedRulesCommonRuleSet"
    priority = 2

    override_action {
      none {}
    }

    statement {
      managed_rule_group_statement {
        name        = "AWSManagedRulesCommonRuleSet"
        vendor_name = "AWS"
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                 = "CommonRuleSetMetric"
      sampled_requests_enabled    = true
    }
  }

  tags = {
    Name        = "${local.project_name}-waf"
    Environment = local.environment
  }
}

# Global DynamoDB for session management
resource "aws_dynamodb_table" "sessions" {
  provider = aws.us_east_1
  name     = "${local.project_name}-sessions"

  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "session_id"
  stream_enabled = true

  attribute {
    name = "session_id"
    type = "S"
  }

  attribute {
    name = "user_id"
    type = "S"
  }

  global_secondary_index {
    name     = "UserIdIndex"
    hash_key = "user_id"
  }

  ttl {
    attribute_name = "expires_at"
    enabled        = true
  }

  # Global Tables for multi-region replication
  replica {
    region_name = "us-west-2"
  }

  replica {
    region_name = "eu-west-1"
  }

  replica {
    region_name = "ap-southeast-1"
  }

  tags = {
    Name        = "${local.project_name}-sessions"
    Environment = local.environment
  }
}

# ElastiCache Global Datastore for Redis
resource "aws_elasticache_global_replication_group" "main" {
  provider                       = aws.us_east_1
  global_replication_group_id    = "${local.project_name}-global-redis"
  description                    = "Global Redis for Consciousness Engine"
  primary_replication_group_id   = aws_elasticache_replication_group.primary.id
  automatic_failover_enabled     = true
  cache_node_type               = "cache.r6g.large"
  engine_version                = "7.0"
  num_cache_clusters            = 2
}

resource "aws_elasticache_replication_group" "primary" {
  provider           = aws.us_east_1
  replication_group_id = "${local.project_name}-redis-primary"
  description        = "Primary Redis cluster"
  
  node_type          = "cache.r6g.large"
  port               = 6379
  parameter_group_name = "default.redis7"
  
  num_cache_clusters = 2
  
  subnet_group_name  = aws_elasticache_subnet_group.primary.name
  security_group_ids = [aws_security_group.redis_primary.id]
  
  at_rest_encryption_enabled = true
  transit_encryption_enabled = true
  
  tags = {
    Name        = "${local.project_name}-redis-primary"
    Environment = local.environment
  }
}

# Outputs
output "cloudfront_domain_name" {
  value = aws_cloudfront_distribution.main.domain_name
}

output "cloudfront_distribution_id" {
  value = aws_cloudfront_distribution.main.id
}

output "route53_zone_id" {
  value = aws_route53_zone.main.zone_id
}
