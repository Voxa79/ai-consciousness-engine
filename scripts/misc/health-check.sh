#!/bin/bash

# Health Check Script for Consciousness Platform
# Comprehensive health monitoring for all components

set -e

# Configuration
API_GATEWAY_URL="http://127.0.0.1:8080"
CONSCIOUSNESS_ENGINE_URL="http://127.0.0.1:8081"
WEB_UI_URL="http://127.0.0.1:80"
HEALTH_CHECK_TIMEOUT=10
LOG_FILE="/app/logs/health-check.log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

# Function to check HTTP endpoint
check_http_endpoint() {
    local name=$1
    local url=$2
    local expected_status=${3:-200}
    
    log "Checking $name at $url..."
    
    if command -v curl &> /dev/null; then
        response=$(curl -s -o /dev/null -w "%{http_code}" --max-time "$HEALTH_CHECK_TIMEOUT" "$url" || echo "000")
    elif command -v wget &> /dev/null; then
        response=$(wget -q -O /dev/null --timeout="$HEALTH_CHECK_TIMEOUT" --server-response "$url" 2>&1 | grep "HTTP/" | tail -1 | awk '{print $2}' || echo "000")
    else
        log "${RED}❌ Neither curl nor wget available for health check${NC}"
        return 1
    fi
    
    if [ "$response" = "$expected_status" ]; then
        log "${GREEN}✅ $name is healthy (HTTP $response)${NC}"
        return 0
    else
        log "${RED}❌ $name is unhealthy (HTTP $response, expected $expected_status)${NC}"
        return 1
    fi
}

# Function to check process
check_process() {
    local process_name=$1
    
    if pgrep -f "$process_name" > /dev/null; then
        log "${GREEN}✅ Process $process_name is running${NC}"
        return 0
    else
        log "${RED}❌ Process $process_name is not running${NC}"
        return 1
    fi
}

# Function to check memory usage
check_memory_usage() {
    local max_memory_percent=${1:-80}
    
    if command -v free &> /dev/null; then
        memory_usage=$(free | grep Mem | awk '{printf "%.0f", $3/$2 * 100.0}')
        
        if [ "$memory_usage" -lt "$max_memory_percent" ]; then
            log "${GREEN}✅ Memory usage is healthy ($memory_usage%)${NC}"
            return 0
        else
            log "${YELLOW}⚠️  Memory usage is high ($memory_usage%)${NC}"
            return 1
        fi
    else
        log "${YELLOW}⚠️  Cannot check memory usage (free command not available)${NC}"
        return 0
    fi
}

# Function to check disk space
check_disk_space() {
    local max_disk_percent=${1:-90}
    
    if command -v df &> /dev/null; then
        disk_usage=$(df /app | tail -1 | awk '{print $5}' | sed 's/%//')
        
        if [ "$disk_usage" -lt "$max_disk_percent" ]; then
            log "${GREEN}✅ Disk space is healthy ($disk_usage%)${NC}"
            return 0
        else
            log "${YELLOW}⚠️  Disk space is low ($disk_usage%)${NC}"
            return 1
        fi
    else
        log "${YELLOW}⚠️  Cannot check disk space (df command not available)${NC}"
        return 0
    fi
}

# Function to check consciousness engine specific health
check_consciousness_health() {
    log "Checking consciousness engine health..."
    
    # Check if consciousness engine responds to health endpoint
    if check_http_endpoint "Consciousness Engine Health" "$CONSCIOUSNESS_ENGINE_URL/health"; then
        # Additional consciousness-specific checks
        if command -v curl &> /dev/null; then
            # Test basic consciousness interaction
            response=$(curl -s --max-time "$HEALTH_CHECK_TIMEOUT" \
                -X POST \
                -H "Content-Type: application/json" \
                -d '{"input": "Health check test"}' \
                "$CONSCIOUSNESS_ENGINE_URL/process" || echo "error")
            
            if [[ "$response" != "error" ]] && [[ "$response" == *"consciousness"* ]]; then
                log "${GREEN}✅ Consciousness processing is functional${NC}"
                return 0
            else
                log "${YELLOW}⚠️  Consciousness processing may have issues${NC}"
                return 1
            fi
        fi
    else
        return 1
    fi
}

# Main health check function
perform_health_check() {
    log "🏥 Starting comprehensive health check..."
    
    local health_score=0
    local total_checks=0
    
    # Check processes
    total_checks=$((total_checks + 1))
    if check_process "nginx"; then
        health_score=$((health_score + 1))
    fi
    
    total_checks=$((total_checks + 1))
    if check_process "api-gateway"; then
        health_score=$((health_score + 1))
    fi
    
    total_checks=$((total_checks + 1))
    if check_process "consciousness-engine"; then
        health_score=$((health_score + 1))
    fi
    
    # Check HTTP endpoints
    total_checks=$((total_checks + 1))
    if check_http_endpoint "Web UI" "$WEB_UI_URL/health"; then
        health_score=$((health_score + 1))
    fi
    
    total_checks=$((total_checks + 1))
    if check_http_endpoint "API Gateway" "$API_GATEWAY_URL/health"; then
        health_score=$((health_score + 1))
    fi
    
    total_checks=$((total_checks + 1))
    if check_consciousness_health; then
        health_score=$((health_score + 1))
    fi
    
    # Check system resources
    total_checks=$((total_checks + 1))
    if check_memory_usage 80; then
        health_score=$((health_score + 1))
    fi
    
    total_checks=$((total_checks + 1))
    if check_disk_space 90; then
        health_score=$((health_score + 1))
    fi
    
    # Calculate health percentage
    health_percentage=$((health_score * 100 / total_checks))
    
    log "📊 Health check completed: $health_score/$total_checks checks passed ($health_percentage%)"
    
    if [ "$health_percentage" -ge 80 ]; then
        log "${GREEN}🎉 System is healthy${NC}"
        return 0
    elif [ "$health_percentage" -ge 60 ]; then
        log "${YELLOW}⚠️  System has some issues but is functional${NC}"
        return 1
    else
        log "${RED}❌ System is unhealthy${NC}"
        return 2
    fi
}

# Monitoring mode (continuous health checking)
monitor_mode() {
    log "🔄 Starting continuous health monitoring..."
    
    while true; do
        perform_health_check
        sleep 60  # Check every minute
    done
}

# Main script logic
case "${1:-check}" in
    "check")
        perform_health_check
        ;;
    "--monitor")
        monitor_mode
        ;;
    "--help")
        echo "Usage: $0 [check|--monitor|--help]"
        echo "  check     - Perform single health check (default)"
        echo "  --monitor - Continuous health monitoring"
        echo "  --help    - Show this help message"
        ;;
    *)
        log "${RED}❌ Unknown option: $1${NC}"
        exit 1
        ;;
esac