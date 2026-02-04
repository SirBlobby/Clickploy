<script lang="ts">
	import { Separator } from "$lib/components/ui/separator";
	import { Card, CardContent } from "$lib/components/ui/card";
</script>

<div class="space-y-6">
	<div class="space-y-2">
		<h1 class="text-4xl font-extrabold tracking-tight lg:text-5xl">
			Architecture
		</h1>
		<p class="text-xl text-muted-foreground">
			How Clickploy components interact.
		</p>
	</div>
	<Separator />

	<Card>
		<CardContent class="pt-6 space-y-4">
			<p>
				Clickploy operates as a control plane for your deployments. Here's how the
				components interact:
			</p>
			<ul class="list-disc pl-6 space-y-2 text-muted-foreground">
				<li>
					<strong class="text-foreground">Backend (Go):</strong> Handles API
					requests, manages the SQLite database, and orchestrates Docker
					containers.
				</li>
				<li>
					<strong class="text-foreground">Frontend (SvelteKit):</strong> Provides a
					reactive web interface for managing projects and viewing logs.
				</li>
				<li>
					<strong class="text-foreground">Builder (Nixpacks):</strong> Analyzes
					source code and generates OCI-compliant images.
				</li>
				<li>
					<strong class="text-foreground">Reverse Proxy:</strong> (Optional)
					Typically runs behind Nginx or Caddy for SSL termination.
				</li>
			</ul>
		</CardContent>
	</Card>
</div>
