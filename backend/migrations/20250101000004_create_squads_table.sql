-- Create squads table
CREATE TABLE IF NOT EXISTS squads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    formation_id INTEGER NOT NULL,

    -- Squad configuration as JSON
    -- Structure: {starting_xi: [{player_id, position, role_id}], substitutes: [player_id]}
    configuration TEXT NOT NULL, -- JSON object

    -- Overall squad metrics
    average_rating REAL, -- Average suitability rating
    total_value INTEGER, -- Total squad value
    total_wage INTEGER, -- Total weekly wages

    -- Metadata
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (formation_id) REFERENCES formations(id) ON DELETE CASCADE
);

-- Create index on formation_id for filtering squads by formation
CREATE INDEX idx_squads_formation_id ON squads(formation_id);

-- Create index on created_at for sorting by recency
CREATE INDEX idx_squads_created_at ON squads(created_at DESC);
