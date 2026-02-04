<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { Book } from "@lucide/svelte";
	import { page } from "$app/stores";

	let { children } = $props();

	function isActive(path: string) {
		return $page.url.pathname === path ? "bg-secondary" : "hover:bg-accent/50";
	}

	const navItems = [
		{ href: "/docs", label: "Introduction" },
		{ href: "/docs/features", label: "Features" },
		{ href: "/docs/architecture", label: "Architecture" },
		{ href: "/docs/api", label: "API Reference" },
	];
</script>

<div class="container mx-auto py-10 px-4">
	<div class="flex flex-col md:flex-row gap-10">
		<!-- Sidebar Navigation -->
		<aside class="hidden md:block w-64 shrink-0 space-y-8">
			<div class="sticky top-20">
				<div class="flex items-center gap-2 mb-6">
					<Book class="h-6 w-6 text-primary" />
					<h2 class="text-xl font-bold tracking-tight">Documentation</h2>
				</div>
				<nav class="space-y-1">
					{#each navItems as item}
						<Button
							variant="ghost"
							class="w-full justify-start {isActive(item.href)}"
							href={item.href}
						>
							{item.label}
						</Button>
					{/each}
				</nav>
			</div>
		</aside>

		<!-- Main Content -->
		<main class="flex-1 space-y-12 max-w-4xl min-h-[50vh]">
			{@render children()}
		</main>
	</div>
</div>
