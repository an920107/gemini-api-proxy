# Research: Performance, Scale, and Scope

## Performance Goals

**Decision**: In addition to the 5ms latency constraint for stream parsing, we will adopt the following goals:
- **P99 Latency**: < 200ms for non-streaming requests (proxy overhead).
- **Throughput**: Target 500 requests per second per instance.

**Rationale**: These are common performance targets for high-performance API proxies.

**Alternatives considered**:
- Stricter latency goals (e.g., P99 < 100ms) were considered but deemed unnecessary for the current use case.

## Scale and Scope

**Decision**:
- **Scale**: The system should be designed to handle up to 10 million requests per day.
- **Scope**: The proxy will focus on serving a single model provider initially, with the potential to expand to others in the future.

**Rationale**: This provides a clear target for capacity planning and architecture decisions.

**Alternatives considered**:
- Designing for a much larger scale (e.g., 100 million requests/day) was rejected as it would introduce unnecessary complexity at this stage.
