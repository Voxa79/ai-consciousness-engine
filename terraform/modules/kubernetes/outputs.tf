output "cluster_id" {
  description = "ID of the EKS cluster"
  value       = aws_eks_cluster.main.id
}

output "cluster_name" {
  description = "Name of the EKS cluster"
  value       = aws_eks_cluster.main.name
}

output "cluster_arn" {
  description = "ARN of the EKS cluster"
  value       = aws_eks_cluster.main.arn
}

output "cluster_endpoint" {
  description = "Endpoint for the EKS cluster API server"
  value       = aws_eks_cluster.main.endpoint
}

output "cluster_version" {
  description = "Version of the EKS cluster"
  value       = aws_eks_cluster.main.version
}

output "cluster_platform_version" {
  description = "Platform version of the EKS cluster"
  value       = aws_eks_cluster.main.platform_version
}

output "cluster_certificate_authority_data" {
  description = "Base64 encoded certificate data required to communicate with the cluster"
  value       = aws_eks_cluster.main.certificate_authority[0].data
}

output "cluster_security_group_id" {
  description = "Security group ID attached to the EKS cluster"
  value       = aws_security_group.cluster.id
}

output "node_security_group_id" {
  description = "Security group ID attached to the EKS node groups"
  value       = aws_security_group.node_group.id
}

output "cluster_iam_role_arn" {
  description = "IAM role ARN of the EKS cluster"
  value       = aws_iam_role.cluster.arn
}

output "cluster_iam_role_name" {
  description = "IAM role name of the EKS cluster"
  value       = aws_iam_role.cluster.name
}

output "node_groups" {
  description = "Map of node group configurations"
  value = {
    for k, v in aws_eks_node_group.main : k => {
      arn           = v.arn
      status        = v.status
      capacity_type = v.capacity_type
      instance_types = v.instance_types
      scaling_config = v.scaling_config
      labels        = v.labels
    }
  }
}

output "node_group_iam_role_arns" {
  description = "IAM role ARNs of the EKS node groups"
  value = {
    for k, v in aws_iam_role.node_group : k => v.arn
  }
}

output "oidc_provider_arn" {
  description = "ARN of the OIDC provider for IRSA"
  value       = var.enable_irsa ? aws_iam_openid_connect_provider.cluster[0].arn : null
}

output "oidc_provider_url" {
  description = "URL of the OIDC provider for IRSA"
  value       = var.enable_irsa ? aws_iam_openid_connect_provider.cluster[0].url : null
}

output "cluster_addons" {
  description = "Map of cluster addons"
  value = {
    vpc_cni = {
      arn     = aws_eks_addon.vpc_cni.arn
      version = aws_eks_addon.vpc_cni.addon_version
    }
    coredns = {
      arn     = aws_eks_addon.coredns.arn
      version = aws_eks_addon.coredns.addon_version
    }
    kube_proxy = {
      arn     = aws_eks_addon.kube_proxy.arn
      version = aws_eks_addon.kube_proxy.addon_version
    }
    ebs_csi_driver = {
      arn     = aws_eks_addon.ebs_csi_driver.arn
      version = aws_eks_addon.ebs_csi_driver.addon_version
    }
  }
}

output "kms_key_arn" {
  description = "ARN of the KMS key used for EKS encryption"
  value       = aws_kms_key.eks.arn
}

output "kms_key_id" {
  description = "ID of the KMS key used for EKS encryption"
  value       = aws_kms_key.eks.key_id
}

output "cloudwatch_log_group_name" {
  description = "Name of the CloudWatch log group for cluster logs"
  value       = aws_cloudwatch_log_group.cluster.name
}

output "cloudwatch_log_group_arn" {
  description = "ARN of the CloudWatch log group for cluster logs"
  value       = aws_cloudwatch_log_group.cluster.arn
}