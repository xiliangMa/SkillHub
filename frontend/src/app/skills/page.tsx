import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Search, Star, GitFork, Download, ExternalLink } from "lucide-react";
import { skillsApi, type Skill } from "@/lib/api";

async function getSkills(search?: string, language?: string) {
  try {
    const response = await skillsApi.list({
      search,
      language,
      page: 1,
      limit: 20,
    });
    return response.data;
  } catch (error) {
    console.error('Failed to fetch skills:', error);
    return [];
  }
}

export default async function SkillsPage({
  searchParams,
}: {
  searchParams: { search?: string; language?: string };
}) {
  const skills = await getSkills(searchParams.search, searchParams.language);

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="border-b sticky top-0 bg-background/95 backdrop-blur z-10">
        <div className="container mx-auto px-4 py-4 flex items-center justify-between">
          <Link href="/" className="flex items-center space-x-2">
            <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
              <span className="text-white font-bold">S</span>
            </div>
            <span className="text-xl font-bold">SkillHub</span>
          </Link>
          <nav className="hidden md:flex items-center space-x-6">
            <Link href="/skills" className="text-foreground font-medium">Browse</Link>
            <Link href="/docs" className="text-muted-foreground hover:text-foreground">Docs</Link>
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

      <main className="container mx-auto px-4 py-8">
        {/* Search and Filters */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold mb-6">Browse Skills</h1>
          <form className="flex flex-col sm:flex-row gap-4">
            <div className="flex-1 flex gap-2">
              <Input
                name="search"
                placeholder="Search skills..."
                defaultValue={searchParams.search}
                className="flex-1"
              />
              <Select name="language" defaultValue={searchParams.language || "all"}>
                <SelectTrigger className="w-[180px]">
                  <SelectValue placeholder="Language" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All Languages</SelectItem>
                  <SelectItem value="TypeScript">TypeScript</SelectItem>
                  <SelectItem value="Python">Python</SelectItem>
                  <SelectItem value="Rust">Rust</SelectItem>
                  <SelectItem value="JavaScript">JavaScript</SelectItem>
                  <SelectItem value="Go">Go</SelectItem>
                </SelectContent>
              </Select>
              <Button type="submit">
                <Search className="w-4 h-4 mr-2" />
                Search
              </Button>
            </div>
          </form>
        </div>

        {/* Skills Grid */}
        {skills.length > 0 ? (
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {skills.map((skill) => (
              <Link href={`/skills/${skill.id}`} key={skill.id}>
                <Card className="h-full hover:shadow-lg transition-all cursor-pointer group">
                  <CardHeader>
                    <div className="flex items-start justify-between">
                      <div className="space-y-1">
                        <CardTitle className="text-lg group-hover:text-primary transition-colors">
                          {skill.name}
                        </CardTitle>
                        <CardDescription className="text-xs">
                          by {skill.github_owner}/{skill.github_repo}
                        </CardDescription>
                      </div>
                      {skill.language && (
                        <Badge variant="secondary">{skill.language}</Badge>
                      )}
                    </div>
                  </CardHeader>
                  <CardContent>
                    <p className="text-muted-foreground text-sm mb-4 line-clamp-2">
                      {skill.description || "No description available"}
                    </p>
                    
                    {skill.tags && skill.tags.length > 0 && (
                      <div className="flex flex-wrap gap-1 mb-4">
                        {skill.tags.slice(0, 3).map((tag) => (
                          <Badge key={tag} variant="outline" className="text-xs">
                            {tag}
                          </Badge>
                        ))}
                        {skill.tags.length > 3 && (
                          <Badge variant="outline" className="text-xs">
                            +{skill.tags.length - 3}
                          </Badge>
                        )}
                      </div>
                    )}

                    <div className="flex items-center justify-between text-sm text-muted-foreground">
                      <div className="flex items-center space-x-4">
                        <span className="flex items-center">
                          <Star className="w-4 h-4 mr-1 text-yellow-500" />
                          {skill.stars.toLocaleString()}
                        </span>
                        <span className="flex items-center">
                          <GitFork className="w-4 h-4 mr-1" />
                          {skill.forks.toLocaleString()}
                        </span>
                        <span className="flex items-center">
                          <Download className="w-4 h-4 mr-1" />
                          {skill.downloaded_count.toLocaleString()}
                        </span>
                      </div>
                      {skill.price > 0 && (
                        <span className="font-semibold text-primary">
                          ${skill.price}
                        </span>
                      )}
                    </div>
                  </CardContent>
                </Card>
              </Link>
            ))}
          </div>
        ) : (
          <div className="text-center py-20">
            <div className="text-6xl mb-4">🔍</div>
            <h3 className="text-xl font-semibold mb-2">No skills found</h3>
            <p className="text-muted-foreground mb-4">
              {searchParams.search
                ? `No results for "${searchParams.search}"`
                : "No skills available yet. Check back soon!"}
            </p>
            <Link href="/">
              <Button variant="outline">Go Home</Button>
            </Link>
          </div>
        )}

        {/* Pagination */}
        {skills.length > 0 && (
          <div className="mt-8 flex justify-center gap-2">
            <Button variant="outline" disabled>Previous</Button>
            <Button variant="outline" disabled>Next</Button>
          </div>
        )}
      </main>
    </div>
  );
}
