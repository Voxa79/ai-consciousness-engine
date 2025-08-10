#!/bin/bash

# User data script for EKS worker nodes
# This script configures the node to join the EKS cluster

set -o xtrace

# Update system packages
yum update -y

# Install additional packages for consciousness workloads
yum install -y \
    htop \
    iotop \
    sysstat \
    tcpdump \
    strace \
    lsof \
    jq \
    awscli

# Configure kubelet with optimizations for consciousness workloads
mkdir -p /etc/kubernetes/kubelet

# Set up kubelet configuration
cat > /etc/kubernetes/kubelet/kubelet-config.json <<EOF
{
    "kind": "KubeletConfiguration",
    "apiVersion": "kubelet.config.k8s.io/v1beta1",
    "address": "0.0.0.0",
    "port": 10250,
    "readOnlyPort": 0,
    "cgroupDriver": "systemd",
    "hairpinMode": "hairpin-veth",
    "serializeImagePulls": false,
    "featureGates": {
        "RotateKubeletServerCertificate": true,
        "CSIMigration": true,
        "CSIMigrationAWS": true
    },
    "serverTLSBootstrap": true,
    "tlsCipherSuites": [
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384"
    ],
    "clusterDomain": "cluster.local",
    "clusterDNS": ["172.20.0.10"],
    "resolvConf": "/run/systemd/resolve/resolv.conf",
    "runtimeRequestTimeout": "15m",
    "kubeReserved": {
        "cpu": "100m",
        "memory": "100Mi",
        "ephemeral-storage": "1Gi"
    },
    "systemReserved": {
        "cpu": "100m",
        "memory": "100Mi",
        "ephemeral-storage": "1Gi"
    },
    "evictionHard": {
        "memory.available": "200Mi",
        "nodefs.available": "10%"
    },
    "maxPods": 110,
    "podPidsLimit": 2048,
    "containerLogMaxSize": "50Mi",
    "containerLogMaxFiles": 5
}
EOF

# Configure containerd for optimized performance
mkdir -p /etc/containerd
cat > /etc/containerd/config.toml <<EOF
version = 2

[plugins]
  [plugins."io.containerd.grpc.v1.cri"]
    sandbox_image = "602401143452.dkr.ecr.us-west-2.amazonaws.com/eks/pause:3.5"
    
    [plugins."io.containerd.grpc.v1.cri".containerd]
      default_runtime_name = "runc"
      
      [plugins."io.containerd.grpc.v1.cri".containerd.runtimes]
        [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runc]
          runtime_type = "io.containerd.runc.v2"
          
          [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runc.options]
            SystemdCgroup = true
            
    [plugins."io.containerd.grpc.v1.cri".registry]
      [plugins."io.containerd.grpc.v1.cri".registry.mirrors]
        [plugins."io.containerd.grpc.v1.cri".registry.mirrors."docker.io"]
          endpoint = ["https://registry-1.docker.io"]
        [plugins."io.containerd.grpc.v1.cri".registry.mirrors."602401143452.dkr.ecr.us-west-2.amazonaws.com"]
          endpoint = ["https://602401143452.dkr.ecr.us-west-2.amazonaws.com"]
          
      [plugins."io.containerd.grpc.v1.cri".registry.configs]
        [plugins."io.containerd.grpc.v1.cri".registry.configs."602401143452.dkr.ecr.us-west-2.amazonaws.com".auth]
          username = "AWS"
          password = ""
EOF

# Restart containerd
systemctl restart containerd

# Set up log rotation for consciousness workloads
cat > /etc/logrotate.d/consciousness <<EOF
/var/log/consciousness/*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    create 0644 root root
    postrotate
        /bin/kill -HUP \$(cat /var/run/rsyslogd.pid 2> /dev/null) 2> /dev/null || true
    endscript
}
EOF

# Configure system limits for consciousness workloads
cat >> /etc/security/limits.conf <<EOF
# Consciousness workload limits
* soft nofile 65536
* hard nofile 65536
* soft nproc 32768
* hard nproc 32768
EOF

# Configure sysctl for consciousness workloads
cat > /etc/sysctl.d/99-consciousness.conf <<EOF
# Network optimizations for consciousness workloads
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 87380 134217728
net.ipv4.tcp_wmem = 4096 65536 134217728
net.core.netdev_max_backlog = 5000
net.ipv4.tcp_congestion_control = bbr

# Memory optimizations
vm.swappiness = 1
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5

# File system optimizations
fs.file-max = 2097152
fs.inotify.max_user_watches = 524288
EOF

# Apply sysctl settings
sysctl -p /etc/sysctl.d/99-consciousness.conf

# Install CloudWatch agent for enhanced monitoring
wget https://s3.amazonaws.com/amazoncloudwatch-agent/amazon_linux/amd64/latest/amazon-cloudwatch-agent.rpm
rpm -U ./amazon-cloudwatch-agent.rpm

# Configure CloudWatch agent
cat > /opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json <<EOF
{
    "agent": {
        "metrics_collection_interval": 60,
        "run_as_user": "cwagent"
    },
    "metrics": {
        "namespace": "EKS/Consciousness",
        "metrics_collected": {
            "cpu": {
                "measurement": [
                    "cpu_usage_idle",
                    "cpu_usage_iowait",
                    "cpu_usage_user",
                    "cpu_usage_system"
                ],
                "metrics_collection_interval": 60
            },
            "disk": {
                "measurement": [
                    "used_percent"
                ],
                "metrics_collection_interval": 60,
                "resources": [
                    "*"
                ]
            },
            "diskio": {
                "measurement": [
                    "io_time"
                ],
                "metrics_collection_interval": 60,
                "resources": [
                    "*"
                ]
            },
            "mem": {
                "measurement": [
                    "mem_used_percent"
                ],
                "metrics_collection_interval": 60
            },
            "netstat": {
                "measurement": [
                    "tcp_established",
                    "tcp_time_wait"
                ],
                "metrics_collection_interval": 60
            },
            "swap": {
                "measurement": [
                    "swap_used_percent"
                ],
                "metrics_collection_interval": 60
            }
        }
    },
    "logs": {
        "logs_collected": {
            "files": {
                "collect_list": [
                    {
                        "file_path": "/var/log/messages",
                        "log_group_name": "/aws/eks/${cluster_name}/system",
                        "log_stream_name": "{instance_id}/messages"
                    },
                    {
                        "file_path": "/var/log/dmesg",
                        "log_group_name": "/aws/eks/${cluster_name}/system",
                        "log_stream_name": "{instance_id}/dmesg"
                    }
                ]
            }
        }
    }
}
EOF

# Start CloudWatch agent
/opt/aws/amazon-cloudwatch-agent/bin/amazon-cloudwatch-agent-ctl \
    -a fetch-config \
    -m ec2 \
    -c file:/opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json \
    -s

# Bootstrap the node to join the EKS cluster
/etc/eks/bootstrap.sh ${cluster_name} ${bootstrap_arguments}

# Install additional tools for consciousness debugging
pip3 install --user awscli boto3

# Set up consciousness-specific directories
mkdir -p /var/log/consciousness
mkdir -p /var/lib/consciousness
mkdir -p /tmp/consciousness

# Set permissions
chown -R 1000:1000 /var/log/consciousness
chown -R 1000:1000 /var/lib/consciousness
chown -R 1000:1000 /tmp/consciousness

# Create consciousness user for running workloads
useradd -r -s /bin/false -d /var/lib/consciousness consciousness

# Install Node Exporter for Prometheus monitoring
wget https://github.com/prometheus/node_exporter/releases/download/v1.6.1/node_exporter-1.6.1.linux-amd64.tar.gz
tar xvfz node_exporter-1.6.1.linux-amd64.tar.gz
cp node_exporter-1.6.1.linux-amd64/node_exporter /usr/local/bin/
rm -rf node_exporter-1.6.1.linux-amd64*

# Create systemd service for Node Exporter
cat > /etc/systemd/system/node_exporter.service <<EOF
[Unit]
Description=Node Exporter
Wants=network-online.target
After=network-online.target

[Service]
User=nobody
Group=nobody
Type=simple
ExecStart=/usr/local/bin/node_exporter --web.listen-address=:9100
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
EOF

# Enable and start Node Exporter
systemctl daemon-reload
systemctl enable node_exporter
systemctl start node_exporter

# Signal that the node is ready
/opt/aws/bin/cfn-signal -e $? --stack ${AWS::StackName} --resource NodeGroup --region ${AWS::Region}

echo "Node initialization completed successfully"