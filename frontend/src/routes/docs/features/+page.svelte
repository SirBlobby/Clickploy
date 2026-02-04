<script lang="ts">
	import { Separator } from "$lib/components/ui/separator";
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
		CardDescription,
	} from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge";
	import {
		Globe,
		Server,
		Terminal,
		Database,
		Settings,
		Shield,
		HardDrive,
	} from "@lucide/svelte";
</script>

<div class="space-y-8">
	<div class="space-y-2">
		<h1 class="text-4xl font-extrabold tracking-tight lg:text-5xl">
			Features
		</h1>
		<p class="text-xl text-muted-foreground">
			Explore the core capabilities of Clickploy and learn how to use them.
		</p>
	</div>
	<Separator />

	<!-- Deployment & Build -->
	<section class="space-y-4">
		<h2 class="text-2xl font-bold tracking-tight">Deployment & Build</h2>
		<div class="grid gap-6 md:grid-cols-2">
			<Card>
				<CardHeader>
					<CardTitle class="flex items-center gap-2">
						<Globe class="h-5 w-5 text-primary" />
						Auto-Build System
					</CardTitle>
					<CardDescription>Zero-config builds using Nixpacks</CardDescription>
				</CardHeader>
				<CardContent class="space-y-2">
					<p>
						Clickploy automatically detects your application's language and framework (Node.js, Python, Go, Rust, etc.) and builds a container image without needing a Dockerfile.
					</p>
					<div class="bg-muted p-3 rounded-md text-sm">
						<strong>How to use:</strong> Simply paste your Git repository URL when creating a new project. Clickploy handles the rest.
					</div>
				</CardContent>
			</Card>

			<Card>
				<CardHeader>
					<CardTitle class="flex items-center gap-2">
						<Server class="h-5 w-5 text-primary" />
						Isolated Containers
					</CardTitle>
					<CardDescription>Docker-based runtime</CardDescription>
				</CardHeader>
				<CardContent class="space-y-2">
					<p>
						Every application runs in its own isolated Docker container. This ensures consistent performance, security, and prevents dependency conflicts between projects.
					</p>
					<div class="bg-muted p-3 rounded-md text-sm">
						<strong>How it works:</strong> The system assigns a unique port and manages the container lifecycle (start, stop, restart) automatically.
					</div>
				</CardContent>
			</Card>
		</div>
	</section>

	<!-- Management & Ops -->
	<section class="space-y-4">
		<h2 class="text-2xl font-bold tracking-tight">Management & Operations</h2>
		<div class="grid gap-6 md:grid-cols-2">
			<Card>
				<CardHeader>
					<CardTitle class="flex items-center gap-2">
						<Terminal class="h-5 w-5 text-primary" />
						Real-time Observability
					</CardTitle>
					<CardDescription>Live logs and deployment history</CardDescription>
				</CardHeader>
				<CardContent class="space-y-2">
					<p>
						Watch your build process and application logs stream in real-time via WebSockets. Access historical logs for any past deployment to debug issues.
					</p>
					<div class="bg-muted p-3 rounded-md text-sm">
						<strong>How to use:</strong> Open a project dashboard. The terminal window shows live logs. Click "History" to view past deployments.
					</div>
				</CardContent>
			</Card>

			<Card>
				<CardHeader>
					<CardTitle class="flex items-center gap-2">
						<Settings class="h-5 w-5 text-primary" />
						Environment Configuration
					</CardTitle>
					<CardDescription>Secrets and Variables</CardDescription>
				</CardHeader>
				<CardContent class="space-y-2">
					<p>
						Manage environment variables (API keys, database URLs) securely. Variables are injected into the container at runtime.
					</p>
					<div class="bg-muted p-3 rounded-md text-sm">
						<strong>How to use:</strong> Go to the <strong>Settings</strong> tab in your project dashboard to add or update variables.
					</div>
				</CardContent>
			</Card>
		</div>
	</section>

	<!-- Storage & Admin -->
	<section class="space-y-4">
		<h2 class="text-2xl font-bold tracking-tight">Storage & Administration</h2>
		<div class="grid gap-6 md:grid-cols-2">
			<Card>
				<CardHeader>
					<CardTitle class="flex items-center gap-2">
						<HardDrive class="h-5 w-5 text-primary" />
						Managed Storage
					</CardTitle>
					<CardDescription>Integrated Databases</CardDescription>
				</CardHeader>
				<CardContent class="space-y-2">
					<p>
						Provision lightweight SQLite databases instantly for your applications. Perfect for prototypes and small-to-medium apps.
					</p>
					<div class="bg-muted p-3 rounded-md text-sm">
						<strong>How to use:</strong> Navigate to the <strong>Storage</strong> page to create and manage databases.
					</div>
				</CardContent>
			</Card>

			<Card>
				<CardHeader>
					<CardTitle class="flex items-center gap-2">
						<Shield class="h-5 w-5 text-primary" />
						Admin Control
					</CardTitle>
					<CardDescription>User Management & Stats</CardDescription>
				</CardHeader>
				<CardContent class="space-y-2">
					<p>
						Administrators have full visibility into system performance, user registration, and global deployment statistics.
					</p>
					<div class="bg-muted p-3 rounded-md text-sm">
						<strong>How to use:</strong> Admin users can access the <strong>Admin</strong> dashboard from the sidebar to manage users and view system health.
					</div>
				</CardContent>
			</Card>
		</div>
	</section>
</div>
