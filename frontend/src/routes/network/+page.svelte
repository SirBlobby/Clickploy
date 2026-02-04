<script lang="ts">
	import { onMount } from "svelte";
	import { listProjects, type Project } from "$lib/api";
	import { Button } from "$lib/components/ui/button";
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
		CardDescription,
	} from "$lib/components/ui/card";
	import {
		Table,
		TableBody,
		TableCell,
		TableHead,
		TableHeader,
		TableRow,
	} from "$lib/components/ui/table";
	import {
		Loader2,
		Network,
		ExternalLink,
		Globe,
		Server,
	} from "@lucide/svelte";

	let projects = $state<Project[]>([]);
	let loading = $state(true);

	onMount(async () => {
		const res = await listProjects();
		if (res) projects = res;
		loading = false;
	});
</script>

<div class="container mx-auto py-10 px-4">
	<div class="mb-8 flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight">Network</h1>
			<p class="text-muted-foreground">
				Active services and network configuration.
			</p>
		</div>
	</div>

	{#if loading}
		<div class="flex justify-center p-10">
			<Loader2 class="h-8 w-8 animate-spin" />
		</div>
	{:else}
		<div class="grid gap-6">
			<Card>
				<CardHeader>
					<CardTitle>Active Services</CardTitle>
					<CardDescription>
						Overview of deployed applications and their internal/external ports.
					</CardDescription>
				</CardHeader>
				<CardContent>
					{#if projects.length === 0}
						<div
							class="flex flex-col items-center justify-center py-10 text-center"
						>
							<Network class="h-10 w-10 text-muted-foreground mb-4" />
							<h3 class="text-lg font-medium">No services found</h3>
							<p class="text-muted-foreground">
								Deploy a project to populate the network map.
							</p>
						</div>
					{:else}
						<Table>
							<TableHeader>
								<TableRow>
									<TableHead>Service</TableHead>
									<TableHead>Host</TableHead>
									<TableHead>Port</TableHead>
									<TableHead>URL</TableHead>
									<TableHead>Status</TableHead>
									<TableHead class="text-right">Action</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{#each projects as project}
									<TableRow>
										<TableCell class="font-medium">
											<div class="flex items-center gap-2">
												<Server class="h-4 w-4 text-muted-foreground" />
												<a
													href={`/projects/${project.ID}`}
													class="hover:underline"
												>
													{project.name}
												</a>
											</div>
										</TableCell>
										<TableCell class="font-mono text-xs">localhost</TableCell>
										<TableCell class="font-mono text-xs"
											>{project.port}</TableCell
										>
										<TableCell class="font-mono text-xs text-muted-foreground">
											http://localhost:{project.port}
										</TableCell>
										<TableCell>
											<span
												class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold
												bg-green-500/15 text-green-500 border-transparent"
											>
												Active
											</span>
										</TableCell>
										<TableCell class="text-right">
											<Button
												variant="ghost"
												size="sm"
												href={`http://localhost:${project.port}`}
												target="_blank"
											>
												<ExternalLink class="h-4 w-4" />
											</Button>
										</TableCell>
									</TableRow>
								{/each}
							</TableBody>
						</Table>
					{/if}
				</CardContent>
			</Card>
		</div>
	{/if}
</div>
