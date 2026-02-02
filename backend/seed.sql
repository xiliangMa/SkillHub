-- Default Admin User (password: admin123)
-- Note: Run this SQL directly if the application setup fails

-- First, make sure pgcrypto extension is enabled
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Insert default admin user
INSERT INTO users (id, email, password_hash, name, provider, role, created_at, updated_at)
VALUES (
    'a0000000-0000-0000-0000-000000000001',
    'admin@skillhub.com',
    -- Password: admin123 (argon2 hash)
    '$argon2id$v=19$m=4096,t=3,p=1$xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx$xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
    'Administrator',
    'email',
    'admin',
    NOW(),
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- Insert test user
INSERT INTO users (id, email, password_hash, name, provider, role, created_at, updated_at)
VALUES (
    'b0000000-0000-0000-0000-000000000001',
    'test@skillhub.com',
    -- Password: test123 (argon2 hash)
    '$argon2id$v=19$m=4096,t=3,p=1$yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy$yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy',
    'Test User',
    'email',
    'user',
    NOW(),
    NOW()
) ON CONFLICT (id) DO NOTHING;
