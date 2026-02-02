# SkillHub SaaS Platform - Design Document

## 📋 Document Information

| Field | Value |
|-------|-------|
| Version | 1.0 |
| Created | 2024-02-02 |
| Status | Development |
| Platform | macOS |

---

## 🎯 Project Overview

### Mission
Build a professional SaaS platform for discovering, sharing, and distributing AI agent skills, enabling developers to monetize their AI skills and users to easily integrate powerful capabilities into their AI agents.

### Target Audience
- **Skill Creators**: Developers creating AI agent skills
- **Skill Consumers**: Users looking for AI capabilities
- **Enterprise Customers**: Companies needing custom AI solutions

---

## 🏗️ System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Frontend Layer                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │  Web App     │  │   Mobile     │  │   Admin      │  │   Public     │ │
│  │  (Next.js)   │  │   (PWA)      │  │   Dashboard  │  │   Website    │ │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           API Gateway Layer                              │
│  ┌─────────────────────────────────────────────────────────────────────┐ │
│  │                    Rust + Axum HTTP Server                           │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │ │
│  │  │ Skills   │ │ Auth     │ │ Favorites│ │ Payments │ │ Crawler  │  │ │
│  │  │ API      │ │ API      │ │ API      │ │ API      │ │ API      │  │ │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘ └──────────┘  │ │
│  └─────────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    ▼                               ▼
┌─────────────────────────────┐   ┌─────────────────────────────┐
│     Data Layer              │   │     Cache Layer             │
│  ┌─────────────────────┐   │   │  ┌─────────────────────┐    │
│  │   PostgreSQL 15     │   │   │  │   Redis 7           │    │
│  │  ┌───────────────┐  │   │   │  │  - Session Cache    │    │
│  │  │  Users        │  │   │   │  │  - Rate Limiting    │    │
│  │  │  Skills       │  │   │   │  │  - API Cache        │    │
│  │  │  Favorites    │  │   │   │  │  - Job Queue        │    │
│  │  │  Payments     │  │   │   │  └─────────────────────┘    │
│  │  │  Crawler Jobs │  │   │   └─────────────────────────────┘
│  │  └───────────────┘  │   │
│  └─────────────────────┘   │
└─────────────────────────────┘
```

### Technology Stack

#### Frontend
| Technology | Version | Purpose |
|------------|---------|---------|
| Next.js | 14.2 | React Framework |
| TypeScript | 5.x | Type Safety |
| Tailwind CSS | 3.4 | Styling |
| shadcn/ui | Latest | UI Components |
| NextAuth.js | 4.24 | Authentication |
| React Query | 5.x | Data Fetching |
| Lucide React | 0.344 | Icons |

#### Backend
| Technology | Version | Purpose |
|------------|---------|---------|
| Rust | 1.75 | Language |
| Axum | 0.7 | Web Framework |
| Sea-ORM | 0.12 | ORM |
| PostgreSQL | 15 | Primary Database |
| Redis | 7 | Caching & Queue |
| JWT | 9.2 | Authentication |

#### Infrastructure
| Technology | Purpose |
|------------|---------|
| Docker | Containerization |
| Docker Compose | Local Orchestration |
| Nginx | Reverse Proxy |

---

## 📊 Database Schema

### Users Table
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255),
    name VARCHAR(100),
    avatar_url VARCHAR(500),
    provider VARCHAR(20) DEFAULT 'email', -- email, github, google
    provider_id VARCHAR(255),
    role VARCHAR(10) DEFAULT 'user', -- user, admin
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);
```

### Skills Table
```sql
CREATE TABLE skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    github_owner VARCHAR(100) NOT NULL,
    github_repo VARCHAR(100) NOT NULL,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    skill_content TEXT, -- Content of SKILL.md
    readme_content TEXT, -- Content of README.md
    stars INTEGER DEFAULT 0,
    forks INTEGER DEFAULT 0,
    language VARCHAR(50),
    tags JSONB, -- Array of tags
    install_command TEXT,
    price DECIMAL(10, 2) DEFAULT 0, -- 0 = free
    marketplace BOOLEAN DEFAULT false, -- Verified by admin
    downloaded_count INTEGER DEFAULT 0,
    last_synced_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(github_owner, github_repo)
);

-- Indexes
CREATE INDEX idx_skills_stars ON skills(stars DESC);
CREATE INDEX idx_skills_updated ON skills(updated_at DESC);
CREATE INDEX idx_skills_language ON skills(language);
CREATE INDEX idx_skills_tags ON skills USING GIN(tags);
CREATE INDEX idx_skills_price ON skills(price);
```

### Favorites Table
```sql
CREATE TABLE favorites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    skill_id UUID REFERENCES skills(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, skill_id)
);

-- Indexes
CREATE INDEX idx_favorites_user ON favorites(user_id);
CREATE INDEX idx_favorites_skill ON favorites(skill_id);
```

### Payments Table
```sql
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    skill_id UUID REFERENCES skills(id),
    amount DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'CNY',
    status VARCHAR(20) DEFAULT 'pending', -- pending, completed, failed, refunded
    payment_method VARCHAR(20), -- wechat, alipay, stripe
    transaction_id VARCHAR(255),
    qr_code_url TEXT, -- Payment QR code
    expired_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_payments_user ON payments(user_id);
CREATE INDEX idx_payments_status ON payments(status);
CREATE INDEX idx_payments_skill ON payments(skill_id);
```

### Crawler Jobs Table
```sql
CREATE TABLE crawler_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_type VARCHAR(20), -- full, incremental, manual
    status VARCHAR(20) DEFAULT 'pending', -- pending, running, completed, failed
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    repos_found INTEGER DEFAULT 0,
    repos_synced INTEGER DEFAULT 0,
    error_message TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_crawler_status ON crawler_jobs(status);
CREATE INDEX idx_crawler_created ON crawler_jobs(created_at DESC);
```

---

## 🔌 API Design

### API Versioning
- Base Path: `/api/v1/`
- Format: RESTful JSON
- Documentation: Swagger/OpenAPI

### Authentication
- Method: JWT Bearer Token
- Token Expiry: 24 hours
- Refresh Token: Supported

### Endpoints

#### Skills API
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/skills` | List skills with pagination |
| GET | `/api/skills?page=1&limit=20&sort=stars&search=react` | Filtered list |
| GET | `/api/skills/:id` | Get skill details |
| GET | `/api/skills/:id/download` | Download skill files |
| GET | `/api/categories` | Get skill categories |
| GET | `/api/languages` | Get programming languages |
| GET | `/api/trending` | Get trending skills |

#### Authentication API
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/register` | Register new user |
| POST | `/api/auth/login` | Login with email/password |
| POST | `/api/auth/oauth/github` | GitHub OAuth callback |
| POST | `/api/auth/oauth/google` | Google OAuth callback |
| GET | `/api/auth/me` | Get current user |
| POST | `/api/auth/logout` | Logout |
| POST | `/api/auth/refresh` | Refresh token |
| POST | `/api/auth/forgot-password` | Request password reset |
| POST | `/api/auth/reset-password` | Reset password |

#### Favorites API
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/favorites` | Get user's favorites |
| POST | `/api/favorites` | Add to favorites |
| DELETE | `/api/favorites/:id` | Remove from favorites |

#### Payments API
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/payments/create` | Create payment order |
| GET | `/api/payments/:id/status` | Check payment status |
| POST | `/api/payments/:id/refund` | Request refund |
| GET | `/api/payments/history` | Get user's payment history |
| POST | `/api/webhooks/wechat` | WeChat Pay webhook |
| POST | `/api/webhooks/alipay` | Alipay webhook |
| POST | `/api/webhooks/stripe` | Stripe webhook |

#### Admin API
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/admin/setup` | Initial admin setup |
| POST | `/api/admin/crawler/run` | Trigger crawler |
| GET | `/api/admin/crawler/status` | Get crawler status |
| GET | `/api/admin/stats` | Platform statistics |
| GET | `/api/admin/skills` | List all skills (admin) |
| PUT | `/api/admin/skills/:id` | Update skill |
| DELETE | `/api/admin/skills/:id` | Delete skill |
| GET | `/api/admin/users` | List users |
| PUT | `/api/admin/users/:id` | Update user |
| GET | `/api/admin/payments` | List all payments |

---

## 🎨 UI/UX Design

### Design System
- **Framework**: Tailwind CSS
- **Component Library**: shadcn/ui
- **Icons**: Lucide React
- **Design Style**: Modern, Clean, Tech-focused

### Color Palette
```css
:root {
  --primary: #2563eb;       /* Blue */
  --primary-hover: #1d4ed8;
  --secondary: #64748b;     /* Slate */
  --accent: #8b5cf6;        /* Purple */
  --success: #22c55e;       /* Green */
  --warning: #f59e0b;       /* Amber */
  --destructive: #ef4444;   /* Red */
  --background: #ffffff;
  --foreground: #0f172a;
  --muted: #f1f5f9;
  --muted-foreground: #64748b;
}
```

### Typography
- **Headings**: Inter Bold
- **Body**: Inter Regular
- **Monospace**: JetBrains Mono

### Responsive Breakpoints
- `sm`: 640px
- `md`: 768px
- `lg`: 1024px
- `xl`: 1280px
- `2xl`: 1536px

### Page Layouts

#### Homepage
```
┌─────────────────────────────────────────┐
│  Header: Logo + Nav + User Actions      │
├─────────────────────────────────────────┤
│  Hero Section                           │
│  - Headline: "Discover Premium AI       │
│    Skills"                              │
│  - Subheadline                          │
│  - Search Bar                           │
├─────────────────────────────────────────┤
│  Stats Bar                              │
│  - Skills Count | Downloads | Users     │
├─────────────────────────────────────────┤
│  Trending Skills Grid                   │
├─────────────────────────────────────────┤
│  Features Section                       │
├─────────────────────────────────────────┤
│  Footer                                 │
└─────────────────────────────────────────┘
```

#### Skills Listing Page
```
┌─────────────────────────────────────────┐
│  Header                                 │
├─────────────────────────────────────────┤
│  Page Title: Browse Skills              │
├────────────────────────────────────┬────┤
│  Filters Sidebar              │      │
│  - Category Filter            │      │
│  - Language Filter            │ Main │
│  - Price Filter               │Content│
│  - Sort Options               │      │
│                               └──────┘
│  Skills Grid (Responsive)              │
├─────────────────────────────────────────┤
│  Pagination                             │
└─────────────────────────────────────────┘
```

#### Skill Detail Page
```
┌───────────────────────────────────────────────┐
│  Header                                       │
├───────────────────────────────────────────────┤
│  Title + Badges                               │
│  Owner: github_user/repo                      │
├──────────────────────────┬────────────────────┤
│  Description             │  Action Card       │
│  Stats (stars, forks)    │  - Price           │
│  Tags                    │  - Install Button  │
│                          │  - Buy Button      │
├──────────────────────────┴────────────────────┤
│  Tabs: Install | README | SKILL.md            │
├───────────────────────────────────────────────┤
│  Related Skills                               │
└───────────────────────────────────────────────┘
```

#### Admin Dashboard
```
┌─────────────────────────────────────────┐
│  Sidebar Navigation                      │
│  - Dashboard                             │
│  - Skills Management                     │
│  - Crawler Control                       │
│  - User Management                       │
│  - Payment Records                       │
├─────────────────────────────────────────┤
│  Main Content Area                       │
│  - Stats Cards                           │
│  - Tables                                │
│  - Charts                                │
└─────────────────────────────────────────┘
```

---

## 🔧 Integration Details

### GitHub Integration

#### Search API
```bash
# Search for repositories with SKILL.md
GET https://api.github.com/search/repositories
    ?q=SKILL.md+in:path+filename:SKILL.md
    &sort=stars
    &order=desc
    &per_page=100
```

#### Rate Limiting
- Authenticated: 5,000 requests/hour
- Unauthenticated: 60 requests/hour
- Strategy: Multiple tokens, caching

#### Sync Strategy
1. Full sync: Weekly, all skills
2. Incremental sync: Daily, updated skills only
3. Manual sync: On-demand

### Payment Integration

#### WeChat Pay (Native)
1. Create order with amount
2. Get QR code URL
3. Poll for payment status
4. Webhook callback

#### Alipay (Wap)
1. Create order
2. Redirect to Alipay
3. Return with result
4. Webhook callback

#### Stripe
1. Create payment intent
2. Use Stripe Elements
3. Confirm payment
4. Webhook callback

### OAuth Integration

#### GitHub OAuth Flow
1. Redirect to GitHub: `https://github.com/login/oauth/authorize`
2. Get code
3. Exchange for token
4. Get user info
5. Create/update user

#### Google OAuth Flow
1. Redirect to Google
2. Get code
3. Exchange for token
4. Get user info
5. Create/update user

---

## 🚀 Deployment Architecture

### Development Environment
```
Local Machine
├── Frontend: localhost:3000 (Next.js)
├── Backend: localhost:8080 (Rust)
├── PostgreSQL: localhost:5432 (Docker)
└── Redis: localhost:6379 (Docker)
```

### Production Environment
```
┌─────────────────────────────────────────────────────────────┐
│                      Load Balancer (Nginx)                  │
│                    SSL Termination + LB                     │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Frontend   │    │   Backend   │    │   Backend   │
│  Server 1   │    │   Server 1  │    │   Server 2  │
└─────────────┘    └─────────────┘    └─────────────┘
                            │
                            ▼
                   ┌─────────────────┐
                   │  PostgreSQL     │
                   │  (Primary +     │
                   │   Replica)      │
                   └─────────────────┘
                            │
                            ▼
                   ┌─────────────────┐
                   │     Redis       │
                   │  (Cluster)      │
                   └─────────────────┘
```

### Docker Configuration

```yaml
# docker-compose.yml
version: '3.8'

services:
  frontend:
    build: ./frontend
    ports:
      - "3000:3000"
    environment:
      - NEXT_PUBLIC_API_URL=${API_URL}
    depends_on:
      - backend

  backend:
    build: ./backend
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=${DATABASE_URL}
      - REDIS_URL=${REDIS_URL}
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:15-alpine
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_DB=skillhub
      - POSTGRES_USER=${DB_USER}
      - POSTGRES_PASSWORD=${DB_PASSWORD}

  redis:
    image: redis:7-alpine
    command: redis-server --appendonly yes
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

---

## 📈 Performance Requirements

### Target Metrics
| Metric | Target | Measurement |
|--------|--------|-------------|
| API Response Time | < 100ms | p95 |
| Page Load Time | < 2s | First Contentful Paint |
| Database Queries | < 50ms | p95 |
| Crawler Throughput | > 100 repos/min | Concurrent |

### Optimization Strategies
1. **Database**: Indexes, connection pooling, query optimization
2. **Cache**: Redis for frequent queries, CDN for static assets
3. **API**: Compression, pagination, async processing
4. **Frontend**: Code splitting, lazy loading, image optimization

---

## 🔒 Security Requirements

### Authentication Security
- JWT with expiration
- Password hashing: Argon2
- OAuth 2.0 implementation
- Session management

### API Security
- Rate limiting: 100 requests/minute per IP
- Input validation: Zod schemas
- CORS configuration
- SQL injection prevention via ORM

### Payment Security
- HTTPS only
- Webhook signature verification
- Transaction logging
- PCI compliance via payment providers

### Data Security
- Encrypted at rest (database)
- Encrypted in transit (HTTPS)
- Regular backups
- Access logging

---

## 📅 Development Roadmap

### Phase 1: MVP (Week 1-2)
- [x] Project setup
- [x] Database schema
- [x] GitHub crawler
- [x] Basic API
- [x] Frontend website
- [x] User authentication

### Phase 2: Core Features (Week 3-4)
- [ ] Payment integration
- [ ] Admin dashboard
- [ ] Search and filtering
- [ ] Favorites system
- [ ] i18n support

### Phase 3: Polish (Week 5-6)
- [ ] Performance optimization
- [ ] Testing
- [ ] Documentation
- [ ] Deployment pipeline
- [ ] Monitoring setup

---

## 📝 Change Log

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2024-02-02 | Initial design document |

---

## 📚 References

### Documentation
- [Next.js Docs](https://nextjs.org/docs)
- [Axum Docs](https://docs.rs/axum/latest/axum/)
- [Sea-ORM Docs](https://www.sea-ql.org/sea-orm/)
- [Tailwind CSS](https://tailwindcss.com/docs)

### External Services
- [GitHub REST API](https://docs.github.com/en/rest)
- [WeChat Pay](https://pay.weixin.qq.com/wiki/doc/apiv3/index.shtml)
- [Alipay](https://global.alipay.com/docs/ac/global)
- [Stripe](https://docs.stripe.com/)

---

*Document generated for SkillHub SaaS Platform development.*
