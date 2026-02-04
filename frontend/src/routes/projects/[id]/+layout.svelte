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
	} from "@lucide/svelte";

	let { children } = $props();
	let projectId = $derived($page.params.id);

	function isActive(path: string) {
		return $page.url.pathname.endsWith(path)
			? "bg-secondary"
			: "hover:bg-secondary/50";
	}
</script>

<div class="container mx-auto py-6 px-4">
	<div class="mb-6 flex items-center gap-4">
		<Button variant="ghost" size="icon" href="/">
			<ArrowLeft class="h-4 w-4" />
		</Button>
		<h1 class="text-2xl font-bold tracking-tight">Project Dashboard</h1>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-[250px_1fr] gap-8">
		<aside class="space-y-2">
			<Button
				href={`/projects/${projectId}`}
				variant="ghost"
				class={`w-full justify-start ${$page.url.pathname === `/projects/${projectId}` ? "bg-secondary" : ""}`}
			>
				<LayoutDashboard class="mr-2 h-4 w-4" /> Overview
			</Button>
			<Button
				href={`/projects/${projectId}/deployments`}
				variant="ghost"
				class={`w-full justify-start ${isActive("/deployments")}`}
			>
				<GitCommit class="mr-2 h-4 w-4" /> Deployments
			</Button>
			<Button
				href={`/projects/${projectId}/settings`}
				variant="ghost"
				class={`w-full justify-start ${isActive("/settings")}`}
			>
				<Settings class="mr-2 h-4 w-4" /> Settings
			</Button>
		</aside>

		<main class="min-h-[500px]">
			{@render children()}
		</main>
	</div>
</div>
