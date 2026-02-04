<script lang="ts">
	import { onMount } from "svelte";
	import { page } from "$app/stores";
	import { getProject, type Project } from "$lib/api";
	import { Button } from "$lib/components/ui/button";
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
		CardDescription,
	} from "$lib/components/ui/card";
	import {
		Loader2,
		ExternalLink,
		GitCommit,
		Search,
		RefreshCw,
		Terminal,
		CheckCircle2,
		XCircle,
		AlertCircle,
	} from "@lucide/svelte";

	let project = $state<Project | null>(null);
	let loading = $state(true);
	let searchTerm = $state("");

	onMount(async () => {
		const id = $page.params.id;
		if (id) {
			const res = await getProject(id);
			if (res) project = res;
		}
		loading = false;
	});

	let filteredDeployments = $derived(
		project?.deployments?.filter(
			(d) =>
				d.commit.toLowerCase().includes(searchTerm.toLowerCase()) ||
				d.status.toLowerCase().includes(searchTerm.toLowerCase()) ||
				d.ID.toString().includes(searchTerm),
		) || [],
	);

	function getStatusColor(status: string) {
		switch (status) {
			case "live":
				return "text-green-500";
			case "failed":
				return "text-red-500";
			case "building":
				return "text-yellow-500";
			default:
				return "text-muted-foreground";
		}
	}

	function getStatusIcon(status: string) {
		switch (status) {
			case "live":
				return CheckCircle2;
			case "failed":
				return XCircle;
			case "building":
				return RefreshCw;
			default:
				return AlertCircle;
		}
	}
</script>

{#if loading}
	<div class="flex justify-center p-10">
		<Loader2 class="h-8 w-8 animate-spin" />
	</div>
{:else if project}
	<div class="space-y-4">
		<div class="flex justify-between items-center">
			<div>
				<h2 class="text-xl font-bold tracking-tight">Deployments</h2>
				<p class="text-sm text-muted-foreground">
					History of your application builds.
				</p>
			</div>
		</div>

		<Card class="border-border/60">
			<CardContent class="p-0">
				{#if project.deployments?.length}
					<div
						class="grid grid-cols-12 gap-4 px-4 py-3 border-b text-xs font-medium text-muted-foreground bg-muted/40 uppercase tracking-wider"
					>
						<div class="col-span-1">ID</div>
						<div class="col-span-2">Status</div>
						<div class="col-span-5">Commit</div>
						<div class="col-span-3">Date</div>
						<div class="col-span-1 text-right">Actions</div>
					</div>

					<div class="divide-y divide-border/40">
						{#each filteredDeployments as deployment}
							{@const StatusIcon = getStatusIcon(deployment.status)}
							<div
								class="grid grid-cols-12 gap-4 px-4 py-2.5 items-center hover:bg-muted/30 transition-colors text-sm group"
							>
								<div class="col-span-1 font-mono text-xs text-muted-foreground">
									#{deployment.ID}
								</div>

								<div class="col-span-2 flex items-center gap-2">
									<StatusIcon
										class="h-4 w-4 {getStatusColor(
											deployment.status,
										)} {deployment.status === 'building' ? 'animate-spin' : ''}"
									/>
									<span
										class="capitalize font-medium {getStatusColor(
											deployment.status,
										)} text-xs"
									>
										{deployment.status}
									</span>
								</div>

								<div
									class="col-span-5 flex items-center gap-2 font-mono text-xs"
								>
									<GitCommit class="h-3.5 w-3.5 text-muted-foreground" />
									<span
										class="bg-muted px-1.5 py-0.5 rounded border border-border/50 text-foreground/80"
									>
										{deployment.commit
											? deployment.commit.substring(0, 7)
											: "HEAD"}
									</span>
									<span
										class="text-muted-foreground truncate hidden md:inline-block max-w-[200px]"
									>
										{deployment.commit === "MANUAL"
											? "Manual Redeploy"
											: deployment.commit === "WEBHOOK"
												? "Webhook Trigger"
												: "Git Push"}
									</span>
								</div>

								<div class="col-span-3 text-xs text-muted-foreground">
									{new Date(deployment.CreatedAt).toLocaleString()}
								</div>

								<div
									class="col-span-1 flex items-center justify-end gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
								>
									{#if deployment.status === "live"}
										<Button
											variant="ghost"
											size="icon"
											class="h-7 w-7"
											href={deployment.url}
											target="_blank"
											title="Visit App"
										>
											<ExternalLink class="h-3.5 w-3.5" />
										</Button>
									{/if}
									<Button
										variant="ghost"
										size="icon"
										class="h-7 w-7"
										href={`/projects/${project.ID}?deployment=${deployment.ID}`}
										title="View Logs"
									>
										<Terminal class="h-3.5 w-3.5" />
									</Button>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div
						class="p-12 text-center flex flex-col items-center justify-center text-muted-foreground gap-2"
					>
						<Search class="h-8 w-8 opacity-20" />
						<p>No deployments found.</p>
					</div>
				{/if}
			</CardContent>
		</Card>
	</div>
{/if}
