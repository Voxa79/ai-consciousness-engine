# Consciousness Engine - Terraform Infrastructure

This directory contains the Terraform infrastructure as code for the Consciousness Engine platform.

## 🏗️ Architecture Overview

The infrastructure is designed with the following principles:
- **Multi-environment support** (dev, staging, prod, dr)
- **High availability** across multiple AZs
- **Security-first** with encryption at rest and in transit
- **Cost optimization** with spot instances and auto-scaling
- **Observability** with comprehensive monitoring

## 📁 Directory Structure

```
terraform/
├── modules/                    # Reusable Terraform modules
│   ├── networking/            # VPC, subnets, routing
│   ├── kubernetes/            # EKS cluster and node groups
│   ├── storage/              # RDS, S3, EBS
│   └── monitoring/           # Prometheus, Grafana, logging
├── environments/             # Environment-specific configurations
│   ├── dev/
│   ├── staging/
│   ├── prod/
│   └── dr/
├── backend-config/           # Terraform backend configurations
├── scripts/                  # Helper scripts
└── shared/                   # Shared configurations
```

## 🚀 Quick Start

### Prerequisites

1. **AWS CLI** configured with appropriate permissions
2. **Terraform** >= 1.6 installed
3. **kubectl** for Kubernetes management
4. **helm** for Kubernetes package management

### 1. Initialize Backend

First, create the S3 bucket and DynamoDB table for Terraform state:

```bash
# Make script executable (Linux/Mac)
chmod +x scripts/init-backend.sh

# Initialize backend for production
./scripts/init-backend.sh prod

# For other environments
./scripts/init-backend.sh staging
./scripts/init-backend.sh dev
```

### 2. Deploy Infrastructure

```bash
# Navigate to environment directory
cd environments/prod

# Initialize Terraform with backend configuration
terraform init -backend-config=../../backend-config/prod.hcl

# Review the plan
terraform plan

# Apply the infrastructure
terraform apply
```

### 3. Configure kubectl

After EKS cluster is created:

```bash
# Update kubeconfig
aws eks update-kubeconfig --region us-west-2 --name consciousness-engine-prod-cluster

# Verify connection
kubectl get nodes
```

## 🔧 Configuration

### Environment Variables

Each environment has its own `terraform.tfvars` file with specific configurations:

- **VPC CIDR blocks** to avoid conflicts
- **Instance types and sizes** based on workload requirements
- **Database configurations** for performance and cost
- **Monitoring and security settings**

### Node Groups

The EKS cluster uses multiple node groups for different workloads:

- **consciousness**: High-performance instances for AI processing
- **analytics**: Memory-optimized instances for data processing
- **system**: General-purpose instances for system services

### Security Features

- **Encryption at rest** for all storage (RDS, S3, EBS)
- **Encryption in transit** with TLS 1.3
- **Network segmentation** with private subnets
- **VPC Flow Logs** for network monitoring
- **IAM roles** with least privilege access

## 📊 Monitoring & Observability

The infrastructure includes comprehensive monitoring:

- **Prometheus** for metrics collection
- **Grafana** for visualization
- **Jaeger** for distributed tracing
- **ELK Stack** for log aggregation
- **CloudWatch** for AWS native monitoring

## 💰 Cost Optimization

Several cost optimization features are implemented:

- **Spot instances** for non-critical workloads (configurable percentage)
- **Auto-scaling** based on demand
- **Right-sizing** recommendations
- **Reserved instances** for predictable workloads

## 🔒 Security Best Practices

- **Zero-trust networking** with service mesh
- **Secrets management** with HashiCorp Vault
- **Policy as code** with Open Policy Agent
- **Runtime security** with Falco
- **Compliance scanning** with custom tools

## 🌍 Multi-Cloud Strategy

The infrastructure is designed to be cloud-agnostic:

- **Terraform modules** can be adapted for other cloud providers
- **Kubernetes** provides consistent runtime environment
- **Service mesh** abstracts network complexity
- **GitOps** enables consistent deployments

## 📋 Operations

### Scaling

```bash
# Scale node group
aws eks update-nodegroup-config \
  --cluster-name consciousness-engine-prod-cluster \
  --nodegroup-name consciousness-prod \
  --scaling-config minSize=5,maxSize=100,desiredSize=10
```

### Monitoring

```bash
# Port forward to Grafana
kubectl port-forward -n monitoring svc/grafana 3000:80

# Access Grafana at http://localhost:3000
```

### Backup & Recovery

- **RDS automated backups** with 30-day retention
- **EBS snapshots** for persistent volumes
- **S3 cross-region replication** for critical data
- **Velero** for Kubernetes cluster backups

## 🚨 Troubleshooting

### Common Issues

1. **EKS nodes not joining cluster**
   - Check security groups and IAM roles
   - Verify subnet routing

2. **RDS connection issues**
   - Check security groups
   - Verify database subnet group

3. **High costs**
   - Review spot instance configuration
   - Check auto-scaling policies
   - Analyze CloudWatch cost metrics

### Useful Commands

```bash
# Check EKS cluster status
aws eks describe-cluster --name consciousness-engine-prod-cluster

# List node groups
aws eks list-nodegroups --cluster-name consciousness-engine-prod-cluster

# Check RDS instances
aws rds describe-db-instances

# Monitor costs
aws ce get-cost-and-usage --time-period Start=2024-01-01,End=2024-01-31 --granularity MONTHLY --metrics BlendedCost
```

## 🔄 CI/CD Integration

The infrastructure supports GitOps workflows:

- **Terraform Cloud** for remote state and planning
- **GitHub Actions** for automated deployments
- **ArgoCD** for Kubernetes application deployment
- **Policy validation** before deployment

## 📚 Additional Resources

- [AWS EKS Best Practices](https://aws.github.io/aws-eks-best-practices/)
- [Terraform AWS Provider Documentation](https://registry.terraform.io/providers/hashicorp/aws/latest/docs)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Prometheus Operator](https://prometheus-operator.dev/)

## 🤝 Contributing

1. Create feature branch
2. Make changes with proper documentation
3. Test in dev environment
4. Submit pull request with detailed description
5. Ensure all checks pass before merging

## 📞 Support

For infrastructure issues:
- Create GitHub issue with detailed description
- Include Terraform logs and error messages
- Tag @platform-team for urgent issues

---

**Built with ❤️ for the Consciousness Engine Platform**