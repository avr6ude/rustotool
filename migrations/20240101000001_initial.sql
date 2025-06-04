-- Create pigs table
CREATE TABLE pigs (
    id SERIAL PRIMARY KEY,
    chat_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    weight INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL,
    last_feed DOUBLE PRECISION NOT NULL DEFAULT 0,
    last_salo DOUBLE PRECISION NOT NULL DEFAULT 0,
    owner_name TEXT NOT NULL,
    salo INTEGER NOT NULL DEFAULT 0,
    poisoned BOOLEAN NOT NULL DEFAULT FALSE,
    barn INTEGER NOT NULL DEFAULT 0,
    pigsty INTEGER NOT NULL DEFAULT 0,
    vetclinic INTEGER NOT NULL DEFAULT 0,
    vet_last_pickup DOUBLE PRECISION NOT NULL DEFAULT 0,
    last_weight INTEGER NOT NULL DEFAULT 0,
    avatar_url TEXT,
    biolab INTEGER NOT NULL DEFAULT 0,
    butchery INTEGER NOT NULL DEFAULT 0,
    pills INTEGER NOT NULL DEFAULT 0,
    factory INTEGER NOT NULL DEFAULT 0,
    warehouse INTEGER NOT NULL DEFAULT 0,
    institute INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(chat_id, user_id)
);

-- Create loot table
CREATE TABLE loot (
    id SERIAL PRIMARY KEY,
    chat_id BIGINT NOT NULL,
    owner BIGINT NOT NULL,
    name TEXT NOT NULL,
    icon TEXT NOT NULL,
    description TEXT,
    class_name TEXT NOT NULL,
    class_icon TEXT NOT NULL,
    weight DOUBLE PRECISION NOT NULL DEFAULT 0,
    base_stats JSONB NOT NULL DEFAULT '{}',
    rarity JSONB NOT NULL DEFAULT '{}',
    uuid UUID NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_pigs_chat_user ON pigs(chat_id, user_id);
CREATE INDEX idx_pigs_chat_name ON pigs(chat_id, name);
CREATE INDEX idx_loot_chat_owner ON loot(chat_id, owner);
CREATE INDEX idx_loot_uuid ON loot(uuid);

-- Enable case-insensitive search for names
CREATE INDEX idx_pigs_name_lower ON pigs(chat_id, LOWER(name));