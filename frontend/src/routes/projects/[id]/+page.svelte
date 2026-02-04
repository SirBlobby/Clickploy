<script lang="ts">
	import { onMount, tick } from "svelte";
	import { page } from "$app/stores";
	import { getProject, type Project, redeployProject, stopProject } from "$lib/api";
	import { Button } from "$lib/components/ui/button";
	import { Card } from "$lib/components/ui/card";
	import {
		Loader2,
		ExternalLink,
		RefreshCw,
		Activity,
		Terminal,
		Settings,
		Play,
		Check,
		Copy,
		GitCommit,
		Square,
	} from "@lucide/svelte";
	import { toast } from "svelte-sonner";

	let project = $state<Project | null>(null);
	let loading = $state(true);
	let latestDeployment = $derived(project?.deployments?.[0]);
	let status = $derived(latestDeployment?.status || "unknown");

	let activeDeploymentLogs = $state("");
	let activeDeploymentId = $state<string | null>(null);
	let ws = $state<WebSocket | null>(null);
	let logContentRef = $state<HTMLDivElement | null>(null);
	let copied = $state(false);

	let autoScroll = $state(true);
	let userScrolled = $state(false);

	onMount(async () => {
		loadProject();
	});

	async function loadProject() {
		const id = $page.params.id;
		if (id) {
			const res = await getProject(id);
			if (res) {
				project = res;
				if (
					!activeDeploymentId &&
					res.deployments &&
					res.deployments.length > 0
				) {
					selectDeployment(res.deployments[0]);
				}
			}
		}
		loading = false;
	}

	async function refresh() {
		loading = true;
		await loadProject();
		loading = false;
	}

	async function handleRedeploy() {
		if (!project) return;
		toast.info("Starting redeployment...");
		const success = await redeployProject(project.id.toString());
		if (success) {
			toast.success("Redeployment started!");
			setTimeout(loadProject, 1000);
		}
	}

	async function handleStop() {
		if (!project) return;
		toast.info("Stopping project...");
		const success = await stopProject(project.id.toString());
		if (success) {
			setTimeout(loadProject, 1000);
		}
	}

	function selectDeployment(deployment: any) {
		if (activeDeploymentId === deployment.id) return;

		activeDeploymentId = deployment.id;
		activeDeploymentLogs = deployment.logs || "";
		userScrolled = false;
		autoScroll = true;
		scrollToBottom(true);

		if (deployment.status === "building") {
			connectWebSocket(deployment.id);
		} else {
			if (ws) {
				ws.close();
				ws = null;
			}
		}
	}

	function connectWebSocket(deploymentId: string) {
		if (ws) ws.close();
		const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
		ws = new WebSocket(
			`${protocol}//${window.location.hostname}:8080/api/deployments/${deploymentId}/logs/stream`,
		);

		ws.onmessage = (event) => {
			activeDeploymentLogs += event.data;
			if (autoScroll) {
				scrollToBottom();
			}
		};

		ws.onclose = () => {
			console.log("Log stream closed");
		};
	}

	async function scrollToBottom(force = false) {
		await tick();
		if (logContentRef) {
			logContentRef.scrollTop = logContentRef.scrollHeight;
		}
	}

	function handleScroll() {
		if (!logContentRef) return;

		const { scrollTop, scrollHeight, clientHeight } = logContentRef;
		const isAtBottom = scrollHeight - scrollTop - clientHeight < 50;

		if (!isAtBottom) {
			userScrolled = true;
			autoScroll = false;
		} else {
			userScrolled = false;
			autoScroll = true;
		}
	}

	function copyLogs() {
		navigator.clipboard.writeText(activeDeploymentLogs);
		copied = true;
		setTimeout(() => (copied = false), 2000);
		toast.success("Logs copied to clipboard");
	}
</script>

{#if loading}
	<div class="flex justify-center items-center h-[50vh]">
		<Loader2 class="h-8 w-8 animate-spin text-muted-foreground" />
	</div>
{:else if project}
	<div class="space-y-4 lg:h-[calc(100vh-140px)] flex flex-col lg:overflow-hidden">
		<div class="shrink-0 space-y-4">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<h2 class="text-xl font-bold tracking-tight">{project.name}</h2>
					<a
						href={project.repo_url}
						target="_blank"
						class="text-muted-foreground hover:text-primary transition-colors text-xs flex items-center gap-1 border px-2 py-0.5 rounded-full"
					>
						<GitCommit class="h-3 w-3" />
						{project.repo_url.split("/").pop()?.replace(".git", "")}
					</a>
				</div>
				<div class="flex gap-2">
					<Button
						variant="ghost"
						size="sm"
						class="h-8"
						onclick={refresh}
						title="Refresh Project"
					>
						<RefreshCw class="h-3.5 w-3.5" />
					</Button>
					<Button
						variant="outline"
						size="sm"
						class="h-8"
						onclick={handleRedeploy}
						disabled={status === "building"}
					>
						{#if status === "building"}
							<Loader2 class="h-3.5 w-3.5 mr-2 animate-spin" /> Redeploying
						{:else}
							<Play class="h-3.5 w-3.5 mr-2" /> Redeploy
						{/if}
					</Button>
					{#if status === "live" || status === "building"}
						<Button
							variant="outline"
							size="sm"
							class="h-8"
							onclick={handleStop}
						>
							<Square class="h-3.5 w-3.5 mr-2" /> Stop
						</Button>
					{/if}
					<Button
						href={latestDeployment?.url}
						target="_blank"
						disabled={status !== "live"}
						size="sm"
						class="h-8"
					>
						<ExternalLink class="mr-2 h-3.5 w-3.5" /> Visit
					</Button>
				</div>
			</div>

			<div
				class="grid grid-cols-2 lg:grid-cols-4 gap-px bg-border/40 rounded-lg overflow-hidden border"
			>
				<div
					class="bg-card p-3 flex flex-col justify-center items-center relative overflow-hidden group"
				>
					<div
						class="absolute inset-0 opacity-5 transition-opacity group-hover:opacity-10 {status ===
						'live'
							? 'bg-green-500'
							: status === 'failed'
								? 'bg-red-500'
								: 'bg-yellow-500'}"
					></div>
					<div
						class="text-[10px] uppercase tracking-wider text-muted-foreground font-semibold mb-0.5"
					>
						Status
					</div>
					<div
						class="font-bold flex items-center gap-2 {status === 'live'
							? 'text-green-500'
							: status === 'failed'
								? 'text-red-500'
								: 'text-yellow-500'}"
					>
						<Activity class="h-3.5 w-3.5" />
						{status}
					</div>
				</div>
				<div class="bg-card p-3 flex flex-col justify-center items-center">
					<div
						class="text-[10px] uppercase tracking-wider text-muted-foreground font-semibold mb-0.5"
					>
						Runtime
					</div>
					<div class="font-bold flex items-center gap-2">
						<Terminal class="h-3.5 w-3.5" />
						{project.runtime || "nodejs"}
					</div>
				</div>
				<div class="bg-card p-3 flex flex-col justify-center items-center">
					<div
						class="text-[10px] uppercase tracking-wider text-muted-foreground font-semibold mb-0.5"
					>
						Deployments
					</div>
					<div class="font-bold flex items-center gap-2">
						<RefreshCw class="h-3.5 w-3.5" />
						{project.deployments?.length || 0}
					</div>
				</div>
				<div class="bg-card p-3 flex flex-col justify-center items-center">
					<div
						class="text-[10px] uppercase tracking-wider text-muted-foreground font-semibold mb-0.5"
					>
						Config
					</div>
					<div class="font-bold flex items-center gap-2">
						<Settings class="h-3.5 w-3.5" />
						{project.env_vars?.length || 0} vars
					</div>
				</div>
			</div>
		</div>

		<div class="flex-1 grid grid-cols-1 lg:grid-cols-4 gap-4 min-h-0">
			<Card
				class="flex flex-col min-h-0 h-[300px] lg:h-auto bg-transparent shadow-none border-0 lg:col-span-1"
			>
				<div class="flex items-center justify-between px-1 pb-2">
					<h3 class="text-sm font-semibold text-muted-foreground">History</h3>
				</div>
				<div class="flex-1 overflow-y-auto pr-1 space-y-1">
					{#if project.deployments?.length}
						{#each project.deployments as deployment}
							<button
								class="w-full flex items-center justify-between p-2.5 rounded-md border text-left transition-all text-xs
								{activeDeploymentId === deployment.id
									? 'bg-primary/5 border-primary/20 shadow-sm'
									: 'bg-card hover:bg-muted/50 border-input'}"
								onclick={() => selectDeployment(deployment)}
							>
								<div class="flex flex-col gap-0.5 min-w-0">
									<div class="flex items-center gap-2">
										<span class="font-semibold text-xs">#{deployment.id}</span>
										<span
											class="font-mono text-[10px] text-muted-foreground bg-muted px-1 rounded flex items-center gap-1"
										>
											<GitCommit class="h-2 w-2" />
											{deployment.commit === "HEAD"
												? "HEAD"
												: deployment.commit === "MANUAL"
													? "Manual"
													: deployment.commit === "WEBHOOK"
														? "Webhook"
														: deployment.commit.substring(0, 7)}
										</span>
									</div>
									<span class="text-[10px] text-muted-foreground truncate">
										{new Date(deployment.created_at).toLocaleString(undefined, {
											month: "short",
											day: "numeric",
											hour: "2-digit",
											minute: "2-digit",
										})}
									</span>
								</div>
								<div class="flex items-center gap-2">
									{#if deployment.status === "building"}
										<Loader2 class="h-3 w-3 animate-spin text-yellow-500" />
									{:else}
										<div
											class="h-2 w-2 rounded-full {deployment.status === 'live'
												? 'bg-green-500'
												: deployment.status === 'failed'
													? 'bg-red-500'
													: 'bg-gray-300'}"
										></div>
									{/if}
								</div>
							</button>
						{/each}
					{:else}
						<div
							class="p-4 text-center text-xs text-muted-foreground border rounded-md border-dashed"
						>
							No deployments
						</div>
					{/if}
				</div>
			</Card>

			<div
				class="lg:col-span-3 flex flex-col min-h-0 h-[500px] lg:h-auto rounded-lg border bg-card shadow-sm overflow-hidden border-border/40"
			>
				<div
					class="flex shrink-0 items-center justify-between px-3 py-2 bg-muted/50 border-b border-border"
				>
					<div class="flex items-center gap-2">
						<Terminal class="h-3.5 w-3.5 text-muted-foreground" />

						<span class="text-xs font-mono text-muted-foreground">
							{#if activeDeploymentId}
								build-log-{activeDeploymentId}.log
							{:else}
								waiting-for-selection...
							{/if}
						</span>
						{#if ws}
							<span class="flex h-1.5 w-1.5 relative ml-2">
								<span
									class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"
								></span>
								<span
									class="relative inline-flex rounded-full h-1.5 w-1.5 bg-green-500"
								></span>
							</span>
						{/if}
					</div>
					<div class="flex items-center gap-1">
						{#if userScrolled}
							<Button
								variant="ghost"
								size="sm"
								class="h-5 text-[10px] px-2 text-yellow-500 hover:text-yellow-400 hover:bg-yellow-500/10 mr-2"
								onclick={() => {
									autoScroll = true;
									userScrolled = false;
									scrollToBottom(true);
								}}
							>
								Resume Scroll
							</Button>
						{/if}
						<Button
							variant="ghost"
							size="icon"
							class="h-6 w-6 text-muted-foreground hover:text-foreground"
							onclick={copyLogs}
							title="Copy Logs"
						>
							{#if copied}
								<Check class="h-3 w-3 text-green-500" />
							{:else}
								<Copy class="h-3 w-3" />
							{/if}
						</Button>
					</div>
				</div>

				<div
					bind:this={logContentRef}
					onscroll={handleScroll}
					class="flex-1 overflow-auto p-4 font-mono text-[11px] leading-relaxed text-foreground scrollbar-thin scrollbar-thumb-border scrollbar-track-transparent selection:bg-primary/20 bg-card"
				>
					{#if activeDeploymentLogs}
						<pre
							class="whitespace-pre-wrap break-all">{activeDeploymentLogs}</pre>
					{:else}
						<div
							class="flex h-full items-center justify-center text-muted-foreground italic text-xs"
						>
							<p>Select a deployment to view logs</p>
						</div>
					{/if}
					<div class="h-px w-full"></div>
				</div>
			</div>
		</div>
	</div>
{:else}
	<div class="flex flex-col items-center justify-center space-y-4 py-20">
		<p class="text-lg text-muted-foreground">Project not found.</p>
		<Button href="/">Go Back</Button>
	</div>
{/if}
