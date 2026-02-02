import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Search, Star, GitFork, Download, Zap, Globe, Shield } from "lucide-react";

const FEATURED_SKILLS = [
  {
    id: "1",
    name: "react-developer",
    description: "Expert React developer skill for building modern UIs",
    stars: 1247,
    forks: 234,
    language: "TypeScript",
    owner: "facebook",
  },
  {
    id: "2",
    name: "python-data-analyst",
    description: "Data analysis and visualization with Python",
    stars: 892,
    forks: 156,
    language: "Python",
    owner: "pandas-dev",
  },
  {
    id: "3",
    name: "rust-systems",
    description: "Systems programming with Rust",
    stars: 756,
    forks: 89,
    language: "Rust",
    owner: "rust-lang",
  },
];

const STATS = [
  { value: "10,000+", label: "Skills" },
  { value: "50,000+", label: "Downloads" },
  { value: "5,000+", label: "Users" },
  { value: "100+", label: "Countries" },
];

const FEATURES = [
  {
    icon: Zap,
    title: "Instant Integration",
    description: "Install any skill with a single command",
  },
  {
    icon: Globe,
    title: "Global Community",
    description: "Skills from developers worldwide",
  },
  {
    icon: Shield,
    title: "Quality Verified",
    description: "All skills reviewed and tested",
  },
];

export default function HomePage() {
  return (
    <div className="min-h-screen">
      {/* Header */}
      <header className="border-b">
        <div className="container mx-auto px-4 py-4 flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
              <span className="text-white font-bold">S</span>
            </div>
            <span className="text-xl font-bold">SkillHub</span>
          </div>
          <nav className="hidden md:flex items-center space-x-6">
            <Link href="/skills" className="text-muted-foreground hover:text-foreground">
              Browse
            </Link>
            <Link href="/docs" className="text-muted-foreground hover:text-foreground">
              Docs
            </Link>
            <Link href="/admin" className="text-muted-foreground hover:text-foreground">
              Admin
            </Link>
          </nav>
          <div className="flex items-center space-x-4">
            <Link href="/login">
              <Button variant="ghost">Sign In</Button>
            </Link>
            <Link href="/login">
              <Button>Get Started</Button>
            </Link>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <section className="py-20 bg-gradient-to-b from-blue-50 to-white dark:from-blue-950 dark:to-background">
        <div className="container mx-auto px-4 text-center">
          <h1 className="text-5xl font-bold mb-6 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
            Discover Premium AI Skills
          </h1>
          <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto">
            Browse thousands of curated skills for your AI agents. Build powerful agents with just a few clicks.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 max-w-xl mx-auto mb-12">
            <Input placeholder="Search skills..." className="flex-1" />
            <Select defaultValue="all">
              <SelectTrigger className="w-[180px]">
                <SelectValue placeholder="Category" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Categories</SelectItem>
                <SelectItem value="frontend">Frontend</SelectItem>
                <SelectItem value="backend">Backend</SelectItem>
                <SelectItem value="ai">AI/ML</SelectItem>
              </SelectContent>
            </Select>
            <Button size="lg">
              <Search className="w-4 h-4 mr-2" />
              Search
            </Button>
          </div>
          <p className="text-sm text-muted-foreground">
            Popular: React, Python, Rust, TypeScript, Machine Learning
          </p>
        </div>
      </section>

      {/* Stats */}
      <section className="py-12 border-y">
        <div className="container mx-auto px-4">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-8">
            {STATS.map((stat) => (
              <div key={stat.label} className="text-center">
                <div className="text-4xl font-bold text-primary">{stat.value}</div>
                <div className="text-muted-foreground">{stat.label}</div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Featured Skills */}
      <section className="py-20">
        <div className="container mx-auto px-4">
          <div className="flex items-center justify-between mb-8">
            <h2 className="text-3xl font-bold">Trending Skills</h2>
            <Link href="/skills">
              <Button variant="outline">View All</Button>
            </Link>
          </div>
          <div className="grid md:grid-cols-3 gap-6">
            {FEATURED_SKILLS.map((skill) => (
              <Card key={skill.id} className="hover:shadow-lg transition-shadow cursor-pointer">
                <CardHeader>
                  <div className="flex items-start justify-between">
                    <div>
                      <CardTitle className="text-lg">{skill.name}</CardTitle>
                      <CardDescription>by {skill.owner}</CardDescription>
                    </div>
                    <Badge variant="secondary">{skill.language}</Badge>
                  </div>
                </CardHeader>
                <CardContent>
                  <p className="text-muted-foreground mb-4">{skill.description}</p>
                  <div className="flex items-center space-x-4 text-sm text-muted-foreground">
                    <span className="flex items-center">
                      <Star className="w-4 h-4 mr-1" />
                      {skill.stars.toLocaleString()}
                    </span>
                    <span className="flex items-center">
                      <GitFork className="w-4 h-4 mr-1" />
                      {skill.forks.toLocaleString()}
                    </span>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* Features */}
      <section className="py-20 bg-muted/50">
        <div className="container mx-auto px-4">
          <h2 className="text-3xl font-bold text-center mb-12">Why SkillHub?</h2>
          <div className="grid md:grid-cols-3 gap-8">
            {FEATURES.map((feature) => (
              <div key={feature.title} className="text-center">
                <div className="w-16 h-16 bg-primary/10 rounded-full flex items-center justify-center mx-auto mb-4">
                  <feature.icon className="w-8 h-8 text-primary" />
                </div>
                <h3 className="text-xl font-semibold mb-2">{feature.title}</h3>
                <p className="text-muted-foreground">{feature.description}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="py-20">
        <div className="container mx-auto px-4 text-center">
          <h2 className="text-3xl font-bold mb-4">Ready to Build Amazing Agents?</h2>
          <p className="text-muted-foreground mb-8">
            Join thousands of developers building the future of AI.
          </p>
          <Link href="/login">
            <Button size="lg">Get Started Free</Button>
          </Link>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t py-12">
        <div className="container mx-auto px-4">
          <div className="grid md:grid-cols-4 gap-8">
            <div>
              <div className="flex items-center space-x-2 mb-4">
                <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
                  <span className="text-white font-bold">S</span>
                </div>
                <span className="text-xl font-bold">SkillHub</span>
              </div>
              <p className="text-muted-foreground text-sm">
                The marketplace for AI agent skills.
              </p>
            </div>
            <div>
              <h4 className="font-semibold mb-4">Product</h4>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li><Link href="/skills">Browse Skills</Link></li>
                <li><Link href="/docs">Documentation</Link></li>
                <li><Link href="/pricing">Pricing</Link></li>
              </ul>
            </div>
            <div>
              <h4 className="font-semibold mb-4">Company</h4>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li><Link href="/about">About</Link></li>
                <li><Link href="/blog">Blog</Link></li>
                <li><Link href="/careers">Careers</Link></li>
              </ul>
            </div>
            <div>
              <h4 className="font-semibold mb-4">Legal</h4>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li><Link href="/privacy">Privacy</Link></li>
                <li><Link href="/terms">Terms</Link></li>
              </ul>
            </div>
          </div>
          <div className="border-t mt-8 pt-8 text-center text-sm text-muted-foreground">
            © 2024 SkillHub. All rights reserved.
          </div>
        </div>
      </footer>
    </div>
  );
}
