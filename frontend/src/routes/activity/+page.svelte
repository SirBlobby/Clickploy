<script lang="ts">
	import { onMount } from "svelte";
	import { listActivity } from "$lib/api";
	import { Button } from "$lib/components/ui/button";
	import { Card, CardContent } from "$lib/components/ui/card";
	import {
		Loader2,
		Activity,
		GitCommit,
		ExternalLink,
		Globe,
	} from "@lucide/svelte";

	let activities = $state<any[]>([]);
	let loading = $state(true);

	onMount(async () => {
		const res = await listActivity();
		if (res) activities = res;
		loading = false;
	});

	function getStatusColor(status: string) {
		switch (status) {
			case "live":
				return "bg-green-500";
			case "failed":
				return "bg-red-500";
			case "building":
				return "bg-yellow-500 animate-pulse";
			default:
				return "bg-gray-500";
		}
	}
</script>

<div class="container mx-auto py-8 px-4 max-w-5xl">
	<div class="mb-6 flex items-center justify-between">
		<div class="flex items-center gap-3">
			<div class="p-2 bg-muted rounded-lg border">
				<Activity class="h-5 w-5 text-foreground" />
			</div>
			<div>
				<h1 class="text-2xl font-bold tracking-tight">Activity Feed</h1>
				<p class="text-sm text-muted-foreground">
					Recent deployments across all projects.
				</p>
			</div>
		</div>
	</div>

	{#if loading}
		<div class="flex justify-center p-20">
			<Loader2 class="h-8 w-8 animate-spin text-muted-foreground" />
		</div>
	{:else}
		<Card class="border-border/60">
			<CardContent class="p-0">
				{#if activities.length}
					<div
						class="grid grid-cols-12 gap-4 px-4 py-3 border-b text-xs font-medium text-muted-foreground bg-muted/40 uppercase tracking-wider"
					>
						<div class="col-span-6 md:col-span-3">Project</div>
						<div class="hidden md:block col-span-4">Commit</div>
						<div class="col-span-4 md:col-span-2">Status</div>
						<div class="col-span-2 md:col-span-3 text-right">Time</div>
					</div>

					<div class="divide-y divide-border/40">
						{#each activities as activity}
							<div
								class="grid grid-cols-12 gap-4 px-4 py-3 items-center hover:bg-muted/30 transition-colors text-sm group"
							>
								<div class="col-span-6 md:col-span-3 flex items-center gap-2 overflow-hidden">
									<div
										class="h-8 w-8 rounded bg-primary/10 flex items-center justify-center shrink-0 text-primary uppercase font-bold text-xs ring-1 ring-inset ring-primary/20"
									>
										{activity.project?.name?.substring(0, 2) || "??"}
									</div>
									<a
										href={`/projects/${activity.project_id}`}
										class="font-semibold hover:underline truncate"
									>
										{activity.project?.name || "Unknown"}
									</a>
								</div>

								<div
									class="hidden md:flex col-span-4 items-center gap-2 font-mono text-xs text-muted-foreground"
								>
									<GitCommit class="h-3.5 w-3.5 shrink-0" />
									<span
										class="bg-muted px-1.5 py-0.5 rounded border border-border/50 text-foreground/80 shrink-0"
									>
										{activity.commit ? activity.commit.substring(0, 7) : "HEAD"}
									</span>
									<span class="truncate hidden sm:inline-block opacity-70">
										{activity.commit === "MANUAL"
											? "Manual Redeploy"
											: activity.commit === "WEBHOOK"
												? "Webhook Trigger"
												: "Git Push"}
									</span>
								</div>

								<div class="col-span-4 md:col-span-2">
									<span
										class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium border
										{activity.status === 'live'
											? 'border-green-500/20 bg-green-500/10 text-green-500'
											: activity.status === 'failed'
												? 'border-red-500/20 bg-red-500/10 text-red-500'
												: 'border-yellow-500/20 bg-yellow-500/10 text-yellow-500'}"
									>
										<span
											class="h-1.5 w-1.5 rounded-full {getStatusColor(
												activity.status,
											)}"
										></span>
										<span class="capitalize">{activity.status}</span>
									</span>
								</div>

								<div
									class="col-span-2 md:col-span-3 flex items-center justify-end gap-3 text-right"
								>
									<span class="hidden md:inline text-xs text-muted-foreground">
										{new Date(activity.created_at).toLocaleString(undefined, {
											month: "short",
											day: "numeric",
											hour: "2-digit",
											minute: "2-digit",
										})}
									</span>
									<Button
										variant="ghost"
										size="icon"
										class="h-7 w-7 opacity-0 group-hover:opacity-100 transition-opacity"
										href={`/projects/${activity.project_id}`}
									>
										<ExternalLink class="h-3.5 w-3.5 text-muted-foreground" />
									</Button>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div
						class="p-12 text-center text-muted-foreground flex flex-col items-center justify-center gap-2"
					>
						<Globe class="h-8 w-8 opacity-20" />
						<p>No activity found.</p>
					</div>
				{/if}
			</CardContent>
		</Card>
	{/if}
</div>
