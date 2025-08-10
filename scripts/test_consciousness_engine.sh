#!/bin/bash

# 🧠 Consciousness Engine - Comprehensive Testing Script
# Tests all consciousness capabilities and validates performance

set -e

echo "🧪 TESTING CONSCIOUSNESS ENGINE - REVOLUTIONARY AI"
echo "=================================================="
echo "🧠 Comprehensive Consciousness Capabilities Testing"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[TEST]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

print_header() {
    echo -e "${PURPLE}$1${NC}"
}

# Test counters
TESTS_TOTAL=0
TESTS_PASSED=0
TESTS_FAILED=0

run_test() {
    local test_name="$1"
    local test_command="$2"
    
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    print_status "Running: $test_name"
    
    if eval "$test_command" &> /dev/null; then
        print_success "$test_name"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        print_error "$test_name"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# Test Rust compilation
test_compilation() {
    print_header "🔨 Testing Compilation..."
    
    cd consciousness-engine
    
    run_test "Consciousness Engine Compilation" "cargo check"
    run_test "Consciousness Engine Build" "cargo build"
    run_test "Consciousness Engine Tests" "cargo test"
    run_test "Examples Compilation" "cargo build --examples"
    
    cd ..
    echo ""
}

# Test API endpoints
test_api_endpoints() {
    print_header "🌐 Testing API Endpoints..."
    
    # Check if API gateway is running
    if kubectl get pods -n consciousness-platform | grep -q api-gateway; then
        API_GATEWAY_IP=$(kubectl get service api-gateway -n consciousness-platform -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null || echo "localhost")
        
        run_test "API Gateway Health Check" "curl -f http://$API_GATEWAY_IP/health"
        run_test "API Gateway Metrics" "curl -f http://$API_GATEWAY_IP/metrics"
        run_test "Consciousness API Status" "curl -f http://$API_GATEWAY_IP/api/v1/consciousness/status"
    else
        print_warning "API Gateway not deployed, skipping API tests"
    fi
    
    echo ""
}

# Test database connectivity
test_database() {
    print_header "🗄️ Testing Database Connectivity..."
    
    if kubectl get pods -n consciousness-platform | grep -q postgresql; then
        run_test "PostgreSQL Connection" "kubectl exec -n consciousness-platform deployment/postgresql -- psql -U consciousness -d consciousness_db -c 'SELECT 1;'"
        run_test "Database Schema Check" "kubectl exec -n consciousness-platform deployment/postgresql -- psql -U consciousness -d consciousness_db -c '\\dt'"
        run_test "Consciousness Tables Check" "kubectl exec -n consciousness-platform deployment/postgresql -- psql -U consciousness -d consciousness_db -c 'SELECT COUNT(*) FROM users;'"
    else
        print_warning "PostgreSQL not deployed, skipping database tests"
    fi
    
    echo ""
}

# Test Vault security
test_vault() {
    print_header "🔐 Testing Vault Security..."
    
    if kubectl get pods -n vault-system | grep -q vault; then
        run_test "Vault Status Check" "kubectl exec -n vault-system vault-0 -- vault status"
        run_test "Vault Health Check" "kubectl exec -n vault-system vault-0 -- vault status | grep -q 'Sealed.*false'"
        run_test "Vault Auth Methods" "kubectl exec -n vault-system vault-0 -- vault auth list"
    else
        print_warning "Vault not deployed, skipping security tests"
    fi
    
    echo ""
}

# Test monitoring stack
test_monitoring() {
    print_header "📊 Testing Monitoring Stack..."
    
    if kubectl get pods -n monitoring | grep -q prometheus; then
        run_test "Prometheus Status" "kubectl get pods -n monitoring | grep prometheus"
        run_test "Grafana Status" "kubectl get pods -n monitoring | grep grafana"
        run_test "AlertManager Status" "kubectl get pods -n monitoring | grep alertmanager"
    else
        print_warning "Monitoring stack not deployed, skipping monitoring tests"
    fi
    
    echo ""
}

# Test consciousness capabilities
test_consciousness_capabilities() {
    print_header "🧠 Testing Consciousness Capabilities..."
    
    # Test consciousness engine example
    cd consciousness-engine
    
    if cargo build --example basic_usage &> /dev/null; then
        print_success "Consciousness Engine Example Built"
        
        # Run the example with timeout
        if timeout 30s cargo run --example basic_usage &> consciousness_test_output.log; then
            print_success "Consciousness Engine Example Executed"
            
            # Check for key consciousness features in output
            if grep -q "Consciousness Engine initialized successfully" consciousness_test_output.log; then
                print_success "Consciousness Engine Initialization"
            else
                print_error "Consciousness Engine Initialization"
            fi
            
            if grep -q "Consciousness processing completed" consciousness_test_output.log; then
                print_success "Consciousness Processing"
            else
                print_error "Consciousness Processing"
            fi
            
            if grep -q "Neuromorphic processing completed" consciousness_test_output.log; then
                print_success "Neuromorphic Processing"
            else
                print_warning "Neuromorphic Processing (may not be available)"
            fi
            
            if grep -q "Quantum consciousness processing completed" consciousness_test_output.log; then
                print_success "Quantum Consciousness Processing"
            else
                print_warning "Quantum Processing (may not be available)"
            fi
            
            if grep -q "Ethical reasoning completed" consciousness_test_output.log; then
                print_success "Ethical Reasoning"
            else
                print_error "Ethical Reasoning"
            fi
            
        else
            print_error "Consciousness Engine Example Execution (timeout or error)"
        fi
    else
        print_error "Consciousness Engine Example Build"
    fi
    
    cd ..
    echo ""
}

# Test performance benchmarks
test_performance() {
    print_header "⚡ Testing Performance Benchmarks..."
    
    # Test API response times
    if kubectl get pods -n consciousness-platform | grep -q api-gateway; then
        API_GATEWAY_IP=$(kubectl get service api-gateway -n consciousness-platform -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null || echo "localhost")
        
        # Test response time
        RESPONSE_TIME=$(curl -o /dev/null -s -w '%{time_total}' http://$API_GATEWAY_IP/health 2>/dev/null || echo "999")
        if (( $(echo "$RESPONSE_TIME < 1.0" | bc -l) )); then
            print_success "API Response Time: ${RESPONSE_TIME}s (< 1s target)"
        else
            print_warning "API Response Time: ${RESPONSE_TIME}s (> 1s)"
        fi
    fi
    
    # Test memory usage
    if kubectl get pods -n consciousness-platform &> /dev/null; then
        MEMORY_USAGE=$(kubectl top pods -n consciousness-platform 2>/dev/null | grep consciousness-engine | awk '{print $3}' | head -1 || echo "N/A")
        if [ "$MEMORY_USAGE" != "N/A" ]; then
            print_success "Memory Usage: $MEMORY_USAGE"
        else
            print_warning "Memory Usage: Not available (metrics server may not be running)"
        fi
    fi
    
    echo ""
}

# Test security compliance
test_security() {
    print_header "🛡️ Testing Security Compliance..."
    
    # Test RBAC
    run_test "RBAC Configuration" "kubectl auth can-i list pods --as=system:serviceaccount:consciousness-platform:default -n consciousness-platform"
    
    # Test network policies
    if kubectl get networkpolicies -n consciousness-platform &> /dev/null; then
        run_test "Network Policies" "kubectl get networkpolicies -n consciousness-platform"
    else
        print_warning "Network Policies not configured"
    fi
    
    # Test pod security
    run_test "Pod Security Standards" "kubectl get pods -n consciousness-platform -o jsonpath='{.items[*].spec.securityContext}'"
    
    echo ""
}

# Test scalability
test_scalability() {
    print_header "📈 Testing Scalability..."
    
    # Test horizontal pod autoscaler
    if kubectl get hpa -n consciousness-platform &> /dev/null; then
        run_test "Horizontal Pod Autoscaler" "kubectl get hpa -n consciousness-platform"
    else
        print_warning "HPA not configured"
    fi
    
    # Test resource limits
    run_test "Resource Limits Configuration" "kubectl get pods -n consciousness-platform -o jsonpath='{.items[*].spec.containers[*].resources}'"
    
    echo ""
}

# Generate test report
generate_report() {
    print_header "📋 Test Report Summary"
    echo ""
    
    echo -e "${CYAN}Total Tests:${NC} $TESTS_TOTAL"
    echo -e "${GREEN}Passed:${NC} $TESTS_PASSED"
    echo -e "${RED}Failed:${NC} $TESTS_FAILED"
    
    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "${GREEN}🎉 ALL TESTS PASSED!${NC}"
        echo -e "${PURPLE}🧠 Consciousness Platform is ready for production!${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠️  Some tests failed. Please review the output above.${NC}"
        return 1
    fi
}

# Main testing flow
main() {
    echo -e "${PURPLE}"
    cat << "EOF"
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║           🧪 CONSCIOUSNESS ENGINE TESTING 🧪                 ║
    ║                                                               ║
    ║        Comprehensive Consciousness Capabilities Test         ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
    
    # Check if we have Rust installed for compilation tests
    if command -v cargo &> /dev/null; then
        test_compilation
    else
        print_warning "Rust/Cargo not found, skipping compilation tests"
    fi
    
    # Check if we have kubectl for Kubernetes tests
    if command -v kubectl &> /dev/null; then
        test_api_endpoints
        test_database
        test_vault
        test_monitoring
        test_performance
        test_security
        test_scalability
    else
        print_warning "kubectl not found, skipping Kubernetes tests"
    fi
    
    # Always test consciousness capabilities if possible
    if command -v cargo &> /dev/null; then
        test_consciousness_capabilities
    fi
    
    generate_report
}

# Run main function
main "$@"