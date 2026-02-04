<script lang="ts">
	import { onMount } from "svelte";
	import { page } from "$app/stores";
	import { getProject, type Project, redeployProject } from "$lib/api";
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
		Play,
		RotateCcw,
	} from "@lucide/svelte";
	import { toast } from "svelte-sonner";

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
				d.id.toString().includes(searchTerm),
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
	async function handleRedeploy(commit?: string) {
		if (!project) return;
		toast.info(commit ? `Redeploying commit ${commit.substring(0, 7)}...` : "Starting redeployment...");
		const success = await redeployProject(project.id.toString(), commit);
		if (success) {
			toast.success("Redeployment started!");
			// Refresh project data to show new deployment
			setTimeout(async () => {
				if (project) {
					const res = await getProject(project.id);
					if (res) project = res;
				}
			}, 1000);
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
			<Button onclick={() => handleRedeploy()}>
				<Play class="mr-2 h-4 w-4" /> Redeploy
			</Button>
		</div>

		<Card class="border-border/60">
			<CardContent class="p-0">
				{#if project.deployments?.length}
					<div
						class="grid grid-cols-12 gap-4 px-4 py-3 border-b text-xs font-medium text-muted-foreground bg-muted/40 uppercase tracking-wider text-center"
					>
						<div class="col-span-4 md:col-span-1">ID</div>
						<div class="col-span-4 md:col-span-2">Status</div>
						<div class="hidden md:block col-span-5">Commit</div>
						<div class="hidden md:block col-span-3">Date</div>
						<div class="col-span-4 md:col-span-1">Actions</div>
					</div>

					<div class="divide-y divide-border/40">
						{#each filteredDeployments as deployment}
							{@const StatusIcon = getStatusIcon(deployment.status)}
							<div
								class="grid grid-cols-12 gap-4 px-4 py-2.5 items-center hover:bg-muted/30 transition-colors text-sm group text-center"
							>
								<div class="col-span-4 md:col-span-1 font-mono text-xs text-muted-foreground">
									#{deployment.id}
								</div>

								<div class="col-span-4 md:col-span-2 flex items-center justify-center gap-2">
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
									class="hidden md:flex col-span-5 items-center justify-center gap-2 font-mono text-xs"
								>
									<GitCommit class="h-3.5 w-3.5 text-muted-foreground" />
									<span
										class="bg-muted px-2 py-0.5 rounded-md border border-border/50 text-foreground/80 font-mono"
										title={deployment.commit}
									>
										{deployment.commit === "HEAD"
											? "HEAD"
											: deployment.commit === "MANUAL"
												? "MANUAL"
												: deployment.commit === "WEBHOOK"
													? "WEBHOOK"
													: deployment.commit.substring(0, 7)}
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

								<div class="hidden md:block col-span-3 text-xs text-muted-foreground">
									{new Date(deployment.created_at).toLocaleString()}
								</div>

								<div
									class="col-span-4 md:col-span-1 flex items-center justify-center gap-1 opacity-100 md:opacity-0 group-hover:opacity-100 transition-opacity"
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
										href={`/projects/${project.id}?deployment=${deployment.id}`}
										title="View Logs"
									>
										<Terminal class="h-3.5 w-3.5" />
									</Button>
									{#if deployment.commit !== "HEAD" && deployment.commit !== "MANUAL" && deployment.commit !== "WEBHOOK"}
										<Button
											variant="ghost"
											size="icon"
											class="h-7 w-7"
											onclick={() => handleRedeploy(deployment.commit)}
											title="Redeploy this version"
										>
											<RotateCcw class="h-3.5 w-3.5" />
										</Button>
									{/if}
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
