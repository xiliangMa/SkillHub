# SkillHub - AI Skills Marketplace

A professional SaaS platform for discovering and distributing AI agent skills.

## 🎉 Quick Start

### Frontend (Currently Running!)

```bash
cd frontend
npm run dev
# Access: http://localhost:3000
```

### Backend

**Option 1: Docker (Recommended when network available)**
```bash
# Configure environment
cp backend/.env.example backend/.env
# Edit backend/.env and add your GitHub token

# Start services
docker compose up -d postgres redis
docker compose up -d backend
# Access: http://localhost:8080
```

**Option 2: Local Rust**
```bash
# Install Rust first: https://rustup.rs/
cd backend
cargo run
# Access: http://localhost:8080
```

## 👤 Admin Account Setup

### First Time Setup

1. Start the backend service (requires PostgreSQL)
2. Visit: **http://localhost:3000/setup**
3. Create your admin account

### Default Credentials (After Setup)

| Field | Value |
|-------|-------|
| Email | `admin@skillhub.com` |
| Password | `admin123` |

### API Setup

```bash
# Create admin via API
curl -X POST http://localhost:8080/api/admin/setup \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@skillhub.com",
    "password": "admin123",
    "name": "Administrator"
  }'
```

### Database Direct Setup

```bash
# Connect to PostgreSQL
docker compose exec postgres psql -U skillhub -d skillhub

# Run seed script
\i /docker-entrypoint-initdb.d/seed.sql
# Or manually run the INSERT statements from backend/seed.sql
```

## ✨ Features

- 🐙 **GitHub Integration**: Auto-sync skills from repositories containing SKILL.md
- 🔍 **Powerful Search**: Find by name, language, and tags
- 💳 **Payment Support**: WeChat Pay, Alipay, Stripe
- 🌐 **i18n**: English and Chinese
- 📱 **Responsive**: Desktop and mobile support
- 🔒 **Auth**: Email, GitHub, Google OAuth

## 📁 Project Structure

```
SkillHub/
├── frontend/              # Next.js 14 + TypeScript
│   ├── src/app/          # Pages (home, skills, login)
│   ├── src/components/   # UI components (shadcn/ui)
│   ├── src/lib/          # API client
│   └── src/locales/      # i18n (en, zh)
│
├── backend/               # Rust + axum
│   ├── api/              # HTTP handlers
│   ├── entity/           # Sea-ORM entities
│   ├── jobs/             # GitHub crawler
│   └── migration/        # DB migrations
│
└── docker-compose.yml    # Docker deployment
```

## 🔧 Configuration

### Environment Variables

**backend/.env**
```env
SKILLHUB_GITHUB_TOKEN=ghp_your_token
SKILLHUB_JWT_SECRET=your-secret
SKILLHUB_DATABASE_URL=postgresql://...
```

**frontend/.env**
```env
NEXT_PUBLIC_API_URL=http://localhost:8080
GITHUB_CLIENT_ID=...
GITHUB_CLIENT_SECRET=...
```

## 📡 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/skills | List skills |
| GET | /api/skills/:id | Skill details |
| POST | /api/auth/register | Register |
| POST | /api/auth/login | Login |
| GET | /api/favorites | User favorites |
| POST | /api/admin/crawler/run | Trigger crawler |

## 🛠️ Tech Stack

### Frontend
- Next.js 14 + TypeScript
- Tailwind CSS + shadcn/ui
- NextAuth.js (GitHub, Google)
- Axios + React Query

### Backend
- Rust + axum
- Sea-ORM + PostgreSQL
- Redis (caching, sessions)
- JWT authentication

### Infrastructure
- Docker + Docker Compose
- PostgreSQL 15
- Redis 7

## 📝 Commands

```bash
# Development
cd frontend && npm run dev    # Frontend
cd backend && cargo run       # Backend (needs Rust)

# Docker
docker compose up -d         # All services
docker compose logs -f       # View logs
docker compose down          # Stop all

# Database
docker compose exec postgres psql -U skillhub -d skillhub
```

## 🚀 Deployment

```bash
# Production build
docker compose -f docker-compose.yml up -d --build
```

## 📄 License

MIT
