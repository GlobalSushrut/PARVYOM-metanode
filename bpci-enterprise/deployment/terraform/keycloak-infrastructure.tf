# Industry-Standard Keycloak Infrastructure on DigitalOcean
# BPCI Enterprise Authentication System

terraform {
  required_version = ">= 1.0"
  required_providers {
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.0"
    }
  }
}

# Configure the DigitalOcean Provider
provider "digitalocean" {
  token = var.do_token
}

# Variables
variable "do_token" {
  description = "DigitalOcean API Token"
  type        = string
  sensitive   = true
}

variable "ssh_key_fingerprint" {
  description = "SSH Key Fingerprint for server access"
  type        = string
}

variable "domain_name" {
  description = "Domain name for Keycloak (e.g., auth.bpci.enterprise)"
  type        = string
  default     = "auth.bpci.enterprise"
}

# VPC for Keycloak Infrastructure
resource "digitalocean_vpc" "keycloak_vpc" {
  name     = "keycloak-vpc"
  region   = "nyc3"
  ip_range = "10.10.0.0/16"
  
  tags = ["keycloak", "production", "bpci-enterprise"]
}

# Keycloak Cluster Droplets
resource "digitalocean_droplet" "keycloak" {
  count  = 3
  image  = "ubuntu-22-04-x64"
  name   = "keycloak-${count.index + 1}"
  region = "nyc3"
  size   = "s-2vcpu-4gb"
  
  ssh_keys = [var.ssh_key_fingerprint]
  vpc_uuid = digitalocean_vpc.keycloak_vpc.id
  
  tags = ["keycloak", "production", "bpci-enterprise"]
  
  user_data = templatefile("${path.module}/user-data/keycloak-init.sh", {
    node_id = count.index + 1
  })
}

# PostgreSQL Database for Keycloak
resource "digitalocean_database_cluster" "keycloak_db" {
  name       = "keycloak-postgres"
  engine     = "pg"
  version    = "15"
  size       = "db-s-2vcpu-4gb"
  region     = "nyc3"
  node_count = 1
  
  tags = ["keycloak", "database", "production"]
}

# Database for Keycloak
resource "digitalocean_database_db" "keycloak_database" {
  cluster_id = digitalocean_database_cluster.keycloak_db.id
  name       = "keycloak"
}

# Database User for Keycloak
resource "digitalocean_database_user" "keycloak_user" {
  cluster_id = digitalocean_database_cluster.keycloak_db.id
  name       = "keycloak"
}

# SSL Certificate for Keycloak
resource "digitalocean_certificate" "keycloak_cert" {
  name    = "keycloak-ssl-cert"
  type    = "lets_encrypt"
  domains = [var.domain_name]
  
  lifecycle {
    create_before_destroy = true
  }
}

# Load Balancer for Keycloak Cluster
resource "digitalocean_loadbalancer" "keycloak_lb" {
  name   = "keycloak-lb"
  region = "nyc3"
  
  forwarding_rule {
    entry_protocol  = "https"
    entry_port      = 443
    target_protocol = "http"
    target_port     = 8080
    certificate_name = digitalocean_certificate.keycloak_cert.name
  }
  
  forwarding_rule {
    entry_protocol  = "http"
    entry_port      = 80
    target_protocol = "http"
    target_port     = 8080
  }
  
  healthcheck {
    protocol = "http"
    port     = 8080
    path     = "/health/ready"
    check_interval_seconds   = 10
    response_timeout_seconds = 5
    healthy_threshold        = 3
    unhealthy_threshold      = 3
  }
  
  droplet_ids = digitalocean_droplet.keycloak[*].id
  
  tags = ["keycloak", "load-balancer", "production"]
}

# Firewall Rules for Keycloak
resource "digitalocean_firewall" "keycloak_firewall" {
  name = "keycloak-firewall"
  
  droplet_ids = digitalocean_droplet.keycloak[*].id
  
  # SSH Access
  inbound_rule {
    protocol         = "tcp"
    port_range       = "22"
    source_addresses = ["0.0.0.0/0", "::/0"]
  }
  
  # HTTP/HTTPS from Load Balancer
  inbound_rule {
    protocol                = "tcp"
    port_range             = "8080"
    source_load_balancer_uids = [digitalocean_loadbalancer.keycloak_lb.id]
  }
  
  # Keycloak Cluster Communication
  inbound_rule {
    protocol         = "tcp"
    port_range       = "7600"
    source_addresses = [digitalocean_vpc.keycloak_vpc.ip_range]
  }
  
  # Database Access
  inbound_rule {
    protocol         = "tcp"
    port_range       = "5432"
    source_addresses = [digitalocean_vpc.keycloak_vpc.ip_range]
  }
  
  # All Outbound Traffic
  outbound_rule {
    protocol              = "tcp"
    port_range           = "1-65535"
    destination_addresses = ["0.0.0.0/0", "::/0"]
  }
  
  outbound_rule {
    protocol              = "udp"
    port_range           = "1-65535"
    destination_addresses = ["0.0.0.0/0", "::/0"]
  }
  
  outbound_rule {
    protocol              = "icmp"
    destination_addresses = ["0.0.0.0/0", "::/0"]
  }
}

# DNS Record for Keycloak
resource "digitalocean_record" "keycloak_dns" {
  domain = "bpci.enterprise"
  type   = "A"
  name   = "auth"
  value  = digitalocean_loadbalancer.keycloak_lb.ip
  ttl    = 300
}

# Spaces Bucket for Backups
resource "digitalocean_spaces_bucket" "keycloak_backups" {
  name   = "bpci-keycloak-backups"
  region = "nyc3"
  
  versioning {
    enabled = true
  }
  
  lifecycle_rule {
    id      = "backup-retention"
    enabled = true
    
    expiration {
      days = 90
    }
    
    noncurrent_version_expiration {
      days = 30
    }
  }
}

# Outputs
output "keycloak_lb_ip" {
  description = "Load Balancer IP for Keycloak"
  value       = digitalocean_loadbalancer.keycloak_lb.ip
}

output "keycloak_url" {
  description = "Keycloak URL"
  value       = "https://${var.domain_name}"
}

output "database_connection" {
  description = "Database connection details"
  value = {
    host     = digitalocean_database_cluster.keycloak_db.host
    port     = digitalocean_database_cluster.keycloak_db.port
    database = digitalocean_database_db.keycloak_database.name
    username = digitalocean_database_user.keycloak_user.name
  }
  sensitive = true
}

output "droplet_ips" {
  description = "Keycloak droplet IP addresses"
  value = {
    public_ips  = digitalocean_droplet.keycloak[*].ipv4_address
    private_ips = digitalocean_droplet.keycloak[*].ipv4_address_private
  }
}
