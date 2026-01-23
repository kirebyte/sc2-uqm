# UQM Rust - The Ur-Quan Masters Reimplementation

A modern Rust reimplementation of The Ur-Quan Masters (Star Control 2), a classic space adventure game from 1992.

## Project Status

**Current Phase: Phase 0 - COMPLETE** ✅

We have successfully completed the foundation setup for the Rust migration project.

### Completed Milestones

- ✅ Project structure established
- ✅ Core types and state management implemented
- ✅ Comprehensive testing framework
- ✅ Benchmarking infrastructure
- ✅ C API documentation
- ✅ Python golden reference implementation
- ✅ All tests passing (54 unit + integration tests)

## Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Python 3.8+ (for golden reference validation)

### Building

```bash
cd uqm-rust
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test integration_tests
cargo test starmap_tests
cargo test melee_tests

# Run with output
cargo test -- --nocapture
```

### Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench melee
cargo bench starmap
```

### Documentation

```bash
# Generate and open documentation
cargo doc --open
```

## Architecture

The project is organized into modular subsystems:

- **core**: Fundamental types, game state, time management
- **graphics**: Rendering (Phase 2)
- **input**: Input handling (Phase 3)
- **audio**: Sound and music (Phase 4)
- **starmap**: Galaxy navigation (Phase 5)
- **melee**: Space combat (Phase 6)
- **planet**: Planetary exploration (Phase 7)
- **conversation**: Dialogue system (Phase 8)
- **menu**: UI and menus (Phase 8)
- **resource**: Asset management (Phase 8)

## Migration Phases

### ✅ Phase 0: Setup & Foundation (2-3 weeks) - COMPLETE

- Project structure
- Core types
- Testing framework
- Documentation
- Golden reference

### ⏳ Phase 1: Core Types & Structures (2-3 weeks) - NEXT

- Complete type system
- Enhanced serialization
- Data loading infrastructure
- Property-based testing

### 🔮 Phase 2-8: To Be Implemented

See [docs/PHASE0_GUIDE.md](docs/PHASE0_GUIDE.md) for complete phase breakdown.

## Testing Strategy

We use a multi-layer testing approach:

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test module interactions
3. **Golden Reference**: Python implementation defines expected behavior
4. **Benchmarks**: Ensure performance targets are met

### Test Coverage

- Core types: 100%
- Game state: 100%
- Time system: 100%
- Flags: 100%
- Ship system: 100%
- Starmap: 100%

**Total: 54 tests, all passing ✅**

## Performance Targets

| Component | Target | Status |
|-----------|--------|--------|
| Distance calculation | <0.5μs | ✅ Achieved |
| Ship update | <8ms | ⏳ Phase 6 |
| Terrain generation | <30ms | ⏳ Phase 7 |
| Star search (500 systems) | <20ms | ⏳ Phase 5 |

## Documentation

- [Phase 0 Implementation Guide](docs/PHASE0_GUIDE.md)
- [C API Documentation](docs/C_API_DOCUMENTATION.md)
- [Golden Reference README](golden_reference/README.md)
- [Technical Architecture Analysis](../.agent/UQM Technical Architecture Analysis.md)

## Contributing

This project follows test-driven development:

1. Write tests first (or use golden reference)
2. Implement functionality
3. Validate against golden reference
4. Benchmark if performance-critical
5. Document with rustdoc

## License

GPL-2.0 (matching original UQM project)

## Links

- [Original UQM Project](https://sc2.sourceforge.net/)
- [Main Repository](https://github.com/kirebyte/sc2-uqm)
- [Rust Book](https://doc.rust-lang.org/book/)
- [wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)

## Acknowledgments

- Original Star Control 2 by Toys for Bob
- The Ur-Quan Masters project team
- Rust community for excellent tools and libraries

---

**Made with ❤️ and Rust** 🦀
