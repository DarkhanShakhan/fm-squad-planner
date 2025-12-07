-- Create tactics table
CREATE TABLE IF NOT EXISTS tactics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,

    -- Tactical settings
    mentality TEXT NOT NULL CHECK(mentality IN ('Very Defensive', 'Defensive', 'Cautious', 'Balanced', 'Positive', 'Attacking', 'Very Attacking')),
    width TEXT NOT NULL CHECK(width IN ('Very Narrow', 'Narrow', 'Standard', 'Wide', 'Very Wide')),
    tempo TEXT NOT NULL CHECK(tempo IN ('Very Slow', 'Slow', 'Standard', 'Fast', 'Very Fast')),
    pressing_intensity TEXT NOT NULL CHECK(pressing_intensity IN ('Much Less', 'Less', 'Standard', 'More', 'Much More')),
    defensive_line TEXT NOT NULL CHECK(defensive_line IN ('Much Deeper', 'Deeper', 'Standard', 'Higher', 'Much Higher')),

    -- Additional instructions as JSON (optional)
    -- Structure: {instruction_name: boolean}
    team_instructions TEXT, -- JSON object

    -- Associated squad (optional)
    squad_id INTEGER,

    -- Metadata
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (squad_id) REFERENCES squads(id) ON DELETE SET NULL
);

-- Create index on squad_id for finding tactics by squad
CREATE INDEX idx_tactics_squad_id ON tactics(squad_id);

-- Create index on mentality for filtering
CREATE INDEX idx_tactics_mentality ON tactics(mentality);
