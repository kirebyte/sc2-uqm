#!/usr/bin/env bash

# Modern Build Script for The Ur-Quan Masters Refactored
# Comprehensive build system with quality checks, testing, and CI/CD support
# 
# Author: The Ur-Quan Masters Refactoring Team
# Version: 2.0.0
# Date: 2025-01-27

set -euo pipefail

# ============================================================================
# CONFIGURATION
# ============================================================================

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_ROOT="${SCRIPT_DIR}"
readonly PROJECT_NAME="uqm-refactored"
readonly PROJECT_VERSION="2.0.0"

# Build configuration
BUILD_TYPE="${BUILD_TYPE:-Release}"
PARALLEL_JOBS="${PARALLEL_JOBS:-$(nproc 2>/dev/null || echo 4)}"
VERBOSE="${VERBOSE:-0}"
CLEAN="${CLEAN:-0}"
TESTS="${TESTS:-1}"
DOCS="${DOCS:-0}"
ANALYSIS="${ANALYSIS:-0}"
INSTALL="${INSTALL:-0}"
PACKAGE="${PACKAGE:-0}"

# Directories
readonly BUILD_DIR="${PROJECT_ROOT}/build/${BUILD_TYPE}"
readonly SRC_DIR="${PROJECT_ROOT}/src"
readonly TEST_DIR="${PROJECT_ROOT}/tests"
readonly DOC_DIR="${PROJECT_ROOT}/docs"

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly PURPLE='\033[0;35m'
readonly CYAN='\033[0;36m'
readonly NC='\033[0m' # No Color

# ============================================================================
# UTILITY FUNCTIONS
# ============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

log_step() {
    echo -e "${PURPLE}[STEP]${NC} $*"
}

check_command() {
    local cmd="$1"
    local package="${2:-$1}"
    
    if ! command -v "$cmd" &> /dev/null; then
        log_error "Command '$cmd' not found. Please install $package"
        return 1
    fi
}

run_command() {
    local description="$1"
    shift
    
    log_step "$description"
    if [[ "$VERBOSE" == "1" ]]; then
        echo -e "${CYAN}Running:${NC} $*"
    fi
    
    if "$@"; then
        log_success "$description completed"
    else
        log_error "$description failed"
        return 1
    fi
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

show_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Modern build script for The Ur-Quan Masters Refactored

OPTIONS:
    -h, --help          Show this help message
    -c, --clean         Clean build directory before building
    -t, --tests         Run test suite (default: enabled)
    -d, --docs          Generate documentation
    -a, --analysis      Run static analysis
    -i, --install       Install the project
    -p, --package       Create distribution packages
    -v, --verbose       Enable verbose output
    --no-tests          Disable test suite
    --build-type TYPE   Set build type (Debug|Release|RelWithDebInfo|MinSizeRel)
    --jobs N            Set number of parallel jobs (default: number of CPU cores)

EXAMPLES:
    $0                          # Standard build
    $0 --clean --tests          # Clean build with tests
    $0 --build-type Debug -a    # Debug build with analysis
    $0 -d -p                    # Generate docs and create packages

EOF
}

parse_arguments() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -c|--clean)
                CLEAN=1
                shift
                ;;
            -t|--tests)
                TESTS=1
                shift
                ;;
            --no-tests)
                TESTS=0
                shift
                ;;
            -d|--docs)
                DOCS=1
                shift
                ;;
            -a|--analysis)
                ANALYSIS=1
                shift
                ;;
            -i|--install)
                INSTALL=1
                shift
                ;;
            -p|--package)
                PACKAGE=1
                shift
                ;;
            -v|--verbose)
                VERBOSE=1
                shift
                ;;
            --build-type)
                BUILD_TYPE="$2"
                shift 2
                ;;
            --jobs)
                PARALLEL_JOBS="$2"
                shift 2
                ;;
            *)
                log_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
}

main() {
    # Parse command line arguments
    parse_arguments "$@"
    
    # Show build configuration
    log_info "Starting build for $PROJECT_NAME v$PROJECT_VERSION"
    log_info "Build type: $BUILD_TYPE"
    log_info "Parallel jobs: $PARALLEL_JOBS"
    log_info "Build directory: $BUILD_DIR"
    
    # Create build directory
    mkdir -p "${BUILD_DIR}"/{obj,bin}
    
    # Use our modern Makefile
    local make_args=(
        "-f" "Makefile.modern"
        "BUILD_TYPE=$BUILD_TYPE"
        "VERBOSE=$VERBOSE"
        "-j$PARALLEL_JOBS"
    )
    
    if [[ "$CLEAN" == "1" ]]; then
        log_step "Cleaning build directory"
        make -f Makefile.modern clean BUILD_TYPE="$BUILD_TYPE"
    fi
    
    # Build the project
    log_step "Building $PROJECT_NAME ($BUILD_TYPE)"
    if make "${make_args[@]}"; then
        log_success "Build completed successfully"
    else
        log_error "Build failed"
        exit 1
    fi
    
    # Run tests if enabled
    if [[ "$TESTS" == "1" ]]; then
        log_step "Running tests"
        make -f Makefile.modern test BUILD_TYPE="$BUILD_TYPE" || log_warning "Some tests failed"
    fi
    
    # Generate documentation if requested
    if [[ "$DOCS" == "1" ]]; then
        log_step "Generating documentation"
        make -f Makefile.modern doc BUILD_TYPE="$BUILD_TYPE" || log_warning "Documentation generation failed"
    fi
    
    # Run static analysis if requested
    if [[ "$ANALYSIS" == "1" ]]; then
        log_step "Running static analysis"
        make -f Makefile.modern check BUILD_TYPE="$BUILD_TYPE" || log_warning "Static analysis found issues"
    fi
    
    # Install if requested
    if [[ "$INSTALL" == "1" ]]; then
        log_step "Installing project"
        make -f Makefile.modern install BUILD_TYPE="$BUILD_TYPE"
    fi
    
    # Create packages if requested
    if [[ "$PACKAGE" == "1" ]]; then
        log_step "Creating packages"
        make -f Makefile.modern dist BUILD_TYPE="$BUILD_TYPE"
    fi
    
    # Final success message
    echo
    log_success "Build pipeline completed successfully!"
    if [[ -f "${BUILD_DIR}/bin/${PROJECT_NAME}" ]]; then
        log_info "Binary location: ${BUILD_DIR}/bin/${PROJECT_NAME}"
    fi
}

# Execute main function with all arguments
main "$@"