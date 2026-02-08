<script lang="ts">
	import { page } from "$app/stores";
	import { Button } from "$lib/components/ui/button";
	import { Toaster } from "svelte-sonner";
	import {
		ArrowLeft,
		LayoutDashboard,
		Settings,
		Activity,
		GitCommit,
		PanelLeft,
		PanelLeftClose
	} from "@lucide/svelte";

	let { children } = $props();
	let projectId = $derived($page.params.id);
	let isSidebarOpen = $state(true);

	function isActive(path: string) {
		return $page.url.pathname.endsWith(path)
			? "bg-secondary text-foreground"
			: "text-muted-foreground hover:bg-secondary/50 hover:text-foreground";
	}
</script>

<div class="h-full flex flex-col overflow-hidden">
	<header class="border-b bg-background/95 backdrop-blur shrink-0">
		<div class="max-w-[1600px] mx-auto w-full px-4 sm:px-6 flex h-14 items-center gap-4">
			<Button variant="ghost" size="icon" href="/" class="-ml-2 h-8 w-8 text-muted-foreground hover:text-foreground">
				<ArrowLeft class="h-4 w-4" />
			</Button>
			<div class="h-4 w-px bg-border/60 mx-1 hidden sm:block"></div>
			<div class="flex items-center gap-2 text-sm font-medium">
				<Button 
					variant="ghost" 
					size="icon" 
					class="h-8 w-8 text-muted-foreground hover:text-foreground" 
					onclick={() => isSidebarOpen = !isSidebarOpen}
					title={isSidebarOpen ? "Collapse Sidebar" : "Expand Sidebar"}
				>
					{#if isSidebarOpen}
						<PanelLeftClose class="h-4 w-4" />
					{:else}
						<PanelLeft class="h-4 w-4" />
					{/if}
				</Button>
				<span class="text-muted-foreground hidden sm:inline-block">/</span>
				<span class="hidden sm:inline-block">Project Dashboard</span>
			</div>
		</div>
	</header>

	<div class="flex-1 flex max-w-[1600px] mx-auto w-full px-4 sm:px-6 py-6 gap-6 min-h-0 overflow-hidden">
		{#if isSidebarOpen}
			<aside class="w-[220px] shrink-0 flex flex-col gap-1 transition-all duration-300 overflow-y-auto">
				<div class="text-xs font-semibold text-muted-foreground mb-2 px-3 uppercase tracking-wider">
					Menu
				</div>
				<Button
					href={`/projects/${projectId}`}
					variant="ghost"
					class={`w-full justify-start h-9 px-3 ${$page.url.pathname === `/projects/${projectId}` ? "bg-secondary text-foreground font-medium" : "text-muted-foreground hover:text-foreground"}`}
				>
					<LayoutDashboard class="mr-2 h-4 w-4" /> Overview
				</Button>
				<Button
					href={`/projects/${projectId}/deployments`}
					variant="ghost"
					class={`w-full justify-start h-9 px-3 ${isActive("/deployments")}`}
				>
					<GitCommit class="mr-2 h-4 w-4" /> Deployments
				</Button>
				<Button
					href={`/projects/${projectId}/settings`}
					variant="ghost"
					class={`w-full justify-start h-9 px-3 ${isActive("/settings")}`}
				>
					<Settings class="mr-2 h-4 w-4" /> Settings
				</Button>
			</aside>
		{/if}

	<main class="flex-1 min-w-0 flex flex-col h-full overflow-y-auto">
		{@render children()}
	</main>
	</div>
</div>
