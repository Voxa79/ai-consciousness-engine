#!/bin/bash

# Script to initialize Terraform backend for Consciousness Engine
# This script creates the S3 bucket and DynamoDB table for Terraform state management

set -e

# Configuration
PROJECT_NAME="consciousness-engine"
AWS_REGION="us-west-2"
ENVIRONMENT=${1:-prod}

# Derived names
BUCKET_NAME="${PROJECT_NAME}-terraform-state-${ENVIRONMENT}"
DYNAMODB_TABLE="${PROJECT_NAME}-terraform-locks"

echo "🚀 Initializing Terraform backend for ${PROJECT_NAME} (${ENVIRONMENT})"
echo "📦 S3 Bucket: ${BUCKET_NAME}"
echo "🔒 DynamoDB Table: ${DYNAMODB_TABLE}"

# Check if AWS CLI is installed
if ! command -v aws &> /dev/null; then
    echo "❌ AWS CLI is not installed. Please install it first."
    exit 1
fi

# Check if AWS credentials are configured
if ! aws sts get-caller-identity &> /dev/null; then
    echo "❌ AWS credentials are not configured. Please run 'aws configure' first."
    exit 1
fi

echo "✅ AWS CLI is configured"

# Create S3 bucket for Terraform state
echo "📦 Creating S3 bucket for Terraform state..."
if aws s3api head-bucket --bucket "${BUCKET_NAME}" 2>/dev/null; then
    echo "✅ S3 bucket ${BUCKET_NAME} already exists"
else
    aws s3api create-bucket \
        --bucket "${BUCKET_NAME}" \
        --region "${AWS_REGION}" \
        --create-bucket-configuration LocationConstraint="${AWS_REGION}"
    
    echo "✅ Created S3 bucket ${BUCKET_NAME}"
fi

# Enable versioning on the S3 bucket
echo "🔄 Enabling versioning on S3 bucket..."
aws s3api put-bucket-versioning \
    --bucket "${BUCKET_NAME}" \
    --versioning-configuration Status=Enabled

echo "✅ Enabled versioning on S3 bucket"

# Enable server-side encryption
echo "🔐 Enabling server-side encryption on S3 bucket..."
aws s3api put-bucket-encryption \
    --bucket "${BUCKET_NAME}" \
    --server-side-encryption-configuration '{
        "Rules": [
            {
                "ApplyServerSideEncryptionByDefault": {
                    "SSEAlgorithm": "AES256"
                }
            }
        ]
    }'

echo "✅ Enabled server-side encryption on S3 bucket"

# Block public access
echo "🚫 Blocking public access to S3 bucket..."
aws s3api put-public-access-block \
    --bucket "${BUCKET_NAME}" \
    --public-access-block-configuration \
        BlockPublicAcls=true,IgnorePublicAcls=true,BlockPublicPolicy=true,RestrictPublicBuckets=true

echo "✅ Blocked public access to S3 bucket"

# Create DynamoDB table for state locking
echo "🔒 Creating DynamoDB table for state locking..."
if aws dynamodb describe-table --table-name "${DYNAMODB_TABLE}" &> /dev/null; then
    echo "✅ DynamoDB table ${DYNAMODB_TABLE} already exists"
else
    aws dynamodb create-table \
        --table-name "${DYNAMODB_TABLE}" \
        --attribute-definitions AttributeName=LockID,AttributeType=S \
        --key-schema AttributeName=LockID,KeyType=HASH \
        --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5 \
        --region "${AWS_REGION}"
    
    echo "✅ Created DynamoDB table ${DYNAMODB_TABLE}"
    
    # Wait for table to be active
    echo "⏳ Waiting for DynamoDB table to be active..."
    aws dynamodb wait table-exists --table-name "${DYNAMODB_TABLE}"
    echo "✅ DynamoDB table is active"
fi

# Add tags to resources
echo "🏷️  Adding tags to resources..."

# Tag S3 bucket
aws s3api put-bucket-tagging \
    --bucket "${BUCKET_NAME}" \
    --tagging 'TagSet=[
        {Key=Project,Value=consciousness-engine},
        {Key=Environment,Value='${ENVIRONMENT}'},
        {Key=ManagedBy,Value=terraform},
        {Key=Purpose,Value=terraform-state}
    ]'

# Tag DynamoDB table
aws dynamodb tag-resource \
    --resource-arn "arn:aws:dynamodb:${AWS_REGION}:$(aws sts get-caller-identity --query Account --output text):table/${DYNAMODB_TABLE}" \
    --tags Key=Project,Value=consciousness-engine Key=Environment,Value="${ENVIRONMENT}" Key=ManagedBy,Value=terraform Key=Purpose,Value=terraform-locks

echo "✅ Added tags to resources"

echo ""
echo "🎉 Terraform backend initialization complete!"
echo ""
echo "📋 Next steps:"
echo "1. Navigate to your Terraform configuration directory"
echo "2. Run: terraform init -backend-config=backend-config/${ENVIRONMENT}.hcl"
echo "3. Run: terraform plan"
echo "4. Run: terraform apply"
echo ""
echo "🔧 Backend Configuration:"
echo "   Bucket: ${BUCKET_NAME}"
echo "   Region: ${AWS_REGION}"
echo "   DynamoDB Table: ${DYNAMODB_TABLE}"
echo ""