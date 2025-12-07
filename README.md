# Squad Planner

A Rust-based web application for optimizing Football Manager squad selections based on tactical systems and player attributes.

## Project Structure

This is a Cargo workspace with three crates:

- **backend**: Actix-Web REST API server with SQLite database
- **frontend**: Leptos WebAssembly frontend
- **shared**: Shared data models and types

## Prerequisites

- Rust 1.75 or later
- SQLite 3
- Trunk (for frontend builds): `cargo install trunk`
- wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

## Development

### Backend

```bash
cd backend
cargo run
```

The backend API will be available at `http://localhost:8080`

### Frontend

```bash
cd frontend
trunk serve
```

The frontend will be available at `http://localhost:8081`

### Running Tests

```bash
cargo test --workspace
```

## Building for Production

### Backend

```bash
cargo build --release --package backend
```

### Frontend

```bash
cd frontend
trunk build --release
```

## Features

### Phase 1 - MVP (Current)
- Player data upload via CSV
- Formation selection (10 predefined formations)
- Role assignment for each position
- Automatic squad generation based on role suitability
- Interactive pitch visualization
- Basic player management

### Phase 2 - Enhancement (Planned)
- Advanced filtering and sorting
- Squad depth analysis
- Tactical instructions
- Player comparison tools
- Export to PDF/JSON

### Phase 3 - Advanced Features (Planned)
- Budget optimization
- Training recommendations
- Custom formations
- Historical tracking
- Custom roles

### Phase 4 - AI/ML (Future)
- Player potential prediction
- Similar player finder
- Match outcome prediction
- Transfer market integration

## License

MIT
