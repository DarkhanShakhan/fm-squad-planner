-- Create formations table
CREATE TABLE IF NOT EXISTS formations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE, -- e.g., "4-4-2", "4-3-3", "4-2-3-1"
    description TEXT,

    -- Formation configuration as JSON
    -- Structure: array of positions with {position: string, x: number, y: number, role?: string}
    positions TEXT NOT NULL, -- JSON array

    -- Metadata
    is_custom BOOLEAN DEFAULT 0, -- 0 for predefined, 1 for user-created
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create index on name for faster lookups
CREATE INDEX idx_formations_name ON formations(name);

-- Create index on is_custom to separate predefined from custom formations
CREATE INDEX idx_formations_is_custom ON formations(is_custom);
