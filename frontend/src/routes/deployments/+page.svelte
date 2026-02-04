<script lang="ts">
	import { onMount } from "svelte";
	import { listActivity } from "$lib/api";
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
	import { Loader2, Rocket, GitCommit, ExternalLink } from "@lucide/svelte";

	let deployments = $state<any[]>([]);
	let loading = $state(true);

	onMount(async () => {
		const res = await listActivity();
		if (res) deployments = res;
		loading = false;
	});
</script>

<div class="container mx-auto py-10 px-4">
	<div class="mb-8 flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight">Deployments</h1>
			<p class="text-muted-foreground">
				View and manage deployments across all your projects.
			</p>
		</div>
	</div>

	{#if loading}
		<div class="flex justify-center p-10">
			<Loader2 class="h-8 w-8 animate-spin" />
		</div>
	{:else}
		<Card>
			<CardHeader>
				<CardTitle>All Deployments</CardTitle>
				<CardDescription>
					A list of recent deployments from all projects.
				</CardDescription>
			</CardHeader>
			<CardContent>
				{#if deployments.length === 0}
					<div
						class="flex flex-col items-center justify-center py-10 text-center"
					>
						<Rocket class="h-10 w-10 text-muted-foreground mb-4" />
						<h3 class="text-lg font-medium">No deployments yet</h3>
						<p class="text-muted-foreground">
							Deploy a project to see it appear here.
						</p>
					</div>
				{:else}
					<Table>
						<TableHeader>
							<TableRow>
								<TableHead>Project</TableHead>
								<TableHead>Commit</TableHead>
								<TableHead>Status</TableHead>
								<TableHead>Created</TableHead>
								<TableHead class="text-right">Actions</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{#each deployments as deploy}
								<TableRow>
									<TableCell class="font-medium">
										{#if deploy.project}
											<a
												href={`/projects/${deploy.project.ID}`}
												class="hover:underline flex items-center gap-2"
											>
												{deploy.project.name}
											</a>
										{:else}
											<span class="text-muted-foreground">Deleted Project</span>
										{/if}
									</TableCell>
									<TableCell>
										<div class="flex items-center gap-2">
											<GitCommit class="h-4 w-4 text-muted-foreground" />
											<span class="font-mono text-sm"
												>{deploy.commit?.substring(0, 7) || "HEAD"}</span
											>
										</div>
									</TableCell>
									<TableCell>
										<span
											class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold
											{deploy.status === 'live'
												? 'border-transparent bg-green-500/15 text-green-500'
												: deploy.status === 'failed'
													? 'border-transparent bg-red-500/15 text-red-500'
													: 'border-transparent bg-yellow-500/15 text-yellow-500'}"
										>
											{deploy.status}
										</span>
									</TableCell>
									<TableCell class="text-muted-foreground">
										{new Date(deploy.CreatedAt).toLocaleDateString()}
										{new Date(deploy.CreatedAt).toLocaleTimeString()}
									</TableCell>
									<TableCell class="text-right">
										<Button
											variant="ghost"
											size="sm"
											href={deploy.url}
											target="_blank"
											disabled={!deploy.url}
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
	{/if}
</div>
