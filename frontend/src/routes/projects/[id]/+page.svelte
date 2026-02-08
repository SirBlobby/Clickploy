<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { page } from "$app/stores";
	import { ProjectState } from "$lib/project-dashboard.svelte";
	import { Button } from "$lib/components/ui/button";
	import { Card } from "$lib/components/ui/card";
	import {
		Loader2,
		ExternalLink,
		RefreshCw,
		Activity,
		Terminal as TerminalIcon,
		Settings,
		Play,
		Check,
		Copy,
		GitCommit,
		Square,
	} from "@lucide/svelte";
	import { Terminal } from "@xterm/xterm";
	import { FitAddon } from "@xterm/addon-fit";
	import "@xterm/xterm/css/xterm.css";

	let projectState = new ProjectState($page.params.id as string);
	let term: Terminal | null = null;
	let fitAddon: FitAddon | null = null;
	let terminalContainer = $state<HTMLDivElement | undefined>(undefined);
	let copied = $state(false);

	onMount(() => {
		projectState.init();
		return () => {
			term?.dispose();
			projectState.destroy();
		};
	});

	// Initialize terminal when container is available
	$effect(() => {
		if (terminalContainer && !term) {
			term = new Terminal({
				theme: {
					background: "#0d1117",
					foreground: "#c9d1d9",
					cursor: "#c9d1d9",
					selectionBackground: "#58a6ff33",
				},
				fontFamily: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
				fontSize: 12,
				lineHeight: 1.4,
				convertEol: true, 
				disableStdin: true,
			});

			fitAddon = new FitAddon();
			term.loadAddon(fitAddon);
			term.open(terminalContainer);
			
			// Load initial logs
			if (projectState.activeDeploymentLogs) {
				term.write(projectState.activeDeploymentLogs.replace(/\n/g, "\r\n"));
			}

			// Hook up the state callbacks to xterm
			projectState.onLogData = (data) => {
				term?.write(data);
				// Heuristic: when build reports success, refresh to pull latest status.
				if (data.includes("Build successful")) {
					projectState.handleBuildCompleted();
				}
			};
			
			projectState.onLogClear = () => {
				term?.clear();
				term?.reset();
			};

			// Handle resizing
			const resizeObserver = new ResizeObserver(() => {
				if (fitAddon && term) {
					requestAnimationFrame(() => fitAddon?.fit());
				}
			});
			resizeObserver.observe(terminalContainer);
			
			// Initial fit
			requestAnimationFrame(() => fitAddon?.fit());

			return () => {
				resizeObserver.disconnect();
			};
		}
	});

	function handleCopyLogs() {
		if (projectState.copyLogs()) {
			copied = true;
			setTimeout(() => (copied = false), 2000);
		}
	}
</script>

{#if projectState.loading}
	<div class="flex justify-center items-center h-[50vh]">
		<Loader2 class="h-8 w-8 animate-spin text-muted-foreground" />
	</div>
{:else if projectState.project}
	<div class="h-full flex flex-col gap-4 overflow-hidden">
		<!-- Header Section -->
		<div class="flex items-start justify-between border-b pb-4 shrink-0">
			<div class="space-y-1">
				<h1 class="text-3xl font-bold tracking-tight">{projectState.project.name}</h1>
				<div class="flex items-center gap-4 text-sm text-muted-foreground">
					<a
						href={projectState.project.repo_url}
						target="_blank"
						class="hover:text-foreground hover:underline transition-colors flex items-center gap-1"
					>
						<GitCommit class="h-3.5 w-3.5" />
						{projectState.project.repo_url.replace("https://", "").replace("http://", "")}
					</a>
					<span class="text-muted-foreground/30">•</span>
					<div class="flex items-center gap-1.5">
						<div class={`h-2 w-2 rounded-full ${projectState.status === 'live' ? 'bg-green-500' : projectState.status === 'failed' ? 'bg-red-500' : projectState.status === 'building' ? 'bg-yellow-500' : 'bg-gray-400'}`}></div>
						<span class="capitalize">{projectState.status}</span>
					</div>
				</div>
			</div>
			<div class="flex items-center gap-2">
				<Button
					variant="outline"
					size="sm"
					onclick={() => projectState.refresh()}
					disabled={projectState.loading}
				>
					<RefreshCw class={`h-3.5 w-3.5 mr-2 ${projectState.loading ? 'animate-spin' : ''}`} /> Refresh
				</Button>
				<Button
					variant="outline"
					size="sm"
					onclick={() => projectState.handleRedeploy()}
					disabled={projectState.status === "building"}
				>
					{#if projectState.status === "building"}
						<Loader2 class="h-3.5 w-3.5 mr-2 animate-spin" /> Redeploying
					{:else}
						<Play class="h-3.5 w-3.5 mr-2" /> Redeploy
					{/if}
				</Button>
				{#if projectState.status === "live" || projectState.status === "building"}
					<Button
						variant="outline"
						size="sm"
						onclick={() => projectState.handleStop()}
						class="text-red-500 hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-950/20"
					>
						<Square class="h-3.5 w-3.5 mr-2 fill-current" /> Stop
					</Button>
				{/if}
				<Button
					href={projectState.latestDeployment?.url}
					target="_blank"
					disabled={projectState.status !== "live"}
					size="sm"
				>
					<ExternalLink class="mr-2 h-3.5 w-3.5" /> Visit App
				</Button>
			</div>
		</div>

		<!-- Stats Grid -->
		<div class="grid grid-cols-2 lg:grid-cols-4 gap-3 shrink-0">
			<Card class="p-3 flex items-center gap-3 shadow-sm">
				<div class={`p-2 rounded-full ${projectState.status === 'live' ? 'bg-green-100 dark:bg-green-900/30 text-green-600' : 'bg-muted text-muted-foreground'}`}>
					<Activity class="h-4 w-4" />
				</div>
				<div class="flex items-center gap-2">
					<span class="text-xs font-medium text-muted-foreground uppercase tracking-wider">Status:</span>
					<div class="font-bold text-sm">
						{#if projectState.status === 'live'}
							<span class="text-green-600 dark:text-green-400">Live</span>
						{:else if projectState.status === 'failed'}
							<span class="text-red-600 dark:text-red-400">Failed</span>
						{:else if projectState.status === 'building'}
							<span class="text-yellow-600 dark:text-yellow-400">Building</span>
						{:else}
							<span class="text-muted-foreground">Unknown</span>
						{/if}
					</div>
				</div>
			</Card>
			
			<Card class="p-3 flex items-center gap-3 shadow-sm">
				<div class="p-2 rounded-full bg-blue-100 dark:bg-blue-900/30 text-blue-600">
					<TerminalIcon class="h-4 w-4" />
				</div>
				<div class="flex items-center gap-2">
					<span class="text-xs font-medium text-muted-foreground uppercase tracking-wider">Runtime:</span>
					<div class="font-bold text-sm capitalize">{projectState.project.runtime || "nodejs"}</div>
				</div>
			</Card>

			<Card class="p-3 flex items-center gap-3 shadow-sm">
				<div class="p-2 rounded-full bg-purple-100 dark:bg-purple-900/30 text-purple-600">
					<GitCommit class="h-4 w-4" />
				</div>
				<div class="flex items-center gap-2">
					<span class="text-xs font-medium text-muted-foreground uppercase tracking-wider">Deployments:</span>
					<div class="font-bold text-sm">{projectState.project.deployments?.length || 0}</div>
				</div>
			</Card>

			<Card class="p-3 flex items-center gap-3 shadow-sm">
				<div class="p-2 rounded-full bg-orange-100 dark:bg-orange-900/30 text-orange-600">
					<Settings class="h-4 w-4" />
				</div>
				<div class="flex items-center gap-2">
					<span class="text-xs font-medium text-muted-foreground uppercase tracking-wider">Env Vars:</span>
					<div class="font-bold text-sm">{projectState.project.env_vars?.length || 0}</div>
				</div>
			</Card>
		</div>

		<!-- Main Content Area (History + Logs) -->
		<div class="flex-1 grid grid-cols-1 lg:grid-cols-12 gap-6 min-h-0">
			<!-- History Sidebar -->
			<div class="lg:col-span-3 flex flex-col h-full bg-card border rounded-lg overflow-hidden shadow-sm">
				<div class="px-4 py-3 border-b flex items-center justify-between bg-muted/30">
					<h3 class="font-semibold text-sm">Deployments</h3>
					<span class="text-xs text-muted-foreground font-mono bg-muted px-2 py-0.5 rounded">
						Total: {projectState.project.deployments?.length || 0}
					</span>
				</div>
				<div class="flex-1 overflow-y-auto p-2 space-y-2">
					{#if projectState.project.deployments?.length}
						{#each projectState.project.deployments as deployment}
							<button
								class="w-full text-left p-3 rounded-lg border transition-all hover:border-primary/50 hover:bg-muted/50 relative overflow-hidden group
								{projectState.activeDeploymentId === deployment.id ? 'bg-primary/5 border-primary shadow-sm ring-1 ring-primary/20' : 'bg-card border-border'}"
								onclick={() => projectState.selectDeployment(deployment)}
							>
								<!-- Status Indicator Strip -->
								<div class={`absolute left-0 top-0 bottom-0 w-1 ${
									deployment.status === 'live' ? 'bg-green-500' : 
									deployment.status === 'failed' ? 'bg-red-500' : 
									deployment.status === 'building' ? 'bg-yellow-500' : 'bg-gray-300'
								}`}></div>

								<div class="pl-2 space-y-1.5">
									<div class="flex items-center justify-between">
										<span class="font-mono text-xs font-semibold">#{deployment.id.slice(0, 8)}</span>
										<span class="text-[10px] text-muted-foreground">
											{new Date(deployment.created_at).toLocaleDateString()}
										</span>
									</div>
									<div class="flex items-center justify-between">
										<div class="flex items-center gap-1.5">
											<GitCommit class="h-3 w-3 text-muted-foreground" />
											<span class="text-xs text-muted-foreground font-mono">
												{deployment.commit === "HEAD" ? "HEAD" : 
												 deployment.commit === "MANUAL" ? "Manual" : 
												 deployment.commit === "WEBHOOK" ? "Webhook" : 
												 deployment.commit.substring(0, 7)}
											</span>
										</div>
										<span class={`text-[10px] px-1.5 py-0.5 rounded font-medium ${
											deployment.status === 'live' ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400' :
											deployment.status === 'failed' ? 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400' :
											deployment.status === 'building' ? 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400' : 
											'bg-gray-100 text-gray-600'
										}`}>
											{deployment.status}
										</span>
									</div>
								</div>
							</button>
						{/each}
					{:else}
						<div class="h-full flex flex-col items-center justify-center text-muted-foreground">
							<RefreshCw class="h-8 w-8 mb-2 opacity-20" />
							<p class="text-sm">No deployments found</p>
						</div>
					{/if}
				</div>
			</div>

			<!-- Logs Terminal -->
			<div class="lg:col-span-9 flex flex-col h-full border rounded-lg overflow-hidden shadow-sm bg-[#0d1117]">
				<!-- Terminal Header -->
				<div class="px-4 py-2 border-b border-white/10 flex items-center justify-between bg-white/5">
					<div class="flex items-center gap-2">
						<TerminalIcon class="h-4 w-4 text-muted-foreground" />
						<span class="text-xs font-mono text-gray-400">
							{#if projectState.activeDeploymentId}
								deployments/{projectState.activeDeploymentId}/build.log
							{:else}
								waiting_for_selection...
							{/if}
						</span>
					</div>
					<div class="flex items-center gap-2">
						{#if projectState.ws}
							<div class="flex items-center gap-1.5 text-xs text-green-400 animate-pulse font-mono mr-2">
								<div class="h-1.5 w-1.5 rounded-full bg-green-500"></div>
								Live Stream
							</div>
						{/if}
						
						<Button
							variant="ghost"
							size="icon"
							class="h-7 w-7 text-gray-400 hover:text-white hover:bg-white/10"
							onclick={handleCopyLogs}
							title="Copy to Clipboard"
						>
							{#if copied}
								<Check class="h-3.5 w-3.5 text-green-500" />
							{:else}
								<Copy class="h-3.5 w-3.5" />
							{/if}
						</Button>
					</div>
				</div>

				<!-- Terminal Content -->
				<div
					class="flex-1 overflow-hidden p-1 bg-[#0d1117]"
				>
					<div bind:this={terminalContainer} class="h-full w-full"></div>
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
