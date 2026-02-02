#!/bin/bash

echo "🚀 SkillHub - Development Mode"
echo "================================"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check what's running
echo "📊 Checking services..."

# Check frontend
if curl -s http://localhost:3000 > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Frontend${NC} - http://localhost:3000"
else
    echo -e "${YELLOW}⚠️  Frontend${NC} - Not running"
    echo "   Run: cd frontend && npm run dev"
fi

# Check backend
echo ""
echo "📝 Backend Status:"
echo "   Rust backend requires Docker or local Rust toolchain"
echo ""
echo "   Option 1 - Docker (if network available):"
echo "      docker compose up -d backend"
echo ""
echo "   Option 2 - Local Rust (after installing Rust):"
echo "      cd backend && cargo run"
echo ""

# Available services
echo "🌐 Access Points:"
echo "   - Frontend: http://localhost:3000"
echo "   - API Docs:  http://localhost:8080/docs (when backend running)"
echo ""

# Project structure
echo "📁 Project Structure:"
echo "   frontend/   - Next.js 14 + TypeScript + shadcn/ui"
echo "   backend/    - Rust + axum + Sea-ORM + PostgreSQL"
echo ""

# Next steps
echo "📋 Next Steps:"
echo "   1. Configure GitHub Token in backend/.env"
echo "   2. Start Docker PostgreSQL: docker compose up -d postgres"
echo "   3. Run backend: cd backend && cargo run"
echo ""
echo "💡 Tip: Frontend is now running! Browse the UI at http://localhost:3000"
