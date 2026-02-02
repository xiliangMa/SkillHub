import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Separator } from "@/components/ui/separator";
import { ArrowLeft, Star, GitFork, Download, Copy, Check, ExternalLink } from "lucide-react";
import { skillsApi, type Skill } from "@/lib/api";
import { notFound } from "next/navigation";

async function getSkill(id: string) {
  try {
    const skill = await skillsApi.get(id);
    return skill;
  } catch (error) {
    console.error('Failed to fetch skill:', error);
    return null;
  }
}

export default async function SkillDetailPage({
  params,
}: {
  params: { id: string };
}) {
  const skill = await getSkill(params.id);

  if (!skill) {
    notFound();
  }

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="border-b">
        <div className="container mx-auto px-4 py-4 flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <Link href="/skills">
              <Button variant="ghost" size="sm">
                <ArrowLeft className="w-4 h-4 mr-2" />
                Back
              </Button>
            </Link>
          </div>
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
        <div className="grid lg:grid-cols-3 gap-8">
          {/* Main Content */}
          <div className="lg:col-span-2 space-y-6">
            {/* Title Section */}
            <div>
              <div className="flex items-center gap-3 mb-2">
                <h1 className="text-3xl font-bold">{skill.name}</h1>
                {skill.language && (
                  <Badge variant="secondary">{skill.language}</Badge>
                )}
                {skill.marketplace && (
                  <Badge>Verified</Badge>
                )}
              </div>
              <p className="text-muted-foreground">
                by <Link href={`https://github.com/${skill.github_owner}/${skill.github_repo}`} className="text-primary hover:underline">
                  {skill.github_owner}/{skill.github_repo}
                </Link>
              </p>
            </div>

            {/* Stats */}
            <div className="flex items-center gap-6 text-sm">
              <span className="flex items-center">
                <Star className="w-4 h-4 mr-1 text-yellow-500" />
                {skill.stars.toLocaleString()} stars
              </span>
              <span className="flex items-center">
                <GitFork className="w-4 h-4 mr-1" />
                {skill.forks.toLocaleString()} forks
              </span>
              <span className="flex items-center">
                <Download className="w-4 h-4 mr-1" />
                {skill.downloaded_count.toLocaleString()} downloads
              </span>
            </div>

            {/* Description */}
            <p className="text-lg leading-relaxed">
              {skill.description || "No description available for this skill."}
            </p>

            {/* Tabs */}
            <Tabs defaultValue="install">
              <TabsList>
                <TabsTrigger value="install">Install</TabsTrigger>
                <TabsTrigger value="readme">README</TabsTrigger>
                <TabsTrigger value="skill">SKILL.md</TabsTrigger>
              </TabsList>
              
              <TabsContent value="install" className="mt-4">
                <Card>
                  <CardHeader>
                    <CardTitle className="text-lg">Installation</CardTitle>
                  </CardHeader>
                  <CardContent>
                    {skill.install_command ? (
                      <div className="bg-muted rounded-lg p-4 font-mono text-sm">
                        <pre className="whitespace-pre-wrap">{skill.install_command}</pre>
                      </div>
                    ) : (
                      <p className="text-muted-foreground">
                        No installation command available.
                      </p>
                    )}
                  </CardContent>
                </Card>
              </TabsContent>
              
              <TabsContent value="readme" className="mt-4">
                <Card>
                  <CardContent className="pt-6">
                    {skill.readme_content ? (
                      <div className="prose prose-sm max-w-none">
                        <pre className="whitespace-pre-wrap text-sm">{skill.readme_content}</pre>
                      </div>
                    ) : (
                      <p className="text-muted-foreground">
                        No README available for this skill.
                      </p>
                    )}
                  </CardContent>
                </Card>
              </TabsContent>
              
              <TabsContent value="skill" className="mt-4">
                <Card>
                  <CardContent className="pt-6">
                    {skill.skill_content ? (
                      <div className="prose prose-sm max-w-none">
                        <pre className="whitespace-pre-wrap text-sm">{skill.skill_content}</pre>
                      </div>
                    ) : (
                      <p className="text-muted-foreground">
                        No SKILL.md available for this skill.
                      </p>
                    )}
                  </CardContent>
                </Card>
              </TabsContent>
            </Tabs>

            {/* Tags */}
            {skill.tags && skill.tags.length > 0 && (
              <div>
                <h3 className="font-semibold mb-2">Tags</h3>
                <div className="flex flex-wrap gap-2">
                  {skill.tags.map((tag) => (
                    <Badge key={tag} variant="outline">
                      {tag}
                    </Badge>
                  ))}
                </div>
              </div>
            )}
          </div>

          {/* Sidebar */}
          <div className="space-y-6">
            {/* Action Card */}
            <Card>
              <CardContent className="pt-6 space-y-4">
                {skill.price > 0 ? (
                  <div className="text-center">
                    <div className="text-4xl font-bold mb-2">
                      ${skill.price}
                    </div>
                    <p className="text-sm text-muted-foreground mb-4">
                      One-time purchase
                    </p>
                    <Link href={`/login?redirect=/skills/${skill.id}&action=buy`}>
                      <Button className="w-full" size="lg">
                        Buy Now
                      </Button>
                    </Link>
                  </div>
                ) : (
                  <div className="text-center">
                    <p className="text-sm text-muted-foreground mb-4">
                      Free to use
                    </p>
                    <Link href={`/login?redirect=/skills/${skill.id}&action=install`}>
                      <Button className="w-full" size="lg">
                        Install Free
                      </Button>
                    </Link>
                  </div>
                )}
                
                <Separator />
                
                <div className="text-center">
                  <Link 
                    href={`https://github.com/${skill.github_owner}/${skill.github_repo}`}
                    target="_blank"
                    className="text-sm text-primary hover:underline inline-flex items-center"
                  >
                    View on GitHub
                    <ExternalLink className="w-3 h-3 ml-1" />
                  </Link>
                </div>
              </CardContent>
            </Card>

            {/* Info Card */}
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">Information</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Owner</span>
                  <span className="font-medium">{skill.github_owner}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Repository</span>
                  <span className="font-medium">{skill.github_repo}</span>
                </div>
                {skill.language && (
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Language</span>
                    <span className="font-medium">{skill.language}</span>
                  </div>
                )}
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Downloads</span>
                  <span className="font-medium">{skill.downloaded_count.toLocaleString()}</span>
                </div>
                {skill.last_synced_at && (
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Last Updated</span>
                    <span className="font-medium">
                      {new Date(skill.last_synced_at).toLocaleDateString()}
                    </span>
                  </div>
                )}
              </CardContent>
            </Card>
          </div>
        </div>
      </main>
    </div>
  );
}
