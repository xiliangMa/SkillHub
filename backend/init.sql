-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_skills_stars ON skills(stars DESC);
CREATE INDEX IF NOT EXISTS idx_skills_updated ON skills(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_skills_tags ON skills USING GIN(tags);
CREATE INDEX IF NOT EXISTS idx_payments_user ON payments(user_id);
CREATE INDEX IF NOT EXISTS idx_payments_status ON payments(status);
CREATE INDEX IF NOT EXISTS idx_favorites_user ON favorites(user_id);

-- Insert some sample data for testing
INSERT INTO users (email, name, role) VALUES 
('admin@skillhub.com', 'Admin User', 'admin'),
('test@skillhub.com', 'Test User', 'user')
ON CONFLICT DO NOTHING;
